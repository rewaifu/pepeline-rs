use filebuffer::FileBuffer;
use pyo3::exceptions::{PyOSError, PyValueError};
use pyo3::prelude::*;
use std::io::Cursor;
use std::path::Path;

fn read_u32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn read_u16(bytes: &[u8]) -> u16 {
    u16::from_be_bytes([bytes[0], bytes[1]])
}

fn read_le_u16(bytes: &[u8]) -> u16 {
    u16::from_le_bytes([bytes[0], bytes[1]])
}

fn read_le_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn find_sequence(cursor: &mut Cursor<&[u8]>, sequence: &[u8]) -> Option<usize> {
    let buffer = cursor.get_ref();
    let seq_len = sequence.len();
    buffer
        .windows(seq_len)
        .position(|window| window == sequence)
}

fn png_size(cursor: &mut Cursor<&[u8]>) -> PyResult<(u32, u32)> {
    const IHDR_CHUNK: &[u8; 4] = b"IHDR";
    let index = match find_sequence(cursor, IHDR_CHUNK) {
        Some(index) => index,
        None => return Err(PyValueError::new_err("PNG - Segment IHDR not found")),
    };
    let width = read_u32(&cursor.get_ref()[index + 4..index + 8]);
    let height = read_u32(&cursor.get_ref()[index + 8..index + 12]);
    Ok((width, height))
}

fn jpeg_size(cursor: &mut Cursor<&[u8]>) -> PyResult<(u32, u32)> {
    const SOF0: &[u8; 2] = &[0xFF, 0xC0];
    const SOF2: &[u8; 2] = &[0xFF, 0xC2];

    let index = match find_sequence(cursor, SOF0) {
        Some(position) => position,
        None => match find_sequence(cursor, SOF2) {
            Some(position) => position,
            None => return Err(PyValueError::new_err("Unsupported JPEG format")),
        },
    };

    let width = read_u16(&cursor.get_ref()[index + 5..index + 7]) as u32;
    let height = read_u16(&cursor.get_ref()[index + 7..index + 9]) as u32;
    Ok((width, height))
}

fn webp_size(cursor: &mut Cursor<&[u8]>) -> PyResult<(u32, u32)> {
    const VP8: &[u8; 3] = b"VP8";

    let index = match find_sequence(cursor, VP8) {
        Some(index) => index,
        None => return Err(PyValueError::new_err("WEBP - Segment VP8 not found")),
    };
    let prefix = &cursor.get_ref()[index + 3];
    return if prefix == &76 {
        let header = read_le_u32(&cursor.get_ref()[index + 9..index + 13]);
        let width = (1 + header) & 0x3FFF;
        let height = (1 + (header >> 14)) & 0x3FFF;
        Ok((width, height))
    } else if prefix == &120 {
        Err(PyValueError::new_err("WEBP - Unsupported VP8X format"))
    } else {
        let width = (read_le_u16(&cursor.get_ref()[index + 14..index + 16]) & 0x3FFF) as u32;
        let height = (read_le_u16(&cursor.get_ref()[index + 16..index + 18]) & 0x3FFF) as u32;
        Ok((width, height))
    };
}

pub fn path_to_size(img_path: &Path) -> PyResult<(u32, u32)> {
    let file_bytes = match FileBuffer::open(img_path) {
        Ok(buffer) => buffer.to_vec(),
        Err(err) => return Err(PyOSError::new_err(format!("Error reading file: {}", err))),
    };
    let signature = &file_bytes[0..4];
    let mut cursor: Cursor<&[u8]> = Cursor::new(&file_bytes);
    match signature {
        [137, 80, 78, 71] => png_size(&mut cursor),
        [255, 216, 255, 224] => jpeg_size(&mut cursor),
        [82, 73, 70, 70] => webp_size(&mut cursor),
        _ => Err(PyValueError::new_err("Unsupported image format")),
    }
}
