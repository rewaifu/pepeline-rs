use numpy::{PyArrayDyn, PyReadonlyArrayDyn, ToPyArray};
use pyo3::{pyfunction, Py, PyResult, Python};

use crate::utils::core::color_levels::levels;

#[pyfunction]
pub fn fast_color_level<'py>(
    input: PyReadonlyArrayDyn<f32>,
    in_low: Option<u8>,
    in_high: Option<u8>,
    out_low: Option<u8>,
    out_high: Option<u8>,
    gamma: Option<f32>,
    py: Python,
) -> PyResult<Py<PyArrayDyn<f32>>> {
    let in_low = in_low.unwrap_or(0u8);
    let in_high = in_high.unwrap_or(255u8);
    let out_low = out_low.unwrap_or(0u8);
    let out_high = out_high.unwrap_or(255u8);
    let gamma = gamma.unwrap_or(1.0f32);
    let array = input.as_array().to_owned();
    let array = levels(array, in_low, in_high, out_low, out_high, gamma);

    Ok(array.to_pyarray(py).to_owned())
}
