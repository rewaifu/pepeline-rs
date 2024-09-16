use ndarray::ArrayD;
use pyo3::pyclass;

#[pyclass]
#[derive(Clone, Copy)]
pub enum ImgColor {
    GRAY,
    AGRAY,
    RGB,
    RGBA,
    DYNAMIC
}
#[pyclass]
#[derive(Clone, Copy)]
pub enum ImgFormat {
    U8,
    F32,
    U16,
    DYNAMIC
}
#[derive(Clone, Debug)]
pub enum ArrayDType {
    F32(ArrayD<f32>),
    U8(ArrayD<u8>),
    U16(ArrayD<u16>),
}