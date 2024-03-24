use std::fs::read;
use std::path::Path;

use ndarray::{Array2, Array3, ArrayD};
use zune_jpeg::JpegDecoder;
use zune_jpeg::zune_core::colorspace::ColorSpace;
use zune_jpeg::zune_core::options::DecoderOptions;
use zune_psd::PSDDecoder;

use crate::utils::core::convert::{
    luma2array, luma2arrayf32, rgb2array, rgb2arrayf32, rgb8_to_gray32, rgb8_to_gray8, u8_to_f32,
};

pub(crate) fn gray_img_open(path: &Path) -> Array2<u8> {
    let img = image::open(path).unwrap().into_luma8();
    luma2array(img)
}

pub(crate) fn rgb_img_open(path: &Path) -> Array3<u8> {
    let img = image::open(path).unwrap().into_rgb8();
    rgb2array(img)
}

pub(crate) fn rgb_img_openf32(path: &Path) -> Array3<f32> {
    let img = image::open(path).unwrap().into_rgb8();
    rgb2arrayf32(img)
}

pub(crate) fn gray_img_openf32(path: &Path) -> Array2<f32> {
    let img = image::open(path).unwrap().into_luma8();
    luma2arrayf32(img)
}

pub(crate) fn jpg_gray_img_open(path: &Path) -> Array2<u8> {
    let file_contents = std::fs::read(path).unwrap();
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::Luma);
    let mut decoder = JpegDecoder::new_with_options(&file_contents, options);
    decoder.decode_headers().unwrap();
    let image_info = decoder.info().unwrap();
    let pixels = decoder.decode().unwrap();
    Array2::from_shape_vec(
        (image_info.height as usize, image_info.width as usize),
        pixels,
    )
        .unwrap()
}

pub(crate) fn jpg_rgb_img_open(path: &Path) -> Array3<u8> {
    let file_contents = std::fs::read(path).unwrap();
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
    let mut decoder = JpegDecoder::new_with_options(&file_contents, options);
    decoder.decode_headers().unwrap();
    let image_info = decoder.info().unwrap();
    let pixels = decoder.decode().unwrap();
    Array3::from_shape_vec(
        (image_info.height as usize, image_info.width as usize, 3),
        pixels,
    )
        .unwrap()
}

pub(crate) fn jpg_gray_img_openf32(path: &Path) -> Array2<f32> {
    let file_contents = std::fs::read(path).unwrap();
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::Luma);
    let mut decoder = JpegDecoder::new_with_options(&file_contents, options);
    decoder.decode_headers().unwrap();
    let image_info = decoder.info().unwrap();
    let pixels = decoder.decode().unwrap();
    let pixels = u8_to_f32(&pixels);
    Array2::from_shape_vec(
        (image_info.height as usize, image_info.width as usize),
        pixels,
    )
        .unwrap()
}

pub(crate) fn jpg_rgb_img_openf32(path: &Path) -> Array3<f32> {
    let file_contents = std::fs::read(path).unwrap();
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
    let mut decoder = JpegDecoder::new_with_options(&file_contents, options);
    decoder.decode_headers().unwrap();
    let image_info = decoder.info().unwrap();
    let pixels = decoder.decode().unwrap();
    let pixels = u8_to_f32(&pixels);
    Array3::from_shape_vec(
        (image_info.height as usize, image_info.width as usize, 3),
        pixels,
    )
        .unwrap()
}

fn decode_size_psd(bytes: &[u8]) -> (u32, u32) {
    let mut height: u32 = 0;
    let mut width: u32 = 0;
    height += bytes[3] as u32;
    height += if bytes[2] > 0 {
        bytes[2] as u32 * 256
    } else {
        0
    };
    height += if bytes[1] > 0 {
        bytes[1] as u32 * 256 * 256
    } else {
        0
    };
    width += bytes[7] as u32;
    width += if bytes[6] > 0 {
        bytes[6] as u32 * 256
    } else {
        0
    };
    width += if bytes[5] > 0 {
        bytes[5] as u32 * 256 * 256
    } else {
        0
    };
    (height, width)
}

pub(crate) fn psd_gray_decode(path: &Path) -> Array2<u8> {
    let img = read(path).unwrap();
    let size_bites: &[u8] = &img[14..22];
    let color_mode = img[25];
    let mut decoder = PSDDecoder::new(&img);
    let px = decoder.decode_raw().unwrap();
    let (height, width) = decode_size_psd(size_bites);
    if color_mode == 1 {
        Array2::from_shape_vec((height as usize, width as usize), px).unwrap()
    } else {
        let gray = rgb8_to_gray8(&px);
        Array2::from_shape_vec((height as usize, width as usize), gray).unwrap()
    }
}

