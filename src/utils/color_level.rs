use ndarray::ArrayD;
use numpy::{PyArrayDyn, PyReadonlyArrayDyn, ToPyArray};
use pyo3::{Py, pyfunction, PyResult, Python};

fn levels(
    array: ArrayD<f32>,
    in_low: u8,
    in_high: u8,
    out_low: u8,
    out_high: u8,
    gamma: f32,
) ->ArrayD<f32>{
    let in_low = in_low as f32/255.0;
    let in_high = in_high as f32/255.0;
    let out_low = out_low as f32/255.0;
    let out_high = out_high as f32/255.0;
    let in_range = in_high - in_low;
    let out_range = out_high - out_low;
    array.mapv(|x| ((x - in_low) / (in_range) * (out_range) + out_low).max(0.0).min(1.0).powf(gamma))

}
#[pyfunction]
pub fn fast_color_level<'py>(
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