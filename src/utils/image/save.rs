use std::error::Error;
use std::path::Path;

use image::{ImageBuffer, Luma, RgbImage};

pub fn save_img_vec(
    vec_img: &[u8],
    shape: &[usize],
    out_path: &Path,
) -> Result<(), Box<dyn Error>> {
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
