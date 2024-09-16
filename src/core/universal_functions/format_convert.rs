use std::cmp::min;

use image::{GrayImage, RgbImage};
use ndarray::{Array2, Array3};
use crate::core::read::byte_decode::read_u16;

pub fn gray8_to_rgb8(gray: &[u8]) -> Vec<u8> {
    let mut rgb: Vec<u8> = Vec::with_capacity(gray.len() * 3);
    for gray_byte in gray {
        rgb.push(*gray_byte);
        rgb.push(*gray_byte);
        rgb.push(*gray_byte);
    }
    rgb
}

pub fn gray16u8_to_rgb16(gray: &[u8]) -> Vec<u16> {
    let mut rgb: Vec<u16> = Vec::with_capacity(gray.len() / 2 * 3);
    for gray_byte in gray.chunks(2) {
        let u16_byte = read_u16(gray_byte);
        rgb.push(u16_byte);
        rgb.push(u16_byte);
        rgb.push(u16_byte);
    }
    rgb
}

pub fn gray16u8_to_rgba16(gray: &[u8]) -> Vec<u16> {
    let mut rgb: Vec<u16> = Vec::with_capacity(gray.len() / 2 * 3);
    for gray_byte in gray.chunks(2) {
        let u16_byte = read_u16(gray_byte);
        rgb.push(u16_byte);
        rgb.push(u16_byte);
        rgb.push(u16_byte);
        rgb.push(u16::MAX);
    }
    rgb
}

pub fn gray8_to_rgba8(gray: &[u8]) -> Vec<u8> {
    let mut rgb: Vec<u8> = Vec::with_capacity(gray.len() * 3);
    for gray_byte in gray {
        rgb.push(*gray_byte);
        rgb.push(*gray_byte);
        rgb.push(*gray_byte);
        rgb.push(u8::MAX);
    }
    rgb
}

pub fn rgb8_to_gray8(rgb: &[u8]) -> Vec<u8> {
    let mut gray_float: Vec<u8> = Vec::with_capacity(rgb.len() / 3);
    for chunk in rgb.chunks(3) {
        gray_float.push(
            (chunk[0] as f32 * 0.2126 + chunk[1] as f32 * 0.7152 + chunk[2] as f32 * 0.0722) as u8,
        )
    }
    gray_float
}

pub fn rgb16u8_to_gray16(rgb: &[u8]) -> Vec<u16> {
    let mut gray_float: Vec<u16> = Vec::with_capacity(rgb.len() / 2 / 3);
    for chunk in rgb.chunks(6) {
        gray_float.push(
            (read_u16(&chunk[0..2]) as f32 * 0.2126 + read_u16(&chunk[2..4]) as f32 * 0.7152 + read_u16(&chunk[4..6]) as f32 * 0.0722) as u16,
        )
    }
    gray_float
}
pub fn rgb16u8_to_agray16(rgb: &[u8]) -> Vec<u16> {
    let mut gray_float: Vec<u16> = Vec::with_capacity(rgb.len()  / 3*4/3/2);
    for chunk in rgb.chunks(6) {
        gray_float.push(
            (read_u16(&chunk[0..2]) as f32 * 0.2126 + read_u16(&chunk[2..4]) as f32 * 0.7152 + read_u16(&chunk[4..6]) as f32 * 0.0722) as u16,
        );
        gray_float.push(u16::MAX)
    }
    gray_float
}
pub fn rgb8_to_agray8(rgb: &[u8]) -> Vec<u8> {
    let mut gray_float: Vec<u8> = Vec::with_capacity(rgb.len() / 3 * 4 / 3);
    for chunk in rgb.chunks(3) {
        gray_float.push(
            (chunk[0] as f32 * 0.2126 + chunk[1] as f32 * 0.7152 + chunk[2] as f32 * 0.0722) as u8,
        );
        gray_float.push(u8::MAX)
    }
    gray_float
}


pub(crate) fn rgb8_to_gray32(rgb: &[u8]) -> Vec<f32> {
    let mut gray_float: Vec<f32> = Vec::with_capacity(rgb.len() / 3);
    for chunk in rgb.chunks(3) {
        gray_float.push(
            (chunk[0] as f32 * 0.2126 + chunk[1] as f32 * 0.7152 + chunk[2] as f32 * 0.0722)
                / 255.0,
        )
    }
    gray_float
}

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
    bytes.iter()
        .map(|&x| (x * 255.0).round() as u8)
        .collect()
}
pub(crate) fn f32_to_u16(bytes: &[f32]) -> Vec<u16> {
    bytes.iter()
        .map(|&x| (x * 65535.0).round() as u16)
        .collect()
}

pub(crate) fn u16_to_f32(bytes: &[u16]) -> Vec<f32> {
    bytes.iter()
        .map(|&x| x as f32 / 65535.0
        ).collect()
}
pub(crate) fn u16_to_u8(bytes: &[u16]) -> Vec<u8> {
    bytes.iter()
        .map(|&x| {
            let f32_value = x as f32 / 65535.0;
            (f32_value * 255.0) as u8
        }).collect()
}
pub(crate) fn u8_to_u16(bytes: &[u8]) -> Vec<u16> {
    bytes.iter()
        .map(|&x| {
            let f32_value = x as f32 / 255.0;
            (f32_value * 65535.0) as u16
        }).collect()
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
