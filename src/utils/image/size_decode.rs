use filebuffer::FileBuffer;
use pyo3::exceptions::{PyOSError};
use pyo3::prelude::*;
use std::io::Cursor;
use std::path::Path;
use image::io::Reader as ImageReader;

pub fn path_to_size(img_path: &Path) -> PyResult<(u32, u32)> {
    let file_bytes = match FileBuffer::open(img_path) {
        Ok(buffer) => buffer.to_vec(),
        Err(err) => return Err(PyOSError::new_err(format!("Error reading file: {}", err))),
    };
    let mut cursor: Cursor<&[u8]> = Cursor::new(&file_bytes);
    match ImageReader::new(&mut cursor).with_guessed_format()?.into_dimensions(){ 
        Ok(dim)=>Ok(dim),
        Err(err)=>Err(PyOSError::new_err(format!("Error decode file: {}", err))),
    }
}
