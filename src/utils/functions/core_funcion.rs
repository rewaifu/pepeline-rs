use ndarray::{Array2, Array3};
use noise::Perlin;
use numpy::{PyArrayDyn, PyReadonlyArrayDyn, ToPyArray};
use pyo3::{Py, pyfunction, PyResult, Python};
use rand::Rng;

use crate::utils::core::color_levels::levels;
use crate::utils::core::noise::{perlin_noise_2d, perlin_noise_3d};

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

#[pyfunction]
pub fn perlin_noise<'py>(
    size: Vec<usize>,
    octaves: u8,
    frequency: f64,
    lacunarity: f64,
    seed: Option<u32>,
    py: Python,
) -> PyResult<Py<PyArrayDyn<f32>>> {
    let seed = match seed {
        Some(s) => s,
        None => rand::thread_rng().gen_range(1..=10000) as u32,
    };
    match size.len() {
        2 => {
            let mut array: Array2<f32> = Array2::zeros((size[0], size[1]));
            let p = Perlin::new(seed);
            for ((x, y), value) in array.indexed_iter_mut() {
                *value = perlin_noise_2d(&p, x, y, octaves, frequency, lacunarity);
            }
            Ok(array.into_dyn().to_pyarray(py).to_owned())
        }
        3 => {
            let mut array: Array3<f32> = Array3::zeros((size[0], size[1], size[2]));
            let p = Perlin::new(seed);
            for ((x, y, z), value) in array.indexed_iter_mut() {
                *value = perlin_noise_3d(&p, x, y, z, octaves, frequency, lacunarity);
            }
            Ok(array.into_dyn().to_pyarray(py).to_owned())
        }
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "Unsupported dimensions",
        )),
    }
}
