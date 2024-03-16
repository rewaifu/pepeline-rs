use ndarray::ArrayD;
use numpy::{PyArrayDyn, PyReadonlyArrayDyn, ToPyArray};
use pyo3::{Py, pyfunction, PyResult, Python};

pub fn levels(
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
#[pyo3(signature = (input), text_signature = "array")]
pub fn normolize(
    input: PyReadonlyArrayDyn<f32>,
    py: Python
)-> PyResult<Py<PyArrayDyn<f32>>>{
    let array = input.as_array().to_owned();
    let (min_value, max_value) = array.iter().fold((1.0 as f32, 0.0 as f32), |(min, max), &val| {
        (min.min(val), max.max(val))
    });

    if min_value ==0.0 && max_value ==1.0 {Ok(array.to_pyarray(py).to_owned())}else {
        let min_minus_max:f32 = max_value-min_value;
        array.mapv(|x| (x-min_value)/min_minus_max);
        Ok(array.to_pyarray(py).to_owned())
    }




}