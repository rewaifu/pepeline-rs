use numpy::PyReadonlyArrayDyn;
use pyo3::{PyErr, PyObject, Python};
use pyo3::exceptions::PyValueError;
use crate::core::universal_functions::enums::ArrayDType;

pub (crate) fn pyobject2arraydtype(input: PyObject, py: Python) -> Result<ArrayDType, PyErr>
    {
        if let Ok(array_py) = input.extract::<PyReadonlyArrayDyn<u8>>(py) {
            let array = array_py.as_array().to_owned();
            Ok(ArrayDType::U8(array))
        }
        else if let Ok(array_py) = input.extract::<PyReadonlyArrayDyn<u16>>(py) {
            let array = array_py.as_array().to_owned();
            Ok(ArrayDType::U16(array))
        }
        else if let Ok(array_py) = input.extract::<PyReadonlyArrayDyn<f32>>(py) {
            let array = array_py.as_array().to_owned();
            Ok(ArrayDType::F32(array))
        }
        else {
            Err(PyErr::new::<PyValueError, _>("Unsupported array format"))
        }
    }