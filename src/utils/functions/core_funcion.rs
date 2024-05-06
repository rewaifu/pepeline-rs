use std::cmp::{max, min};

use ndarray::{Array2, Array3};
use noise::{NoiseFn, OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex};
use numpy::{PyArrayDyn, PyReadonlyArrayDyn, ToPyArray};
use pyo3::{pyfunction, Py, PyResult, Python};
use rand::Rng;

use crate::utils::core::enums::TypeNoise;
use crate::utils::core::noise::{noise_2d, noise_3d};

fn generate_noise2d(type_noise: TypeNoise, seed: u32) -> Box<dyn NoiseFn<f64, 2>> {
    match type_noise {
        TypeNoise::PERLIN => Box::new(Perlin::new(seed)),
        TypeNoise::SIMPLEX => Box::new(Simplex::new(seed)),
        TypeNoise::OPENSIMPLEX => Box::new(OpenSimplex::new(seed)),
        TypeNoise::SUPERSIMPLEX => Box::new(SuperSimplex::new(seed)),
        TypeNoise::PERLINSURFLET => Box::new(PerlinSurflet::new(seed)),
    }
}

fn generate_noise3d(type_noise: TypeNoise, seed: u32) -> Box<dyn NoiseFn<f64, 3>> {
    match type_noise {
        TypeNoise::PERLIN => Box::new(Perlin::new(seed)),
        TypeNoise::SIMPLEX => Box::new(Simplex::new(seed)),
        TypeNoise::OPENSIMPLEX => Box::new(OpenSimplex::new(seed)),
        TypeNoise::SUPERSIMPLEX => Box::new(SuperSimplex::new(seed)),
        TypeNoise::PERLINSURFLET => Box::new(PerlinSurflet::new(seed)),
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
    let seed = seed.unwrap_or(rand::thread_rng().gen_range(1..=10000) as u32);
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

fn crop_cord_2d(img: &ndarray::ArrayD<f32>) -> (usize, usize, usize, usize) {
    let shap = img.shape();
    let mut x_min = shap[0];
    let mut x_max = 0;
    let mut y_min = shap[1];
    let mut y_max = 0;
    for x in 0..shap[0] {
        for y in 0..shap[1] {
            if img[[x, y]] != 0.0 {
                x_min = min(x_min, x);
                x_max = max(x_max, x);
                y_min = min(y_min, y);
                y_max = max(y_max, y);
            }
        }
    }
    return (x_min, x_max, y_min, y_max);
}

fn crop_cord_3d(img: &ndarray::ArrayD<f32>) -> (usize, usize, usize, usize) {
    let shap = img.shape();
    let mut x_min = shap[0];
    let mut x_max = 0;
    let mut y_min = shap[1];
    let mut y_max = 0;
    for x in 0..shap[0] {
        for y in 0..shap[1] {
            for c in 0..shap[2] {
                if img[[x, y, c]] != 0.0 {
                    x_min = min(x_min, x);
                    x_max = max(x_max, x);
                    y_min = min(y_min, y);
                    y_max = max(y_max, y);
                }
            }
        }
    }
    return (x_min, x_max, y_min, y_max);
}

#[pyfunction]
pub fn crop_cord(input: PyReadonlyArrayDyn<f32>) -> PyResult<(usize, usize, usize, usize)> {
    let array = input.as_array().to_owned();
    match array.shape().len() {
        2 => Ok(crop_cord_2d(&array)),
        3 => Ok(crop_cord_3d(&array)),
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "Unsupported dimensions",
        )),
    }
}
