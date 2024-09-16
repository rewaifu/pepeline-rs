use std::error::Error;
use image::DynamicImage;
use ndarray::{Array2, Array3, ArrayD};
use crate::core::read::byte_decode::read_u16;
use crate::core::universal_functions::enums::{ArrayDType, ImgFormat};
use crate::core::universal_functions::format_convert::{f32_to_u16, f32_to_u8, gray16u8_to_rgb16, gray16u8_to_rgba16, gray8_to_rgb8, gray8_to_rgba8, rgb16u8_to_agray16, rgb16u8_to_gray16, rgb8_to_agray8, rgb8_to_gray8, u16_to_f32, u16_to_u8, u8_to_f32, u8_to_u16};

pub(crate) fn rgb_img2array_du8(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    let img_vec = img.to_rgb8().into_raw();
    match color_format {  
        ImgFormat::U8|ImgFormat::DYNAMIC=>{
            let array: ArrayD<u8> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 3),
                img_vec)?.into_dyn();
            Ok(ArrayDType::U8(array))
        }
        ImgFormat::F32=>{
            let array: ArrayD<f32> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 3),
                u8_to_f32(&img_vec))?.into_dyn();
            Ok(ArrayDType::F32(array))
        }
        ImgFormat::U16=>{
            let array: ArrayD<u16> = Array3::from_shape_vec(
            (img.height() as usize, img.width() as usize, 3),
            u8_to_u16(&img_vec))?.into_dyn();
            Ok(ArrayDType::U16(array))
        }
    }

}

pub(crate) fn rgb_img2array_du16(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    let img_vec = img.to_rgb16().into_raw();
    match color_format { 
        ImgFormat::DYNAMIC|ImgFormat::U16=>{
            let array: ArrayD<u16> = Array3::from_shape_vec(
            (img.height() as usize, img.width() as usize, 3),
            img_vec)?.into_dyn();
            Ok(ArrayDType::U16(array))
        }
        ImgFormat::F32=>{
            let array: ArrayD<f32> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 3),
                u16_to_f32(&img_vec))?.into_dyn();
            Ok(ArrayDType::F32(array))
        }
        ImgFormat::U8=>{
            let array: ArrayD<u8> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 3),
                u16_to_u8(&img_vec))?.into_dyn();
            Ok(ArrayDType::U8(array))
        }
    }

}

pub(crate) fn rgb_img2array_df32(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    let img_vec = img.to_rgb32f().into_raw();
    match color_format {
        ImgFormat::DYNAMIC|ImgFormat::F32=>{
            let array: ArrayD<f32> = Array3::from_shape_vec(
            (img.height() as usize, img.width() as usize, 3),
            img_vec)?.into_dyn();
            Ok(ArrayDType::F32(array))
        }
        ImgFormat::U8=>{
            let array: ArrayD<u8> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 3),
               f32_to_u8(&img_vec))?.into_dyn();
            Ok(ArrayDType::U8(array))
        }
        ImgFormat::U16=>{
            let array: ArrayD<u16> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 3),
                f32_to_u16(&img_vec))?.into_dyn();
            Ok(ArrayDType::U16(array))
        }
    }
    

}

pub(crate) fn rgba_img2array_du8(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    let img_vec = img.to_rgba8().into_raw();
    match color_format {
        ImgFormat::U8|ImgFormat::DYNAMIC=>{
            let array: ArrayD<u8> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 4),
                img_vec)?.into_dyn();
            Ok(ArrayDType::U8(array))
        }
        ImgFormat::F32=>{
            let array: ArrayD<f32> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 4),
                u8_to_f32(&img_vec))?.into_dyn();
            Ok(ArrayDType::F32(array))
        }
        ImgFormat::U16=>{
            let array: ArrayD<u16> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 4),
                u8_to_u16(&img_vec))?.into_dyn();
            Ok(ArrayDType::U16(array))
        }
    }
}

