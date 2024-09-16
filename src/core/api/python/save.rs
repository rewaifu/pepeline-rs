use std::path::Path;
use pyo3::{pyfunction, PyErr, PyObject, PyResult, Python};
use pyo3::exceptions::PyValueError;
use crate::core::api::python::convert::pyobject2arraydtype;
use crate::core::save::save::save_img_ndarray;

#[pyfunction]
pub fn save(input: PyObject, out_path: String, py: Python) -> PyResult<()> {
    let array = pyobject2arraydtype(input,py)?;
    match save_img_ndarray(array,Path::new(&out_path)) {
        Ok(()) => Ok(()),
        Err(err) => Err(PyErr::new::<PyValueError, _>(format!(
            "Error saving image: {}",
            err
        ))),
    }
    
}