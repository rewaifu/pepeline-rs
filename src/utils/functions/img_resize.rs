use crate::utils::core::enums::ResizeFilters;
use crate::utils::image::resize::resize_image;
use bytemuck::cast_slice;
use fast_image_resize::images::{Image, ImageRef};
use fast_image_resize::{FilterType, PixelType, ResizeAlg};
use image::EncodableLayout;
use ndarray::{Array2, Array3};
use numpy::{PyArrayDyn, PyReadonlyArrayDyn, ToPyArray};
use pyo3::{pyfunction, Py, PyResult, Python};
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
) -> PyResult<Py<PyArrayDyn<f32>>> {
    let array = input.as_array().to_owned();
    let shape = array.shape().to_vec();
    let pixel_type = img_shape_to_pixel_type(&shape);
    let mut resize = Image::new(size.0, size.1, pixel_type);
    let img = array.into_raw_vec();
    let img = ImageRef::new(shape[1] as u32, shape[0] as u32, img.as_bytes(), pixel_type).unwrap();
    resize_image(
        &img,
        get_res_opt(
            &filter.unwrap_or(ResizeFilters::Nearest),
            conv.unwrap_or(false),
            sampling,
        )
        .unwrap_or(ResizeAlg::Nearest),
        &mut resize,
    )
    .unwrap();
    let result_vec = resize.buffer();
    if shape.get(2).is_some() {
        Ok(unsafe {
            Array3::from_shape_vec_unchecked(
                [size.1 as usize, size.0 as usize, shape[2]],
                cast_slice(result_vec).to_vec(),
            )
            .into_dyn()
        }
        .to_pyarray_bound(py)
        .into())
    } else {
        Ok(unsafe {
            Array2::from_shape_vec_unchecked(
                [size.1 as usize, size.0 as usize],
                cast_slice(result_vec).to_vec(),
            )
            .into_dyn()
        }
        .to_pyarray_bound(py)
        .into())
    }
}
