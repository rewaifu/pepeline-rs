use std::path::Path;

use numpy::{PyReadonlyArrayDyn, ToPyArray};
use pyo3::{PyErr, pyfunction, PyObject, PyResult, Python};
use pyo3::exceptions::{PyOSError, PyTypeError};

use crate::utils::image::decode::{all_read_f32, all_read_u8};
use crate::utils::image::save::save_img_vec;
use crate::utils::image::size_decode::path_to_size;

#[pyfunction]
pub fn save(input: PyObject, out_path: String, py: Python) -> PyResult<()> {
    //function to save an image, currently supports:
    //   f32 0-1 array
    //   f64 0-1 array
    //   u8 0-255 array

    let vec_img: Vec<u8>;
    let shape: Vec<usize>;

    //array extension definition
    if let Ok(array_py) = input.extract::<PyReadonlyArrayDyn<u8>>(py) {
        let array8 = array_py.as_array().to_owned();
        vec_img = array8.iter().map(|&x| x).collect();
        shape = array8.shape().to_vec();
    } else if let Ok(array_py) = input.extract::<PyReadonlyArrayDyn<f32>>(py) {
        let arr32 = array_py.as_array().to_owned();
        vec_img = arr32.iter().map(|&x| (x * 255.0) as u8).collect();
        shape = arr32.shape().to_vec();
    } else if let Ok(array_py) = input.extract::<PyReadonlyArrayDyn<f64>>(py) {
        let arr64 = array_py.as_array().to_owned();
        vec_img = arr64.iter().map(|&x| (x * 255.0) as u8).collect();
        shape = arr64.shape().to_vec();
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
            Ok(array) => Ok(array.to_pyarray_bound(py).into()),
            Err(err) => Err(PyErr::new::<PyOSError, _>(format!(
                "Error reading file: {}",
                err
            ))),
        },
        _ => match all_read_u8(path, mode) {
            Ok(array) => Ok(array.to_pyarray_bound(py).into()),
            Err(err) => Err(PyErr::new::<PyOSError, _>(format!(
                "Error reading file: {}",
                err
            ))),
        },
    }
}
/// Reads the dimensions (width and height) of the image at the given path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the image file.
///
/// # Returns
///
/// A tuple containing the width and height of the image, or an error if the image could not be read.
///
/// # Examples
///
/// ```
/// let dimensions = read_size(String::from("path/to/image.png"))?;
/// println!("Width: {}, Height: {}", dimensions.0, dimensions.1);
/// ```
///
/// # Errors
///
/// This function will return an error if the file does not exist, the file is not an image,
/// or if there is an issue reading the image dimensions.
#[pyfunction]
pub fn read_size(path: String) -> PyResult<(u32, u32)> {
    path_to_size(Path::new(&path))
}
