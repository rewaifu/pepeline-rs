use pyo3::Python;
use std::path::Path;
use image::{ImageBuffer, Luma, RgbImage};
use ndarray::ArrayD;
use numpy::{PyReadonlyArrayDyn, ToPyArray};
use pyo3::{ pyfunction, PyObject, PyResult};
use crate::utils::image::convert::{f32_to_u8, gray_img_open, gray_img_openf32, rgb_img_open, rgb_img_openf32};
use crate::utils::image::decode::{jpg_gray_img_open, jpg_gray_img_openf32, jpg_rgb_img_open, jpg_rgb_img_openf32, psd_din32_decode, psd_din_decode, psd_gray32_decode, psd_gray_decode, psd_rgb32_decode, psd_rgb_decode};
fn save_img_vec(vec_img:&[u8],shape: &[usize],out_path:&Path){
    match shape.len() {
        2 => {
            let img = ImageBuffer::from_fn(shape[1]as u32, shape[0]as u32, move |x, y| Luma([vec_img[(y * shape[1]as u32 + x) as usize]]));
            img.save(Path::new(&out_path)).expect("Not Save");
        }
        3 => {
            match shape[2]{
                3=>{let img =     RgbImage::from_raw(shape[1]as u32, shape[0]as u32, vec_img.to_vec())
                    .expect("container should have the right size for the image dimensions");
                    img.save(Path::new(&out_path)).expect("Not Save");}
                1=>{
                    let img = ImageBuffer::from_fn(shape[1]as u32, shape[0]as u32, move |x, y| Luma([vec_img[(y * shape[1]as u32 + x) as usize]]));
                    img.save(Path::new(&out_path)).expect("Not Save");
                }
                _=>{panic!("color channel error")}
            }}

        _ => {panic!("The array must be 2D or 3D")}
    }

}
#[pyfunction]
pub fn save(
    input:PyObject,
    out_path:String,
    py: Python
)
{

    let vec_img:Vec<u8>;
    let shape:Vec<usize>;
    if let Ok(typee) = input.extract::<PyReadonlyArrayDyn<u8>>(py) {
        let array8 = typee.as_array().to_owned();
        vec_img = array8.clone().into_raw_vec();
        shape = array8.shape().to_vec();
    } else if let Ok(typee) = input.extract::<PyReadonlyArrayDyn<f32>>(py) {
        let arr32 = typee.as_array().to_owned();
        vec_img = f32_to_u8(&arr32.clone().into_raw_vec());
        shape = arr32.shape().to_vec();

    } else {
        panic!("сука")
    }
    save_img_vec(&vec_img,&shape,Path::new(&out_path))
    }


fn all_read_u8(path: &Path, mode: u8, extension: &str) -> ArrayD<u8> {
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

fn all_read_f32(path: &Path, mode: u8, extension: &str) -> ArrayD<f32> {
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

#[pyfunction]
pub fn read(
    path: String,
    mode: Option<u8>,
    format: Option<u8>,
    py: Python) -> PyResult<PyObject> {
    let path = Path::new(&path);
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("error");
    let mode = mode.unwrap_or(2u8);
    let format = format.unwrap_or(1u8);

    match format {0=>{
        let array = all_read_f32(path, mode, extension);
        Ok(array.to_pyarray(py).into())
    } _=> {
        let array = all_read_u8(path, mode, extension);
        Ok(array.to_pyarray(py).into())}}
}