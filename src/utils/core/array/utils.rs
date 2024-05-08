use ndarray::ArrayD;
use numpy::PyReadonlyArrayDyn;
use pyo3::exceptions::PyTypeError;
use pyo3::{PyErr, PyObject, Python};

//preparation for the future
pub fn py_obj_to_array<T>(py_array: PyObject, py: Python) -> Result<ArrayD<T>, PyErr>
where
    T: numpy::Element,
{
    let array_data: Result<ArrayD<T>, _> =
        if let Ok(array_py) = py_array.extract::<PyReadonlyArrayDyn<T>>(py) {
            Ok(array_py.as_array().to_owned())
        } else {
            Err(PyErr::new::<PyTypeError, _>("Unsupported array type"))
        };

    array_data
}