pub(crate) fn rgba_img2array_du16(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    let img_vec = img.to_rgba16().into_raw();
    match color_format {
        ImgFormat::DYNAMIC|ImgFormat::U16=>{
            let array: ArrayD<u16> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 4),
                img_vec)?.into_dyn();
            Ok(ArrayDType::U16(array))
        }
        ImgFormat::F32=>{
            let array: ArrayD<f32> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 4),
                u16_to_f32(&img_vec))?.into_dyn();
            Ok(ArrayDType::F32(array))
        }
        ImgFormat::U8=>{
            let array: ArrayD<u8> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 4),
                u16_to_u8(&img_vec))?.into_dyn();
            Ok(ArrayDType::U8(array))
        }
    }

}

pub(crate) fn rgba_img2array_df32(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    let img_vec =img.to_rgba32f().into_raw();
    match color_format {
        ImgFormat::DYNAMIC|ImgFormat::F32=>{
            let array: ArrayD<f32> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 4),
                img_vec)?.into_dyn();
            Ok(ArrayDType::F32(array))
        }
        ImgFormat::U8=>{
            let array: ArrayD<u8> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 4),
                f32_to_u8(&img_vec))?.into_dyn();
            Ok(ArrayDType::U8(array))
        }
        ImgFormat::U16=>{
            let array: ArrayD<u16> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 4),
                f32_to_u16(&img_vec))?.into_dyn();
            Ok(ArrayDType::U16(array))
        }
    }
}

pub(crate) fn luma_img2array_du8(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    let img_vec =img.to_luma8().into_raw();
    match color_format { 
        ImgFormat::DYNAMIC|ImgFormat::U8=>{
            let array: ArrayD<u8> = Array2::from_shape_vec(
            (img.height() as usize, img.width() as usize),
            img_vec )?.into_dyn();
            Ok(ArrayDType::U8(array))
        }
        ImgFormat::F32=>{
            let array: ArrayD<f32> = Array2::from_shape_vec(
                (img.height() as usize, img.width() as usize),
                u8_to_f32(&img_vec) )?.into_dyn();
            Ok(ArrayDType::F32(array))
        }
        ImgFormat::U16=>{
            let array: ArrayD<u16> = Array2::from_shape_vec(
                (img.height() as usize, img.width() as usize),
                u8_to_u16(&img_vec))?.into_dyn();
            Ok(ArrayDType::U16(array))
        }
    }

}

pub(crate) fn luma_img2array_du16(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    let image_vec = img.to_luma16().into_raw();
    match color_format {
        ImgFormat::DYNAMIC|ImgFormat::U16=>{
            let array: ArrayD<u16> = Array2::from_shape_vec(
                (img.height() as usize, img.width() as usize),
                image_vec)?.into_dyn();
            Ok(ArrayDType::U16(array))
        }
        ImgFormat::F32=>{
                let array: ArrayD<f32> = Array2::from_shape_vec(
                    (img.height() as usize, img.width() as usize),
                    u16_to_f32(&image_vec))?.into_dyn();
                Ok(ArrayDType::F32(array))
        }
        ImgFormat::U8=>{
            let array: ArrayD<u8> = Array2::from_shape_vec(
                (img.height() as usize, img.width() as usize),
                u16_to_u8(&image_vec))?.into_dyn();
            Ok(ArrayDType::U8(array))
        }
    }

}

pub(crate) fn aluma_img2array_du8(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    let image_vec = img.to_luma_alpha8().into_raw();
    match color_format { 
        ImgFormat::DYNAMIC|ImgFormat::U8=>{
            let array: ArrayD<u8> = Array3::from_shape_vec(
            (img.height() as usize, img.width() as usize, 2),
            image_vec)?.into_dyn();
            Ok(ArrayDType::U8(array))
        }
        ImgFormat::F32=>{
            let array: ArrayD<f32> = Array3::from_shape_vec(
            (img.height() as usize, img.width() as usize, 2),
            u8_to_f32(&image_vec))?.into_dyn();
            Ok(ArrayDType::F32(array))
        }
        ImgFormat::U16=>{
            let array: ArrayD<u16> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 2),
                u8_to_u16(&image_vec))?.into_dyn();
            Ok(ArrayDType::U16(array))
        }
    }
    

}

