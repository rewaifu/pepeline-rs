use std::error::Error;
use std::io::Cursor;
use std::path::Path;

use filebuffer::FileBuffer;
use ndarray::{Array2, Array3, ArrayD};
use zune_jpeg::zune_core::colorspace::ColorSpace;
use zune_jpeg::zune_core::options::DecoderOptions;
use zune_jpeg::JpegDecoder;
use zune_psd::PSDDecoder;

use crate::utils::core::convert::{
    luma2array, luma2arrayf32, rgb2array, rgb2arrayf32, rgb8_to_gray32, rgb8_to_gray8, u16_to_f32,
    u16_to_u8, u8_to_f32,
};

pub(crate) fn gray_img_open(bytes: &[u8]) -> Result<Array2<u8>, Box<dyn Error>> {
    let img = image::io::Reader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;
    let img_luma = img.to_luma8();
    Ok(luma2array(img_luma))
}

pub(crate) fn rgb_img_open(bytes: &[u8]) -> Result<Array3<u8>, Box<dyn Error>> {
    let img = image::io::Reader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;
    let img_rgb8 = img.to_rgb8();
    Ok(rgb2array(img_rgb8))
}

pub(crate) fn rgb_img_openf32(bytes: &[u8]) -> Result<Array3<f32>, Box<dyn Error>> {
    let img = image::io::Reader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;
    let img_rgb8 = img.to_rgb8();
    Ok(rgb2arrayf32(img_rgb8))
}

pub(crate) fn gray_img_openf32(bytes: &[u8]) -> Result<Array2<f32>, Box<dyn Error>> {
    let img = image::io::Reader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;
    let img_luma = img.to_luma8();
    Ok(luma2arrayf32(img_luma))
}

pub(crate) fn jpg_gray_img_open(file: &[u8]) -> Result<Array2<u8>, Box<dyn Error>> {
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::Luma);
    let mut decoder = JpegDecoder::new_with_options(file, options);
    decoder
        .decode_headers()
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let image_info = decoder.info().ok_or("Failed to get image info")?;
    let pixels = decoder
        .decode()
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok(Array2::from_shape_vec(
        (image_info.height as usize, image_info.width as usize),
        pixels,
    )?)
}

pub(crate) fn jpg_rgb_img_open(file: &[u8]) -> Result<Array3<u8>, Box<dyn Error>> {
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
    let mut decoder = JpegDecoder::new_with_options(file, options);
    decoder.decode_headers()?;
    let image_info = decoder.info().ok_or("error read image info")?;
    let pixels = decoder
        .decode()
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok(Array3::from_shape_vec(
        (image_info.height as usize, image_info.width as usize, 3),
        pixels,
    )?)
}

pub(crate) fn jpg_gray_img_openf32(file: &[u8]) -> Result<Array2<f32>, Box<dyn Error>> {
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::Luma);
    let mut decoder = JpegDecoder::new_with_options(file, options);
    decoder.decode_headers()?;
    let image_info = decoder.info().ok_or("error read image info")?;
    let pixels = decoder.decode()?;
    let pixels = u8_to_f32(&pixels);
    Ok(Array2::from_shape_vec(
        (image_info.height as usize, image_info.width as usize),
        pixels,
    )?)
}

