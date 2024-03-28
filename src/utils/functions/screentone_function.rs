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
    // screentone overlay function:
    //     input -> array only 2D f32 0-1
    //     dot_size -> uint screenton size in pixels
    //     lx_plus and ly_plus -> uint offset of the pattern by the number of pixels specified by these parameters. None=dot_size/2
    let lx_plus = lx_plus.unwrap_or(dot_size / 2);
    let ly_plus = ly_plus.unwrap_or(dot_size / 2);
    let mut array = input.as_array().to_owned();
    screentone_add(&mut array, dot_size, ly_plus, lx_plus);
    Ok(array.to_pyarray(py).to_owned())
}