pub(crate) fn aluma_img2array_du16(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    let image_vec = img.to_luma_alpha16().into_raw();
    match color_format { 
        ImgFormat::DYNAMIC|ImgFormat::U16=>{
            let array: ArrayD<u16> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 2),
                image_vec)?.into_dyn();
            Ok(ArrayDType::U16(array))
        }
        ImgFormat::U8=>{
            let array: ArrayD<u8> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 2),
                u16_to_u8(&image_vec))?.into_dyn();
            Ok(ArrayDType::U8(array))
        }
        ImgFormat::F32=>{
            let array: ArrayD<f32> = Array3::from_shape_vec(
                (img.height() as usize, img.width() as usize, 2),
                u16_to_f32(&image_vec))?.into_dyn();
            Ok(ArrayDType::F32(array))
        }
    }

}

pub(crate) fn dynamic_img2array_dtype(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    match img {
        DynamicImage::ImageRgb8(_) => Ok(rgb_img2array_du8(img,color_format)?),
        DynamicImage::ImageRgb16(_) => Ok(rgb_img2array_du16(img,color_format)?),
        DynamicImage::ImageRgb32F(_) => Ok(rgb_img2array_df32(img,color_format)?),

        DynamicImage::ImageRgba8(_) => Ok(rgba_img2array_du8(img,color_format)?),
        DynamicImage::ImageRgba16(_) => Ok(rgba_img2array_du16(img,color_format)?),
        DynamicImage::ImageRgba32F(_) => Ok(rgba_img2array_df32(img,color_format)?),

        DynamicImage::ImageLuma8(_) => Ok(luma_img2array_du8(img,color_format)?),
        DynamicImage::ImageLuma16(_) => Ok(luma_img2array_du16(img,color_format)?),

        DynamicImage::ImageLumaA8(_) => Ok(aluma_img2array_du8(img,color_format)?),
        DynamicImage::ImageLumaA16(_) => Ok(aluma_img2array_du16(img,color_format)?),
        _ => Err("unsupported image format".into())
    }
}

pub(crate) fn dynamic_rgb_img2array_dtype(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    match img {
        DynamicImage::ImageRgb8(_) |
        DynamicImage::ImageRgba8(_) |
        DynamicImage::ImageLuma8(_) |
        DynamicImage::ImageLumaA8(_) => Ok(rgb_img2array_du8(img,color_format)?),

        DynamicImage::ImageRgb16(_) |
        DynamicImage::ImageRgba16(_) |
        DynamicImage::ImageLuma16(_) |
        DynamicImage::ImageLumaA16(_) => Ok(rgb_img2array_du16(img,color_format)?),

        DynamicImage::ImageRgb32F(_) |
        DynamicImage::ImageRgba32F(_) => Ok(rgb_img2array_df32(img,color_format)?),
        _ => Err("unsupported image format".into())
    }
}

pub(crate) fn dynamic_rgba_img2array_dtype(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    match img {
        DynamicImage::ImageRgb8(_) |
        DynamicImage::ImageRgba8(_) |
        DynamicImage::ImageLuma8(_) |
        DynamicImage::ImageLumaA8(_) => Ok(rgba_img2array_du8(img,color_format)?),

        DynamicImage::ImageRgb16(_) |
        DynamicImage::ImageRgba16(_) |
        DynamicImage::ImageLuma16(_) |
        DynamicImage::ImageLumaA16(_) => Ok(rgba_img2array_du16(img,color_format)?),

        DynamicImage::ImageRgb32F(_) |
        DynamicImage::ImageRgba32F(_) => Ok(rgba_img2array_df32(img,color_format)?),
        _ => Err("unsupported image format".into())
    }
}

