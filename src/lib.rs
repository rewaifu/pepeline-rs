mod utils;

use std::path::Path;
use utils::image::{rgb_img_open, gray_img_open,save};
use utils::screenton::screenton_add;
use utils::color_level::levels;
use numpy::{PyArray2, ToPyArray, PyArray3, PyReadonlyArray2, PyReadonlyArrayDyn, PyArrayDyn};
use pyo3::prelude::*;

#[pyfunction]
fn read_gray<'py>(path:String,py: Python)  -> PyResult<Py<PyArray2<u8>>> {
    let array = gray_img_open(Path::new(&path));
    Ok(array.to_pyarray(py).to_owned()

    )
}
#[pyfunction]
fn read<'py>(path: String, py: Python) -> PyResult<Py<PyArray3<u8>>> {
    let array = rgb_img_open(Path::new(&path));
    Ok(array.to_pyarray(py).to_owned())
}

#[pyfunction]
fn fast_color_level<'py>(
          input: PyReadonlyArrayDyn<f32>,
          in_low: u8,
          in_high: u8,
          out_low: u8,
          out_high: u8,
          gamma: f32,
          py: Python,
) -> PyResult<Py<PyArrayDyn<f32>>> {
    let array = input.as_array().to_owned();
    let array = levels(array,in_low,in_high,out_low,out_high,gamma);
    Ok(array.to_pyarray(py).to_owned())
}

#[pyfunction]
fn screenton<'py>(input: PyReadonlyArray2<f32>,dot_size: usize, lx_plus: Option<usize>, ly_plus: Option<usize>, py: Python) -> PyResult<Py<PyArray2<f32>>> {
    let lx_plus = match lx_plus {
        Some(val) => val,
        None => dot_size / 2,
    };
    let ly_plus = match ly_plus {
        Some(val) => val,
        None => dot_size / 2,
    };
    let mut array = input.as_array().to_owned();
    screenton_add(&mut array,dot_size,ly_plus,lx_plus);
    Ok(array.to_pyarray(py).to_owned())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pepeline(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_gray, m)?)?;
    m.add_function(wrap_pyfunction!(read, m)?)?;
    m.add_function(wrap_pyfunction!(screenton, m)?)?;
    m.add_function(wrap_pyfunction!(fast_color_level, m)?)?;
    m.add_function(wrap_pyfunction!(save, m)?)?;
    Ok(())
}