pub(crate) fn psd_rgb_decode(path: &Path) -> Array3<u8> {
    let img = read(path).unwrap();
    let size_bites: &[u8] = &img[14..22];
    let color_mode = img[25];
    let mut decoder = PSDDecoder::new(&img);
    let px = decoder.decode_raw().unwrap();
    let (height, width) = decode_size_psd(size_bites);
    if color_mode == 1 {
        let mut rgb_values = Vec::with_capacity(px.len() * 3);

        for gray in &px {
            rgb_values.extend([*gray, *gray, *gray].iter().copied());
        }
        Array3::from_shape_vec((height as usize, width as usize, 3), rgb_values).unwrap()
    } else {
        Array3::from_shape_vec((height as usize, width as usize, 3), px).unwrap()
    }
}

pub(crate) fn psd_gray32_decode(path: &Path) -> Array2<f32> {
    let img = read(path).unwrap();
    let size_bites: &[u8] = &img[14..22];
    let color_mode = img[25];
    let mut decoder = PSDDecoder::new(&img);
    let px = decoder.decode_raw().unwrap();
    let (height, width) = decode_size_psd(size_bites);
    if color_mode == 1 {
        let px = u8_to_f32(&px);
        Array2::from_shape_vec((height as usize, width as usize), px).unwrap()
    } else {
        let gray = rgb8_to_gray32(&px);
        Array2::from_shape_vec((height as usize, width as usize), gray).unwrap()
    }
}

pub(crate) fn psd_rgb32_decode(path: &Path) -> Array3<f32> {
    let img = read(path).unwrap();
    let size_bites: &[u8] = &img[14..22];
    let color_mode = img[25];
    let mut decoder = PSDDecoder::new(&img);
    let px = decoder.decode_raw().unwrap();
    let (height, width) = decode_size_psd(size_bites);
    if color_mode == 1 {
        let mut rgb_values: Vec<f32> = Vec::with_capacity(px.len() * 3);

        for gray in &px {
            let gray_f32 = *gray as f32 * 0.00392156862745f32;
            rgb_values.extend([gray_f32, gray_f32, gray_f32].iter().copied());
        }
        Array3::from_shape_vec((height as usize, width as usize, 3), rgb_values).unwrap()
    } else {
        let px = u8_to_f32(&px);
        Array3::from_shape_vec((height as usize, width as usize, 3), px).unwrap()
    }
}

pub(crate) fn psd_din_decode(path: &Path) -> ArrayD<u8> {
    let img = read(path).unwrap();
    let size_bites: &[u8] = &img[14..22];
    let channels = img[13] as usize;
    let mut decoder = PSDDecoder::new(&img);
    let px = decoder.decode_raw().unwrap();
    let (height, width) = decode_size_psd(size_bites);
    if channels == 1 {
        Array2::from_shape_vec((height as usize, width as usize), px)
            .unwrap()
            .into_dyn()
    } else {
        Array3::from_shape_vec((height as usize, width as usize, channels), px)
            .unwrap()
            .into_dyn()
    }
}

pub(crate) fn psd_din32_decode(path: &Path) -> ArrayD<f32> {
    let img = read(path).unwrap();
    let size_bites: &[u8] = &img[14..22];
    let channels = img[13] as usize;
    let mut decoder = PSDDecoder::new(&img);
    let px = decoder.decode_raw().unwrap();
    let (height, width) = decode_size_psd(size_bites);
    let px = u8_to_f32(&px);
    if channels == 1 {
        Array2::from_shape_vec((height as usize, width as usize), px)
            .unwrap()
            .into_dyn()
    } else {
        Array3::from_shape_vec((height as usize, width as usize, channels), px)
            .unwrap()
            .into_dyn()
    }
}

pub fn all_read_u8(path: &Path, mode: u8, extension: &str) -> ArrayD<u8> {
    match extension {
        "jpg" | "jpeg" => match mode {
            0 => jpg_gray_img_open(path).into_dyn(),
            _ => jpg_rgb_img_open(path).into_dyn(),
        },
        "psd" | "PSD" => match mode {
            0 => psd_gray_decode(path).into_dyn(),
            1 => psd_rgb_decode(path).into_dyn(),
            _ => psd_din_decode(path).into_dyn(),
        },
        "error" => panic!("no_file"),
        _ => match mode {
            0 => gray_img_open(path).into_dyn(),
            _ => rgb_img_open(path).into_dyn(),
        },
    }
}

pub fn all_read_f32(path: &Path, mode: u8, extension: &str) -> ArrayD<f32> {
    match extension {
        "jpg" | "jpeg" => match mode {
            0 => jpg_gray_img_openf32(path).into_dyn(),
            _ => jpg_rgb_img_openf32(path).into_dyn(),
        },
        "psd" | "PSD" => match mode {
            0 => psd_gray32_decode(path).into_dyn(),
            1 => psd_rgb32_decode(path).into_dyn(),
            _ => psd_din32_decode(path).into_dyn(),
        },
        "error" => panic!("no_file"),
        _ => match mode {
            0 => gray_img_openf32(path).into_dyn(),
            _ => rgb_img_openf32(path).into_dyn(),
        },
    }
}
