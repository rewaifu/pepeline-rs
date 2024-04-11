use ndarray::{Array2, Array3};
use noise::{NoiseFn, OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex};
use numpy::{PyArrayDyn, PyReadonlyArrayDyn, ToPyArray};
use pyo3::{Py, pyclass, pyfunction, PyResult, Python};
use rand::Rng;

use crate::utils::core::color_levels::levels;
use crate::utils::core::noise::{noise_2d, noise_3d};

#[pyclass]
#[derive(Clone)]
pub enum TypeNoise {
    PERLIN = 0,
    SIMPLEX = 1,
    OPENSIMPLEX = 2,
    SUPERSIMPLEX = 3,
    PERLINSURFLET = 4,
}

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

fn generate_noise2d(
    type_noise: TypeNoise,
    seed: u32,
) -> Box<dyn NoiseFn<f64, 2>> {
    match type_noise {
        TypeNoise::PERLIN => Box::new(Perlin::new(seed)),
        TypeNoise::SIMPLEX => Box::new(Simplex::new(seed)),
        TypeNoise::OPENSIMPLEX => Box::new(OpenSimplex::new(seed)),
        TypeNoise::SUPERSIMPLEX => Box::new(SuperSimplex::new(seed)),
        TypeNoise::PERLINSURFLET => Box::new(PerlinSurflet::new(seed))
    }
}

fn generate_noise3d(
    type_noise: TypeNoise,
    seed: u32,
) -> Box<dyn NoiseFn<f64, 3>> {
    match type_noise {
        TypeNoise::PERLIN => Box::new(Perlin::new(seed)),
        TypeNoise::SIMPLEX => Box::new(Simplex::new(seed)),
        TypeNoise::OPENSIMPLEX => Box::new(OpenSimplex::new(seed)),
        TypeNoise::SUPERSIMPLEX => Box::new(SuperSimplex::new(seed)),
        TypeNoise::PERLINSURFLET => Box::new(PerlinSurflet::new(seed))
    }
}

#[pyfunction]
pub fn noise_generate<'py>(
    size: Vec<usize>,
    type_noise: TypeNoise,
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
            let type_fn = generate_noise2d(type_noise, seed);
            for ((x, y), value) in array.indexed_iter_mut() {
                *value = noise_2d(&type_fn, x, y, octaves, frequency, lacunarity);
            }
            Ok(array.into_dyn().to_pyarray(py).to_owned())
        }
        3 => {
            let mut array: Array3<f32> = Array3::zeros((size[0], size[1], size[2]));
            let type_fn = generate_noise3d(type_noise, seed);
            for ((x, y, z), value) in array.indexed_iter_mut() {
                *value = noise_3d(&type_fn, x, y, z, octaves, frequency, lacunarity);
            }
            Ok(array.into_dyn().to_pyarray(py).to_owned())
        }
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "Unsupported dimensions",
        )),
    }
}


