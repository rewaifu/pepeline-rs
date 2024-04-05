use std::path::Path;

use numpy::{PyReadonlyArrayDyn, ToPyArray};
use pyo3::exceptions::{PyOSError, PyTypeError};
use pyo3::{pyfunction, PyErr, PyObject, PyResult, Python};

use crate::utils::core::convert::f32_to_u8;
use crate::utils::image::decode::{all_read_f32, all_read_u8};
use crate::utils::image::save::save_img_vec;

#[pyfunction]
pub fn save(input: PyObject, out_path: String, py: Python) -> PyResult<()> {
    //function to save an image, currently supports:
    //   f32 0-1 array
    //   u8 0-255 array

    let vec_img: Vec<u8>;
    let shape: Vec<usize>;

    //array extension definition
    if let Ok(array_py) = input.extract::<PyReadonlyArrayDyn<u8>>(py) {
        let array8 = array_py.as_array().to_owned();
        vec_img = array8.clone().into_raw_vec();
        shape = array8.shape().to_vec();
    } else if let Ok(array_py) = input.extract::<PyReadonlyArrayDyn<f32>>(py) {
        let arr32 = array_py.as_array().to_owned();
        vec_img = f32_to_u8(&arr32.clone().into_raw_vec());
        shape = arr32.shape().to_vec();
    } else {
        return Err(PyErr::new::<PyTypeError, _>("Unsupported array type"));
    }

    //saving the finished vector, on the passed path
    match save_img_vec(&vec_img, &shape, Path::new(&out_path)) {
        Ok(()) => Ok(()),
        Err(err) => Err(PyErr::new::<PyOSError, _>(format!(
            "Error saving image: {}",
            err
        ))),
    }
}

#[pyfunction]
pub fn read(path: String, mode: Option<u8>, format: Option<u8>, py: Python) -> PyResult<PyObject> {
    // The function to read the image.
    // Input parameters:
    //      path -> str file path
    //      mode -> uint 0 -> gray 1-> rgb 2-> psd dynamic format, and in other cases rgb, None = 2
    //      format -> uint 0 -> f32 0-1 img, 1+ -> u8 0-255, None = 1

    let path = Path::new(&path);
    let mode = mode.unwrap_or(2u8);
    let format = format.unwrap_or(1u8);

    match format {
        0 => match all_read_f32(path, mode) {
            Ok(array) => Ok(array.to_pyarray(py).into()),
            Err(err) => Err(PyErr::new::<PyOSError, _>(format!(
                "Error reading file: {}",
                err
            ))),
        },
        _ => match all_read_u8(path, mode) {
            Ok(array) => Ok(array.to_pyarray(py).into()),
            Err(err) => Err(PyErr::new::<PyOSError, _>(format!(
                "Error reading file: {}",
                err
            ))),
        },
    }
}
