use image::{GrayImage, RgbImage};
use ndarray::{Array2, Array3};

pub(crate) fn rgb8_to_gray8(rgb: (u8, u8, u8)) -> u8 {
    let (r, g, b) = rgb;
    let gray = (r as f32 * 0.114 + g as f32 * 0.587 + b as f32 * 0.299) as u8;
    gray
}

pub(crate) fn rgb8_to_gray32(rgb: (u8, u8, u8)) -> f32 {
    let (r, g, b) = rgb;
    let gray = (r as f32 * 0.114 + g as f32 * 0.587 + b as f32 * 0.299) / 255.0;
    gray
}

//TODO переписать.
pub(crate) fn u8_to_f32(bytes: &[u8]) -> Vec<f32> {
    let mut floats = vec![0.0; bytes.len()];
    floats.iter_mut().zip(bytes.iter()).for_each(|(f, &b)| {
        *f = if b == 0 {
            b as f32
        } else {
            b as f32 * 0.00392156862745f32
        }
    });

    floats
}

pub(crate) fn f32_to_u8(bytes: &[f32]) -> Vec<u8> {
    let mut floats = vec![0; bytes.len()];
    floats
        .iter_mut()
        .zip(bytes.iter())
        .for_each(|(f, &b)| *f = if b == 0.0 { b as u8 } else { (b * 255.0) as u8 });
    floats
}

pub(crate) fn rgb2arrayf32(img: RgbImage) -> Array3<f32> {
    let (width, height) = img.dimensions();
    let input = img.into_raw();
    let input_f32 = u8_to_f32(&input);

    Array3::from_shape_vec((height as usize, width as usize, 3), input_f32).unwrap()
}

pub(crate) fn luma2arrayf32(img: GrayImage) -> Array2<f32> {
    let (width, height) = img.dimensions();
    let input = img.into_raw();
    let input_f32 = u8_to_f32(&input);
    Array2::from_shape_vec((height as usize, width as usize), input_f32).unwrap()
}

pub(crate) fn luma2array(img: GrayImage) -> Array2<u8> {
    let (width, height) = img.dimensions();
    let input = img.into_raw();
    Array2::from_shape_vec((height as usize, width as usize), input).unwrap()
}

pub(crate) fn rgb2array(img: RgbImage) -> Array3<u8> {
    let (width, height) = img.dimensions();
    let input = img.into_raw();
    Array3::from_shape_vec((height as usize, width as usize, 3), input).unwrap()
}