pub(crate) fn dynamic_luma_img2array_dtype(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    match img {
        DynamicImage::ImageRgb8(_) |
        DynamicImage::ImageRgba8(_) |
        DynamicImage::ImageLuma8(_) |
        DynamicImage::ImageRgb32F(_) |
        DynamicImage::ImageRgba32F(_) |
        DynamicImage::ImageLumaA8(_) => Ok(luma_img2array_du8(img,color_format)?),

        DynamicImage::ImageRgb16(_) |
        DynamicImage::ImageRgba16(_) |
        DynamicImage::ImageLuma16(_) |
        DynamicImage::ImageLumaA16(_) => Ok(luma_img2array_du16(img,color_format)?),
        _ => Err("unsupported image format".into())
    }
}

pub(crate) fn dynamic_aluma_img2array_dtype(img: &DynamicImage,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    match img {
        DynamicImage::ImageRgb8(_) |
        DynamicImage::ImageRgba8(_) |
        DynamicImage::ImageLuma8(_) |
        DynamicImage::ImageRgb32F(_) |
        DynamicImage::ImageRgba32F(_) |
        DynamicImage::ImageLumaA8(_) => Ok(aluma_img2array_du8(img,color_format)?),

        DynamicImage::ImageRgb16(_) |
        DynamicImage::ImageRgba16(_) |
        DynamicImage::ImageLuma16(_) |
        DynamicImage::ImageLumaA16(_) => Ok(aluma_img2array_du16(img,color_format)?),
        _ => Err("unsupported image format".into())
    }
}

pub fn psd_dynamic2dtype(px: &[u8], channels: usize, format_type: &u8, height: u32, width: u32) -> Result<ArrayDType, Box<dyn Error>> {
    if channels == 1 {
        if format_type == &16 {
            let mut u16_px: Vec<u16> = Vec::with_capacity(&px.len() / 2);
            for dooble_u8 in px.chunks(2) {
                u16_px.push(read_u16(dooble_u8))
            }
            Ok(ArrayDType::U16(Array2::from_shape_vec((height as usize, width as usize), u16_px)?.into_dyn()))
        } else {
            Ok(ArrayDType::U8(Array2::from_shape_vec((height as usize, width as usize), px.into())?.into_dyn()))
        }
    } else {
        if format_type == &16 {
            let mut u16_px: Vec<u16> = Vec::with_capacity(&px.len() / 2);
            for dooble_u8 in px.chunks(2) {
                u16_px.push(read_u16(dooble_u8))
            }
            Ok(ArrayDType::U16(Array3::from_shape_vec((height as usize, width as usize, channels), u16_px)?.into_dyn()))
        } else {
            Ok(ArrayDType::U8(Array3::from_shape_vec((height as usize, width as usize, channels), px.into())?.into_dyn()))
        }
    }
}

pub fn psd_rgb2dtype(px: &[u8], channels: usize, format_type: &u8, height: u32, width: u32) -> Result<ArrayDType, Box<dyn Error>> {
    if channels == 1 {
        if format_type == &16 {
            let u16_px = gray16u8_to_rgb16(&px);
            Ok(ArrayDType::U16(Array2::from_shape_vec((height as usize, width as usize), u16_px)?.into_dyn()))
        } else {
            Ok(ArrayDType::U8(Array2::from_shape_vec((height as usize, width as usize), gray8_to_rgb8(&px))?.into_dyn()))
        }
    } else {
        if format_type == &16 {
            let mut u16_px: Vec<u16> = Vec::with_capacity(&px.len() / 2);
            for dooble_u8 in px.chunks(2) {
                u16_px.push(read_u16(dooble_u8))
            }
            Ok(ArrayDType::U16(Array3::from_shape_vec((height as usize, width as usize, channels), u16_px)?.into_dyn()))
        } else {
            Ok(ArrayDType::U8(Array3::from_shape_vec((height as usize, width as usize, channels), px.into())?.into_dyn()))
        }
    }
}

