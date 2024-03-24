use image::{ImageBuffer, Luma, RgbImage};
use std::path::Path;

pub fn save_img_vec(vec_img: &[u8], shape: &[usize], out_path: &Path) {
    match shape.len() {
        2 => {
            let img = ImageBuffer::from_fn(shape[1] as u32, shape[0] as u32, move |x, y| {
                Luma([vec_img[(y * shape[1] as u32 + x) as usize]])
            });
            img.save(Path::new(&out_path)).expect("Not Save");
        }
        3 => match shape[2] {
            3 => {
                let img = RgbImage::from_raw(shape[1] as u32, shape[0] as u32, vec_img.to_vec())
                    .expect("container should have the right size for the image dimensions");
                img.save(Path::new(&out_path)).expect("Not Save");
            }
            1 => {
                let img = ImageBuffer::from_fn(shape[1] as u32, shape[0] as u32, move |x, y| {
                    Luma([vec_img[(y * shape[1] as u32 + x) as usize]])
                });
                img.save(Path::new(&out_path)).expect("Not Save");
            }
            _ => {
                panic!("color channel error")
            }
        },

        _ => {
            panic!("The array must be 2D or 3D")
        }
    }
}
