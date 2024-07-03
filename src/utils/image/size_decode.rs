use std::path::Path;
use filebuffer::FileBuffer;
use pyo3::prelude::*;
use pyo3::exceptions::{PyOSError, PyValueError};

/// Reads a 32-bit value from a byte array at the specified offset.
///
/// # Arguments
///
/// * `bytes` - Byte array.
/// * `offset` - Offset in the byte array.
///
/// # Returns
///
/// 32-bit value.
fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]])
}

/// Reads a 16-bit value from a byte array at the specified offset.
///
/// # Arguments
///
/// * `bytes` - Byte array.
/// * `offset` - Offset in the byte array.
///
/// # Returns
///
/// 16-bit value.
fn read_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([bytes[offset], bytes[offset + 1]])
}

/// Extracts the width and height from PNG bytes.
///
/// # Arguments
///
/// * `png_bytes` - Byte array representing the PNG file.
///
/// # Returns
///
/// A tuple `(width, height)` if the IHDR chunk is found, otherwise an error.
fn png_size(png_bytes: &[u8]) -> PyResult<(u32, u32)> {
    const IHDR_CHUNK: &[u8; 4] = b"IHDR";
    let mut index = 8;
    let png_len = png_bytes.len();

    while index < png_len {
        let length = read_u32(png_bytes, index) as usize;
        let chunk_type = &png_bytes[index + 4..index + 8];

        if chunk_type == IHDR_CHUNK {
            let width = read_u32(png_bytes, index + 8);
            let height = read_u32(png_bytes, index + 12);
            return Ok((width, height));
        }
        index += 8 + length + 4;
    }

    Err(PyValueError::new_err("PNG - IHDR segment not found"))
}

/// Extracts the width and height from JPEG bytes.
///
/// # Arguments
///
/// * `jpeg_bytes` - Byte array representing the JPEG file.
///
/// # Returns
///
/// A tuple `(width, height)` if the SOF0 segment is found, otherwise an error.
fn jpeg_size(jpeg_bytes: &[u8]) -> PyResult<(u32, u32)> {
    let mut index = 2;
    let jpeg_len = jpeg_bytes.len();

    while index < jpeg_len {
        if jpeg_bytes[index] != 0xFF {
            return Err(PyValueError::new_err("JPEG - could not find marker"));
        }

        let marker = &jpeg_bytes[index..index + 2];
        if marker == [0xFF, 0xC0] { // SOF0 (Start of Frame)
            let height = read_u16(jpeg_bytes, index + 5);
            let width = read_u16(jpeg_bytes, index + 7);
            return Ok((width as u32, height as u32));
        }
        let length = read_u16(jpeg_bytes, index + 2);
        index += 2 + length as usize;
    }

    Err(PyValueError::new_err("JPEG - SOF0 segment not found"))
}

/// Extracts the width and height from WEBP bytes.
///
/// # Arguments
///
/// * `webp_bytes` - Byte array representing the WEBP file.
///
/// # Returns
///
/// A tuple `(width, height)` if the VP8 or VP8L segment is found, otherwise an error.
fn webp_size(webp_bytes: &[u8]) -> PyResult<(u32, u32)> {
    const VP8_HEADER: &[u8] = b"VP8 ";
    const VP8L_HEADER: &[u8] = b"VP8L";
    let mut index = 12;
    let webp_len = webp_bytes.len();

    while index < webp_len {
        let chunk_type = &webp_bytes[index..index + 4];
        let chunk_size = read_u32(webp_bytes, index + 4) as usize;

        if chunk_type == VP8_HEADER && chunk_size >= 10 {
            let width = read_u16(webp_bytes, index + 26) & 0x3FFF;
            let height = read_u16(webp_bytes, index + 28) & 0x3FFF;
            return Ok((width as u32, height as u32));
        }

        if chunk_type == VP8L_HEADER && chunk_size >= 5 {
            let b0 = webp_bytes[index + 9] as u32;
            let b1 = webp_bytes[index + 10] as u32;
            let b2 = webp_bytes[index + 11] as u32;
            let b3 = webp_bytes[index + 12] as u32;
            let width = 1 + (((b1 & 0x3F) << 8) | b0);
            let height = 1 + (((b3 & 0xF) << 10) | (b2 << 2) | ((b1 & 0xC0) >> 6));
            return Ok((width, height));
        }

        index += 8 + chunk_size + (chunk_size % 2);
    }

    Err(PyValueError::new_err("WEBP - Segment VP8 or VP8L not found"))
}

/// Determines the size of an image given its file path.
///
/// # Arguments
///
/// * `img_path` - Path to the image file.
///
/// # Returns
///
/// A tuple `(width, height)` representing the dimensions of the image.
///
/// # Errors
///
/// Returns an error if the file cannot be read or if the image format is unsupported.
pub fn path_to_size(img_path: &Path) -> PyResult<(u32, u32)> {
    let file_bytes = match FileBuffer::open(img_path) {
        Ok(buffer) => buffer.to_vec(),
        Err(err) => return Err(PyOSError::new_err(format!("Error reading file: {}", err))),
    };
    let signature = &file_bytes[0..4];
    match signature {
        [137, 80, 78, 71] => png_size(&file_bytes),
        [255, 216, 255, 224] => jpeg_size(&file_bytes),
        [82, 73, 70, 70] => webp_size(&file_bytes),
        _ => Err(PyValueError::new_err("Unsupported image format"))
    }
}