pub fn psd_rgba2dtype(px: &[u8], channels: usize, format_type: &u8, height: u32, width: u32) -> Result<ArrayDType, Box<dyn Error>> {
    if channels == 1 {
        if format_type == &16 {
            Ok(ArrayDType::U16(Array3::from_shape_vec((height as usize, width as usize, 4), gray16u8_to_rgba16(&px))?.into_dyn()))
        } else {
            Ok(ArrayDType::U8(Array3::from_shape_vec((height as usize, width as usize, 4), gray8_to_rgba8(&px))?.into_dyn()))
        }
    } else {
        if format_type == &16 {
            let mut u16_px: Vec<u16> = vec![];
            for dooble_u8 in px.chunks(6) {
                u16_px.push(read_u16(&dooble_u8[0..2]));
                u16_px.push(read_u16(&dooble_u8[2..4]));
                u16_px.push(read_u16(&dooble_u8[4..6]));
                u16_px.push(u16::MAX)
            }
            Ok(ArrayDType::U16(Array3::from_shape_vec((height as usize, width as usize, 4), u16_px)?.into_dyn()))
        } else {
            let mut u8_px: Vec<u8> = vec![];
            for u8_values in px.chunks(3) {
                u8_px.extend_from_slice(u8_values);
                u8_px.push(u8::MAX)
            }
            Ok(ArrayDType::U8(Array3::from_shape_vec((height as usize, width as usize, 4), u8_px)?.into_dyn()))
        }
    }
}

pub fn psd_luma2dtype(px: &[u8], channels: usize, format_type: &u8, height: u32, width: u32) -> Result<ArrayDType, Box<dyn Error>> {
    if channels == 1 {
        if format_type == &16 {
            let mut u16_px: Vec<u16> = Vec::with_capacity(&px.len() / 2);
            for dooble_u8 in px.chunks(2) {
                u16_px.push(read_u16(dooble_u8))
            }
            Ok(ArrayDType::U16(Array2::from_shape_vec((height as usize, width as usize), u16_px)?.into_dyn()))
        } else {
            Ok(ArrayDType::U8(Array2::from_shape_vec((height as usize, width as usize), px.into())?.into_dyn()))
        }
    } else {
        if format_type == &16 {
            Ok(ArrayDType::U16(Array2::from_shape_vec((height as usize, width as usize), rgb16u8_to_gray16(px))?.into_dyn()))
        } else {
            Ok(ArrayDType::U8(Array2::from_shape_vec((height as usize, width as usize), rgb8_to_gray8(px))?.into_dyn()))
        }
    }
}

pub fn psd_aluma2dtype(px: &[u8], channels: usize, format_type: &u8, height: u32, width: u32) -> Result<ArrayDType, Box<dyn Error>> {
    if channels == 1 {
        if format_type == &16 {
            let mut u16_px: Vec<u16> = Vec::with_capacity(&px.len() / 2);
            for dooble_u8 in px.chunks(2) {
                u16_px.push(read_u16(dooble_u8));
                u16_px.push(u16::MAX)
            }
            Ok(ArrayDType::U16(Array3::from_shape_vec((height as usize, width as usize, 2), u16_px)?.into_dyn()))
        } else {
            let mut au8_vec: Vec<u8> = vec![];
            for u8_value in px {
                au8_vec.push(*u8_value);
                au8_vec.push(u8::MAX)
            }
            Ok(ArrayDType::U8(Array3::from_shape_vec((height as usize, width as usize, 2), au8_vec)?.into_dyn()))
        }
    } else {
        if format_type == &16 {
            Ok(ArrayDType::U16(Array3::from_shape_vec((height as usize, width as usize, 2), rgb16u8_to_agray16(px))?.into_dyn()))
        } else {
            Ok(ArrayDType::U8(Array3::from_shape_vec((height as usize, width as usize, 2), rgb8_to_agray8(px))?.into_dyn()))
        }
    }
}