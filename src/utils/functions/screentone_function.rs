use numpy::{PyArray2, PyReadonlyArray2, ToPyArray};
use pyo3::{pyfunction, Py, PyResult, Python};

use crate::utils::screentone::screentone_add::screentone_add;

#[pyfunction]
pub fn screentone<'py>(
    input: PyReadonlyArray2<f32>,
    dot_size: usize,
    lx_plus: Option<usize>,
    ly_plus: Option<usize>,
    py: Python,
) -> PyResult<Py<PyArray2<f32>>> {
    let lx_plus = match lx_plus {
        Some(val) => val,
        None => dot_size / 2,
    };
    let ly_plus = match ly_plus {
        Some(val) => val,
        None => dot_size / 2,
    };
    let mut array = input.as_array().to_owned();
    screentone_add(&mut array, dot_size, ly_plus, lx_plus);
    Ok(array.to_pyarray(py).to_owned())
}
