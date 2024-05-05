use std::cmp::{max, min};

use ndarray::{Array2, Array3};
use noise::{NoiseFn, OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex};
use numpy::{PyArray3, PyArrayDyn, PyReadonlyArray3, PyReadonlyArrayDyn, ToPyArray};
use pyo3::{pyclass, pyfunction, Py, PyResult, Python};
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

fn rgb_to_cmyk(r: f32, g: f32, b: f32) -> (f32, f32, f32, f32) {
    if (r, g, b) == (0.0, 0.0, 0.0) {
        return (0.0, 0.0, 0.0, 1.0);
    }
    let c = 1.0 - r;
    let m = 1.0 - g;
    let y = 1.0 - b;

    // extract out k [0, 1]
    let min_cmy = c.min(m).min(y);
    let c = (c - min_cmy) / (1.0 - min_cmy);
    let m = (m - min_cmy) / (1.0 - min_cmy);
    let y = (y - min_cmy) / (1.0 - min_cmy);
    let k = min_cmy;

    (c, m, y, k)
}

fn cmyk_to_rgb(c: f32, m: f32, y: f32, k: f32) -> (f32, f32, f32) {
    let k = 1.0 - k;
    let r = (1.0 - c) * k;
    let g = (1.0 - m) * k;
    let b = (1.0 - y) * k;
    (r, g, b)
}

#[pyfunction]
pub fn rgb2cmyk(input: PyReadonlyArray3<f32>, py: Python) -> PyResult<Py<PyArray3<f32>>> {
    let array = input.as_array().to_owned();
    let shape = array.shape();
    if shape[2] != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "this is not an RGB array",
        ));
    }
    let mut cmyk_array = Array3::zeros([shape[0], shape[1], 4]);
    for x in 0..shape[0] {
        for y in 0..shape[1] {
            let cmyk = rgb_to_cmyk(array[[x, y, 0]], array[[x, y, 1]], array[[x, y, 2]]);
            cmyk_array[[x, y, 0]] = cmyk.0;
            cmyk_array[[x, y, 1]] = cmyk.1;
            cmyk_array[[x, y, 2]] = cmyk.2;
            cmyk_array[[x, y, 3]] = cmyk.3;
        }
    }
    Ok(cmyk_array.to_pyarray(py).to_owned())
}

#[pyfunction]
pub fn cmyk2rgb(input: PyReadonlyArray3<f32>, py: Python) -> PyResult<Py<PyArray3<f32>>> {
    let array = input.as_array().to_owned();
    let shape = array.shape();
    if shape[2] != 4 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "this is not an CMYK array",
        ));
    }
    let mut rgb_array = Array3::zeros([shape[0], shape[1], 3]);
    for x in 0..shape[0] {
        for y in 0..shape[1] {
            let rgb = cmyk_to_rgb(
                array[[x, y, 0]],
                array[[x, y, 1]],
                array[[x, y, 2]],
                array[[x, y, 3]],
            );
            rgb_array[[x, y, 0]] = rgb.0;
            rgb_array[[x, y, 1]] = rgb.1;
            rgb_array[[x, y, 2]] = rgb.2;
        }
    }
    Ok(rgb_array.to_pyarray(py).to_owned())
}
