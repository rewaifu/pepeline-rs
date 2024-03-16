use std::path::Path;
use ndarray::{Array2, Array3};

pub fn gray_img_open(path:&Path)->Array2<u8>{
    let img = image::open(path).unwrap().into_luma8();
    let (width, height) = img.dimensions();
    let input = img.into_raw();
    Array2::from_shape_vec((height as usize, width as usize), input).unwrap()
}
pub fn rgb_img_open(path:&Path)->Array3<u8>{
    let img = image::open(path).unwrap().into_rgb8();
    let (width, height) = img.dimensions();
    let input= img.into_raw();
    Array3::from_shape_vec((height as usize, width as usize, 3), input).unwrap()

}
