use std::fs::read;
use std::path::Path;
use ndarray::{Array2, Array3};
use zune_jpeg::JpegDecoder;
use zune_jpeg::zune_core::colorspace::ColorSpace;
use zune_jpeg::zune_core::options::DecoderOptions;
use zune_psd::PSDDecoder;
use crate::utils::image::convert::{rgb8_to_gray32, rgb8_to_gray8, u8_to_f32};

pub(crate)fn jpg_gray_img_open(path:&Path) ->Array2<u8>{
    let file_contents = std::fs::read(path).unwrap();
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::Luma);
    let mut decoder = JpegDecoder::new_with_options(&file_contents,options);
    decoder.decode_headers().unwrap();
    let image_info = decoder.info().unwrap();
    let pixels = decoder.decode().unwrap();
    Array2::from_shape_vec((image_info.height as usize, image_info.width as usize),pixels).unwrap()
}
pub(crate)fn jpg_rgb_img_open(path:&Path)->Array3<u8>{
    let file_contents = std::fs::read(path).unwrap();
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
    let mut decoder = JpegDecoder::new_with_options(&file_contents,options);
    decoder.decode_headers().unwrap();
    let image_info = decoder.info().unwrap();
    let pixels = decoder.decode().unwrap();
    Array3::from_shape_vec((image_info.height as usize, image_info.width as usize,3),pixels).unwrap()
}
pub(crate)fn jpg_gray_img_openf32(path:&Path)->Array2<f32>{
    let file_contents = std::fs::read(path).unwrap();
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::Luma);
    let mut decoder = JpegDecoder::new_with_options(&file_contents,options);
    decoder.decode_headers().unwrap();
    let image_info = decoder.info().unwrap();
    let pixels = decoder.decode().unwrap();
    let pixels = crate::utils::image::convert::u8_to_f32(&pixels);
    Array2::from_shape_vec((image_info.height as usize, image_info.width as usize),pixels).unwrap()
}
pub(crate)fn jpg_rgb_img_openf32(path:&Path)->Array3<f32>{
    let file_contents = std::fs::read(path).unwrap();
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
    let mut decoder = JpegDecoder::new_with_options(&file_contents,options);
    decoder.decode_headers().unwrap();
    let image_info = decoder.info().unwrap();
    let pixels = decoder.decode().unwrap();
    let pixels = crate::utils::image::convert::u8_to_f32(&pixels);
    Array3::from_shape_vec((image_info.height as usize, image_info.width as usize,3),pixels).unwrap()
}
fn decode_size_psd(bytes: &[u8]) ->(u32,u32){
    let mut height :u32= 0;
    let mut width :u32= 0;
    height+= bytes[3] as u32;
    height+=if bytes[2]>0{bytes[2]as u32*256}else { 0 };
    height+=if bytes[1]>0{bytes[1]as u32*256*256}else { 0 };
    width+= bytes[7] as u32;
    width+=if bytes[6]>0{bytes[6]as u32*256}else { 0 };
    width+=if bytes[5]>0{bytes[5]as u32*256*256}else { 0 };
    (height,width)

}
pub (crate)fn psd_gray_decode(path: &Path)->Array2<u8>{
    let img = read(path).unwrap();
    let size_bites : &[u8] = &img[14..22];
    let color_mode  = img[25];
    let mut decoder = PSDDecoder::new(&img);
    let px = decoder.decode_raw().unwrap();
    let (height,width) = decode_size_psd(size_bites);
    if color_mode == 1 {
        Array2::from_shape_vec((height as usize, width as usize), px).unwrap()
    }
    else {
        let mut gray_values = Vec::with_capacity(px.len() / 3);
        for chunk in px.chunks(3) {
            let rgb = (chunk[0], chunk[1], chunk[2]);
            let gray = rgb8_to_gray8(rgb);
            gray_values.push(gray);
        }
        Array2::from_shape_vec((height as usize, width as usize), gray_values).unwrap()
    }
}
pub (crate)fn psd_rgb_decode(path: &Path)->Array3<u8>{
    let img = read(path).unwrap();
    let size_bites : &[u8] = &img[14..22];
    let color_mode  = img[25];
    let mut decoder = PSDDecoder::new(&img);
    let px = decoder.decode_raw().unwrap();
    let (height,width) = decode_size_psd(size_bites);
    if color_mode == 1 {
        let mut rgb_values = Vec::with_capacity(px.len() * 3);

        for gray in &px {
            rgb_values.push(*gray);
            rgb_values.push(*gray);
            rgb_values.push(*gray);
        }
        Array3::from_shape_vec((height as usize, width as usize,3),rgb_values).unwrap()
    }
    else {

        Array3::from_shape_vec((height as usize, width as usize,3),px).unwrap()
    }
}
pub (crate)fn psd_gray32_decode(path: &Path)->Array2<f32>{
    let img = read(path).unwrap();
    let size_bites : &[u8] = &img[14..22];
    let color_mode  = img[25];
    let mut decoder = PSDDecoder::new(&img);
    let px = decoder.decode_raw().unwrap();
    let (height,width) = decode_size_psd(size_bites);
    if color_mode == 1 {
        let px = u8_to_f32(&px);
        Array2::from_shape_vec((height as usize, width as usize), px).unwrap()
    }
    else {
        let mut gray_values = Vec::with_capacity(px.len() / 3);
        for chunk in px.chunks(3) {
            let rgb = (chunk[0], chunk[1], chunk[2]);
            let gray = rgb8_to_gray32(rgb);
            gray_values.push(gray);
        }
        Array2::from_shape_vec((height as usize, width as usize), gray_values).unwrap()
    }
}
pub (crate)fn psd_rgb32_decode(path: &Path)->Array3<f32>{
    let img = read(path).unwrap();
    let size_bites : &[u8] = &img[14..22];
    let color_mode  = img[25];
    let mut decoder = PSDDecoder::new(&img);
    let px = decoder.decode_raw().unwrap();
    let (height,width) = decode_size_psd(size_bites);
    if color_mode == 1 {
        let mut rgb_values:Vec<f32> = Vec::with_capacity(px.len() * 3);

        for gray in &px {
            let gray = *gray as f32/255.0;
            rgb_values.push(gray );
            rgb_values.push(gray );
            rgb_values.push(gray );
        }
        Array3::from_shape_vec((height as usize, width as usize,3),rgb_values).unwrap()
    }
    else {
        let px = u8_to_f32(&px);
        Array3::from_shape_vec((height as usize, width as usize,3),px).unwrap()
    }
}