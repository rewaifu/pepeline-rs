use std::cmp::{max, min};

use ndarray::{Array2, Array3};
use noise::{NoiseFn, OpenSimplex, Perlin, PerlinSurflet, Simplex, SuperSimplex};
use numpy::{PyArrayDyn, PyReadonlyArrayDyn, ToPyArray};
use pyo3::{Py, pyfunction, PyResult, Python};
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
            Ok(array.into_dyn().to_pyarray_bound(py).into())
        }
        3 => {
            let mut array: Array3<f32> = Array3::zeros((size[0], size[1], size[2]));
            let type_fn = generate_noise3d(type_noise, seed);
            for ((x, y, z), value) in array.indexed_iter_mut() {
                *value = noise_3d(&type_fn, x, y, z, octaves, frequency, lacunarity);
            }
            Ok(array.into_dyn().to_pyarray_bound(py).into())
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

// #[pyfunction]
// pub fn cmyk_shift<'py>(img: PyReadonlyArray3<f32>, c_bias: Vec<isize>, m_bias: Vec<isize>, y_bias: Vec<isize>, k_bias: Vec<isize>, py: Python) -> PyResult<Py<PyArray3<f32>>> {
//     let array = img.as_array().to_owned();
//     let shape_img = array.shape();
//     let hh = 30;
//     let mut result: Array3<f32> = Array3::ones([shape_img[0], shape_img[1], shape_img[2]]);
//     for x in 0..shape_img[0] {
//         let amount_cx = x as isize + c_bias[0];
//         let amount_mx = x as isize + m_bias[0];
//         let amount_yx = x as isize + y_bias[0];
//         let amount_kx = x as isize + k_bias[0];
//         let a = rand::thread_rng().gen_range(-hh..=hh) as isize;
//         let b = rand::thread_rng().gen_range(-hh..=hh) as isize;
//         let c = rand::thread_rng().gen_range(-hh..=hh) as isize;
//         let d = rand::thread_rng().gen_range(-hh..=hh) as isize;
//         for y in 0..shape_img[1] {
//             let amount_cy = y as isize + c_bias[1] + a;
//             let amount_my = y as isize + m_bias[1] + b;
//             let amount_yy = y as isize + y_bias[1] + c;
//             let amount_ky = y as isize + k_bias[1] + d;
//             if amount_cx > 0 && amount_cy > 0 && amount_cx < shape_img[0] as isize && amount_cy < shape_img[1] as isize {
//                 result[[x, y, 0]] = array[[amount_cx as usize, amount_cy as usize, 0]]
//             }
//             if amount_mx > 0 && amount_my > 0 && amount_mx < shape_img[0] as isize && amount_my < shape_img[1] as isize {
//                 result[[x, y, 1]] = array[[amount_mx as usize, amount_my as usize, 1]]
//             }
//             if amount_yx > 0 && amount_yy > 0 && amount_yx < shape_img[0] as isize && amount_yy < shape_img[1] as isize {
//                 result[[x, y, 2]] = array[[amount_yx as usize, amount_yy as usize, 2]]
//             }
//             if amount_kx > 0 && amount_ky > 0 && amount_kx < shape_img[0] as isize && amount_ky < shape_img[1] as isize {
//                 result[[x, y, 3]] = array[[amount_kx as usize, amount_ky as usize, 3]]
//             }
//         }
//     }
//     Ok(result.to_pyarray_bound(py).into())
// }