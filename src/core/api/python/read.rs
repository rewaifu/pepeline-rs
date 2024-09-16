use std::path::Path;
use ndarray::ArrayD;
use numpy::{ToPyArray};
use pyo3;
use pyo3::{pyfunction, PyObject, PyResult, Python};
use crate::core::universal_functions::enums::{ArrayDType, ImgColor, ImgFormat};
use crate::core::read::decode::decoder;
use crate::core::universal_functions::format_convert::{f32_to_u16, f32_to_u8, u16_to_f32, u16_to_u8, u8_to_f32, u8_to_u16};

#[pyfunction]
pub fn read(path: String, color: Option<ImgColor>, format: Option<ImgFormat>, py: Python) -> PyResult<PyObject> {
    let color = color.unwrap_or(ImgColor::DYNAMIC);
    let format = format.unwrap_or(ImgFormat::U8);
    let array = decoder(&Path::new(&path), color,format);
    let array = array.unwrap();
    match array {
        ArrayDType::F32(value) => Ok(value.to_pyarray_bound(py).into()),
        ArrayDType::U8(value) => Ok(value.to_pyarray_bound(py).into()),
        ArrayDType::U16(value) => Ok(value.to_pyarray_bound(py).into())
    }
}