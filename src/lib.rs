use ndarray::{Array2, Array3};
use numpy::{PyArray2, ToPyArray,PyArray3};
use pyo3::prelude::*;
/// Formats the sum of two numbers as string.
#[pyfunction]
fn read_gray(path:String,py: Python)  -> PyResult<Py<PyArray2<u8>>> {
    let img = image::open(path).unwrap().into_luma8();
    let (width, height) = img.dimensions();
    let input = img.into_raw();
    let array = Array2::from_shape_vec((height as usize, width as usize), input).unwrap();
    Ok(array.to_pyarray(py).to_owned()

    )
}

#[pyfunction]
fn read(path: String, py: Python) -> PyResult<Py<PyArray3<u8>>> {
    let img = image::open(path).unwrap().into_rgb8();
    let (width, height) = img.dimensions();
    let input= img.into_raw();
    let array = Array3::from_shape_vec((height as usize, width as usize, 3), input).unwrap();

    Ok(array.to_pyarray(py).to_owned())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pepeline(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_gray, m)?)?;
    m.add_function(wrap_pyfunction!(read, m)?)?;
    Ok(())
}