pub(crate) fn jpg_rgb_img_openf32(file: &[u8]) -> Result<Array3<f32>, Box<dyn Error>> {
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
    let mut decoder = JpegDecoder::new_with_options(file, options);
    decoder.decode_headers()?;
    let image_info = decoder.info().ok_or("error read image info")?;
    let pixels = decoder.decode()?;
    let pixels = u8_to_f32(&pixels);
    Ok(Array3::from_shape_vec(
        (image_info.height as usize, image_info.width as usize, 3),
        pixels,
    )?)
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

pub(crate) fn psd_gray_decode(img: &[u8]) -> Result<Array2<u8>, Box<dyn Error>> {
    let size_bites: &[u8] = &img[14..22];
    let color_mode = img[25];
    let mut decoder = PSDDecoder::new(img);
    let mut px = decoder.decode_raw().unwrap();
    if &img[23] == &16 {
        px = u16_to_u8(&px);
    }
    let (height, width) = decode_size_psd(size_bites);
    if color_mode == 1 {
        Ok(Array2::from_shape_vec(
            (height as usize, width as usize),
            px,
        )?)
    } else {
        let gray = rgb8_to_gray8(&px);
        Ok(Array2::from_shape_vec(
            (height as usize, width as usize),
            gray,
        )?)
    }
}

pub(crate) fn psd_rgb_decode(img: &[u8]) -> Result<Array3<u8>, Box<dyn Error>> {
    let size_bites: &[u8] = &img[14..22];
    let color_mode = img[25];
    let mut decoder = PSDDecoder::new(img);
    let mut px = decoder.decode_raw().unwrap();
    if &img[23] == &16 {
        px = u16_to_u8(&px);
    }
    let (height, width) = decode_size_psd(size_bites);
    if color_mode == 1 {
        let mut rgb_values = Vec::with_capacity(px.len() * 3);

        for gray in &px {
            rgb_values.extend([*gray, *gray, *gray].iter().copied());
        }
        Ok(Array3::from_shape_vec(
            (height as usize, width as usize, 3),
            rgb_values,
        )?)
    } else {
        Ok(Array3::from_shape_vec(
            (height as usize, width as usize, 3),
            px,
        )?)
    }
}

pub(crate) fn psd_gray32_decode(img: &[u8]) -> Result<Array2<f32>, Box<dyn Error>> {
    let size_bites: &[u8] = &img[14..22];
    let color_mode = img[25];
    let mut decoder = PSDDecoder::new(img);
    let mut px = decoder.decode_raw().unwrap();
    if &img[23] == &16 {
        px = u16_to_u8(&px);
    }
    let (height, width) = decode_size_psd(size_bites);
    if color_mode == 1 {
        let px = u8_to_f32(&px);
        Ok(Array2::from_shape_vec(
            (height as usize, width as usize),
            px,
        )?)
    } else {
        let gray = rgb8_to_gray32(&px);
        Ok(Array2::from_shape_vec(
            (height as usize, width as usize),
            gray,
        )?)
    }
}

pub(crate) fn psd_rgb32_decode(img: &[u8]) -> Result<Array3<f32>, Box<dyn Error>> {
    let size_bites: &[u8] = &img[14..22];
    let color_mode = img[25];
    let mut decoder = PSDDecoder::new(img);
    let mut px = decoder.decode_raw().unwrap();
    let (height, width) = decode_size_psd(size_bites);
    if &img[23] == &16 {
        px = u16_to_u8(&px);
    }
    if color_mode == 1 {
        let mut rgb_values: Vec<f32> = Vec::with_capacity(px.len() * 3);

        for gray in &px {
            let gray_f32 = *gray as f32 * 0.00392156862745f32;
            rgb_values.extend([gray_f32, gray_f32, gray_f32].iter().copied());
        }
        Ok(Array3::from_shape_vec(
            (height as usize, width as usize, 3),
            rgb_values,
        )?)
    } else {
        let px = u8_to_f32(&px);
        Ok(Array3::from_shape_vec(
            (height as usize, width as usize, 3),
            px,
        )?)
    }
}

pub(crate) fn psd_din_decode(img: &[u8]) -> Result<ArrayD<u8>, Box<dyn Error>> {
    let size_bites: &[u8] = &img[14..22];
    let channels = img[13] as usize;
    let mut decoder = PSDDecoder::new(img);
    let mut px = decoder.decode_raw().unwrap();
    let (height, width) = decode_size_psd(size_bites);
    if &img[23] == &16 {
        px = u16_to_u8(&px);
    }
    if channels == 1 {
        Ok(Array2::from_shape_vec((height as usize, width as usize), px)?.into_dyn())
    } else {
        Ok(Array3::from_shape_vec((height as usize, width as usize, channels), px)?.into_dyn())
    }
}

pub(crate) fn psd_din32_decode(img: &[u8]) -> Result<ArrayD<f32>, Box<dyn Error>> {
    let size_bites: &[u8] = &img[14..22];
    let channels = img[13] as usize;
    let mut decoder = PSDDecoder::new(img);
    let px = decoder.decode_raw().unwrap();
    let (height, width) = decode_size_psd(size_bites);
    let px_float = match &img[23] {
        16 => u16_to_f32(&px),
        8 => u8_to_f32(&px),
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Unsupported bits: {}", &img[23]),
            )));
        }
    };
    if channels == 1 {
        Ok(Array2::from_shape_vec((height as usize, width as usize), px_float)?.into_dyn())
    } else {
        Ok(
            Array3::from_shape_vec((height as usize, width as usize, channels), px_float)?
                .into_dyn(),
        )
    }
}

pub fn all_read_u8(path: &Path, mode: u8) -> Result<ArrayD<u8>, Box<dyn Error>> {
    let img = FileBuffer::open(path).map_err(|err| Box::new(err) as Box<dyn Error>)?;
    let img_magic_byte = &img[..4];
    match img_magic_byte {
        [255, 216, 255, 224] | [255, 216, 255, 225] => match &img[6..8] {
            [74, 70] | [69, 120] => match mode {
                0 => Ok(gray_img_open(&img)?.into_dyn()),
                _ => Ok(rgb_img_open(&img)?.into_dyn()),
            },
            _ => match mode {
                0 => Ok(jpg_gray_img_open(&img)?.into_dyn()),
                _ => Ok(jpg_rgb_img_open(&img)?.into_dyn()),
            },
        },
        [56, 66, 80, 83] => match mode {
            0 => Ok(psd_gray_decode(&img)?.into_dyn()),
            1 => Ok(psd_rgb_decode(&img)?.into_dyn()),
            _ => Ok(psd_din_decode(&img)?.into_dyn()),
        },
        _ => match mode {
            0 => Ok(gray_img_open(&img)?.into_dyn()),
            _ => Ok(rgb_img_open(&img)?.into_dyn()),
        },
    }
}

pub fn all_read_f32(path: &Path, mode: u8) -> Result<ArrayD<f32>, Box<dyn Error>> {
    let img = FileBuffer::open(path).map_err(|err| Box::new(err) as Box<dyn Error>)?;
    let img_magic_byte = &img[..4];
    match img_magic_byte {
        [255, 216, 255, 224] | [255, 216, 255, 225] => match &img[6..8] {
            [74, 70] | [69, 120] => match mode {
                0 => Ok(gray_img_openf32(&img)?.into_dyn()),
                _ => Ok(rgb_img_openf32(&img)?.into_dyn()),
            },
            _ => match mode {
                0 => Ok(jpg_gray_img_openf32(&img)?.into_dyn()),
                _ => Ok(jpg_rgb_img_openf32(&img)?.into_dyn()),
            },
        },
        [56, 66, 80, 83] => match mode {
            0 => Ok(psd_gray32_decode(&img)?.into_dyn()),
            1 => Ok(psd_rgb32_decode(&img)?.into_dyn()),
            _ => Ok(psd_din32_decode(&img)?.into_dyn()),
        },
        _ => match mode {
            0 => Ok(gray_img_openf32(&img)?.into_dyn()),
            _ => Ok(rgb_img_openf32(&img)?.into_dyn()),
        },
    }
}
