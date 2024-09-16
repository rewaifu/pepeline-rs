use std::error::Error;
use std::io::Cursor;
use std::path::Path;
use filebuffer::FileBuffer;

use crate::core::universal_functions::enums::{ArrayDType, ImgColor, ImgFormat};
use image;
use zune_psd::PSDDecoder;
use crate::core::read::byte_decode::{read_u32};
use crate::core::read::decode_methods::{dynamic_aluma_img2array_dtype, dynamic_img2array_dtype,
                                        dynamic_luma_img2array_dtype, dynamic_rgb_img2array_dtype,
                                        dynamic_rgba_img2array_dtype, psd_aluma2dtype,
                                        psd_dynamic2dtype, psd_luma2dtype, psd_rgb2dtype,
                                        psd_rgba2dtype};


fn image_decode(bytes: &[u8], color_mode: ImgColor,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>> {
    let bytes_cursor: Cursor<&[u8]> = Cursor::new(&bytes);
    let dynamic_img = image::io::Reader::new(bytes_cursor)
        .with_guessed_format()?
        .decode()?;
    match color_mode {
        ImgColor::DYNAMIC => Ok(dynamic_img2array_dtype(&dynamic_img,color_format)?),
        ImgColor::RGB => Ok(dynamic_rgb_img2array_dtype(&dynamic_img,color_format)?),
        ImgColor::RGBA => Ok(dynamic_rgba_img2array_dtype(&dynamic_img,color_format)?),
        ImgColor::GRAY => Ok(dynamic_luma_img2array_dtype(&dynamic_img,color_format)?),
        ImgColor::AGRAY => Ok(dynamic_aluma_img2array_dtype(&dynamic_img,color_format)?)
    }
}

fn psd_decode(bytes: &[u8], color_mode: ImgColor) -> Result<ArrayDType, Box<dyn Error>> {
    let size_bites: &[u8] = &bytes[14..22];
    let channels = bytes[13] as usize;
    let mut decoder = PSDDecoder::new(bytes);
    let px = decoder.decode_raw().unwrap();
    let (height, width) = (read_u32(&size_bites[0..4]), read_u32(&size_bites[4..8]));
    let format_type = &bytes[23];
    match color_mode {
        ImgColor::DYNAMIC => Ok(psd_dynamic2dtype(&px, channels, format_type, height, width)?),
        ImgColor::RGB => Ok(psd_rgb2dtype(&px, channels, format_type, height, width)?),
        ImgColor::RGBA => Ok(psd_rgba2dtype(&px, channels, format_type, height, width)?),
        ImgColor::GRAY => Ok(psd_luma2dtype(&px, channels, format_type, height, width)?),
        ImgColor::AGRAY => Ok(psd_aluma2dtype(&px, channels, format_type, height, width)?),
    }
}


pub fn decoder(path: &Path, color_mode: ImgColor,color_format:ImgFormat) -> Result<ArrayDType, Box<dyn Error>>

{
    let bytes = FileBuffer::open(path)?.to_vec();
    let bytes_magic_byte = &bytes[..4];
    match bytes_magic_byte {
        [56, 66, 80, 83] => Ok(psd_decode(&bytes, color_mode)?.into()),
        _ => Ok(image_decode(&bytes, color_mode,color_format)?.into()),
    }
    // Ok(image_decode(&bytes, color_mode)?)
}