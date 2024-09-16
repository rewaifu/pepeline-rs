use std::error::Error;
use std::path::Path;
use image::{ImageBuffer, Luma, LumaA, Rgb, RgbImage, Rgba, RgbaImage};
use crate::core::universal_functions::enums::ArrayDType;
use crate::core::universal_functions::format_convert::f32_to_u8;

fn vec_shape_save(vec_img:&[u8],shape:Vec<usize>,out_path: &Path)-> Result<(), Box<dyn Error>>{
    println!("{:?}",shape);
    // println!()
    match shape.len() {
        2 => {
            let img = ImageBuffer::from_fn(shape[1] as u32, shape[0] as u32, move |x, y| {
                Luma([vec_img[(y * shape[1] as u32 + x) as usize]])
            });
            img.save(Path::new(&out_path))?;
        }
        3 => match shape[2] {
            3 => {
                let img = RgbImage::from_raw(shape[1] as u32, shape[0] as u32, vec_img.to_vec())
                    .ok_or("container should have the right size for the image dimensions")?;
                img.save(Path::new(&out_path))?;
            }
            4 =>{
                let img = RgbaImage::from_raw(shape[1] as u32, shape[0] as u32, vec_img.to_vec())
                    .ok_or("container should have the right size for the image dimensions")?;
                img.save(Path::new(&out_path))?;
            }
            2=>{
                let img: ImageBuffer<LumaA<u8>, Vec<u8>> = ImageBuffer::from_raw(shape[1] as u32, shape[0] as u32, vec_img.to_vec())
                    .ok_or("container should have the right size for the image dimensions")?;
                img.save(Path::new(&out_path))?;
            }
            1 => {
                let img = ImageBuffer::from_fn(shape[1] as u32, shape[0] as u32, move |x, y| {
                    Luma([vec_img[(y * shape[1] as u32 + x) as usize]])
                });
                img.save(Path::new(&out_path))?;
            }
            _ => {
                return Err("color channel error".into());
            }
        },

        _ => {
            return Err("The array must be 2D or 3D".into());
        }
    }
    Ok(())
}
fn vec_shape_saveu16(vec_img:&[u16],shape:Vec<usize>,out_path: &Path)-> Result<(), Box<dyn Error>>{
    match shape.len() {
        2 => {
            let img = ImageBuffer::from_fn(shape[1] as u32, shape[0] as u32, move |x, y| {
                Luma([vec_img[(y * shape[1] as u32 + x) as usize]])
            });
            img.save(Path::new(&out_path))?;
        }
        3 => match shape[2] {
            3 => {
                let img: ImageBuffer<Rgb<u16>, Vec<u16>> = ImageBuffer::from_raw(shape[1] as u32, shape[0] as u32, vec_img.to_vec())
                    .ok_or("container should have the right size for the image dimensions")?;
                img.save(Path::new(&out_path))?;
            }
            4 =>{
                let img: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::from_raw(shape[1] as u32, shape[0] as u32, vec_img.to_vec())
                    .ok_or("container should have the right size for the image dimensions")?;
                img.save(Path::new(&out_path))?;
            }
            2=>{
                let img: ImageBuffer<LumaA<u16>, Vec<u16>> = ImageBuffer::from_raw(shape[1] as u32, shape[0] as u32, vec_img.to_vec())
                    .ok_or("container should have the right size for the image dimensions")?;
                img.save(Path::new(&out_path))?;
            }
            1 => {
                let img = ImageBuffer::from_fn(shape[1] as u32, shape[0] as u32, move |x, y| {
                    Luma([vec_img[(y * shape[1] as u32 + x) as usize]])
                });
                img.save(Path::new(&out_path))?;
            }
            _ => {
                return Err("color channel error".into());
            }
        },

        _ => {
            return Err("The array must be 2D or 3D".into());
        }
    }
    Ok(())
}
pub fn save_img_ndarray(
    vec_img: ArrayDType,
    out_path: &Path,
) -> Result<(), Box<dyn Error>> {
    println!("{:?}",vec_img);
    match vec_img {
        ArrayDType::F32(value)=>{
            let img_shape = value.shape().to_vec();
            let vec = value.into_raw_vec();
            Ok(vec_shape_save(&f32_to_u8(&vec), img_shape, out_path)?)
        }
        ArrayDType::U8(value)=>{
            let img_shape = value.shape().to_vec();
            let vec = value.into_raw_vec();
            Ok(vec_shape_save(&vec, img_shape, out_path)?)
        }
        ArrayDType::U16(value)=>{
            let img_shape = value.shape().to_vec();
            let vec = value.into_raw_vec();
            Ok(vec_shape_saveu16(&vec, img_shape, out_path)?)
        }
    }
    
}
