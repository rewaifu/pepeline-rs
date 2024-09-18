use crate::utils::core::enums::ResizeFilters;
use bytemuck::cast_slice;
use fast_image_resize::images::{Image, ImageRef};
use fast_image_resize::{FilterType, PixelType, ResizeAlg, ResizeOptions, Resizer};
use image::EncodableLayout;
use ndarray::Ix;
use numpy::{PyArray, PyArrayMethods, PyReadonlyArrayDyn, PyUntypedArrayMethods};
use pyo3::{pyfunction, IntoPy,  PyObject, PyResult, Python};
use std::error::Error;

fn img_shape_to_pixel_type(img_shape: &[usize]) -> PixelType {
    if img_shape.len() == 2 {
        PixelType::F32
    } else {
        match img_shape[2] {
            1 => PixelType::F32,
            2 => PixelType::F32x2,
            3 => PixelType::F32x3,
            4 => PixelType::F32x4,
            _ => panic!("huy"),
        }
    }
}
fn get_filter_type(filter: &ResizeFilters) -> Result<FilterType, Box<dyn Error>> {
    match filter {
        ResizeFilters::Box => Ok(FilterType::Box),
        ResizeFilters::Lanczos3 => Ok(FilterType::Lanczos3),
        ResizeFilters::Bilinear => Ok(FilterType::Bilinear),
        ResizeFilters::Gaussian => Ok(FilterType::Gaussian),
        ResizeFilters::CatmullRom => Ok(FilterType::CatmullRom),
        ResizeFilters::Hamming => Ok(FilterType::Hamming),
        ResizeFilters::Mitchell => Ok(FilterType::Mitchell),
        ResizeFilters::Nearest => Err("WTF!".into()),
    }
}

fn get_res_opt(
    filter: &ResizeFilters,
    conv: bool,
    sampling: Option<u8>,
) -> Result<ResizeAlg, Box<dyn Error>> {
    match filter {
        ResizeFilters::Nearest => Ok(ResizeAlg::Nearest),
        _ => {
            let alg = get_filter_type(filter)?;
            if conv {
                Ok(ResizeAlg::Convolution(alg))
            } else if sampling.is_some() {
                Ok(ResizeAlg::SuperSampling(alg, sampling.unwrap()))
            } else {
                Ok(ResizeAlg::Interpolation(alg))
            }
        }
    }
}
#[pyfunction]
pub fn resize_img<'py>(
    input: PyReadonlyArrayDyn<f32>,
    size: (u32, u32),
    filter: Option<ResizeFilters>,
    conv: Option<bool>,
    sampling: Option<u8>,
    py: Python,
) -> PyResult<PyObject> {
    let array = input.as_array().to_owned();
    let shape = array.shape().to_vec();
    let pixel_type = img_shape_to_pixel_type(&shape);
    let mut resize = Image::new(size.0, size.1, pixel_type);
    let img = array.into_raw_vec();
    let img = ImageRef::new(shape[1] as u32, shape[0] as u32, img.as_bytes(), pixel_type).unwrap();
    let mut resizer = Resizer::new();
    resizer.resize(&img, &mut resize, &ResizeOptions::new().resize_alg(get_res_opt(&filter.unwrap_or(ResizeFilters::Nearest),conv.unwrap_or(false),sampling).unwrap())).unwrap();
    let result_vec:&[f32] = cast_slice(resize.buffer());
    if shape.get(2).is_some() {
        Ok(PyArray::from_vec_bound(py, result_vec.to_vec()).to_dyn().reshape([size.1 as Ix,size.0 as Ix,shape[2] as Ix])?.into_py(py))
    } else {
        Ok(PyArray::from_vec_bound(py, result_vec.to_vec()).to_dyn().reshape([size.1 as Ix,size.0 as Ix])?.into_py(py))
    }
}
