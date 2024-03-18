use std::path::Path;
use image::{GrayImage, ImageBuffer, Luma, Rgb, RgbImage};
use ndarray::{Array2, Array3, ArrayD, Axis, IxDyn};
use numpy::{PyArray, PyReadonlyArrayDyn, ToPyArray};
use pyo3::{Py, pyfunction, PyResult, Python};
fn u8_to_f32(bytes: &[u8]) -> Vec<f32> {
    // Создаем вектор для хранения результата с заранее известной ёмкостью
    let mut floats = Vec::with_capacity(bytes.len());

    // Выполняем преобразование, нормализуя значения в диапазоне от 0 до 1
    floats.extend(bytes.iter().map(|&byte| byte as f32 / 255.0));

    floats
}
fn f32_to_u8(bytes: &[f32]) -> Vec<u8> {
    // Создаем вектор для хранения результата с заранее известной ёмкостью
    let mut floats = Vec::with_capacity(bytes.len());

    // Выполняем преобразование, нормализуя значения в диапазоне от 0 до 1
    floats.extend(bytes.iter().map(|&byte| (byte * 255.0) as u8));

    floats
}

fn array_gray2image(array:ArrayD<u8>,shape:&[usize])->ImageBuffer<Luma<u8>,Vec<u8>>{
    let array2: Array2<u8> = array.into_dimensionality().unwrap();
    let (w, h) = (shape[1] as u32, shape[0] as u32);
    ImageBuffer::from_fn(
        w,
        h,
        |x, y| {
            let value = array2[[y as usize, x as usize]];
            Luma([value])
        })

}
fn array_rgb2image(array: ArrayD<u8>,shape:&[usize]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {

    let (w, h) = (shape[1] as u32, shape[0] as u32);
    let raw = array.into_raw_vec();

    RgbImage::from_raw(w , h, raw)
        .expect("container should have the right size for the image dimensions")


}
fn array_grayf32_to_image(array:ArrayD<f32>,shape:&[usize])->ImageBuffer<Luma<u8>,Vec<u8>>{
    let vec: Vec<f32>=array.into_raw_vec();
    let vec_u8:Vec<u8>=f32_to_u8(&vec);
    let (w, h) = (shape[1] as u32, shape[0] as u32);
    ImageBuffer::from_fn(w, h, move |x, y| Luma([vec_u8[(y * w + x) as usize]]))
}
fn array_rgbf32_to_image(array: ArrayD<f32>,shape:&[usize]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {

    let (w, h) = (shape[1] as u32, shape[0] as u32);
    let raw = array.into_raw_vec();
    let raw = f32_to_u8(&raw);

    RgbImage::from_raw(w , h, raw)
        .expect("container should have the right size for the image dimensions")


}



fn rgb2arrayf32(img: RgbImage) -> Array3<f32> {
    let (width, height) = img.dimensions();
    let input = img.into_raw();
    let input_f32 = u8_to_f32(&input);

    Array3::from_shape_vec((height as usize, width as usize, 3), input_f32).unwrap()

}
fn luma2arrayf32(img:GrayImage)->Array2<f32>{
    let (width, height) = img.dimensions();
    let input = img.into_raw();
    let input_f32 = u8_to_f32(&input);
    Array2::from_shape_vec((height as usize, width as usize), input_f32).unwrap()

}
fn luma2array(img:GrayImage)->Array2<u8>{
    let (width, height) = img.dimensions();
    let input = img.into_raw();
    Array2::from_shape_vec((height as usize, width as usize), input).unwrap()

}
fn rgb2array(img:RgbImage)->Array3<u8>{
    let (width, height) = img.dimensions();
    let input= img.into_raw();
    Array3::from_shape_vec((height as usize, width as usize, 3), input).unwrap()
}
// fn luma162array(img:ImageBuffer<Luma<u16>, Vec<u16>>)->Array2<u16>{
//     let (width, height) = img.dimensions();
//     let input = img.into_raw();
//     Array2::from_shape_vec((height as usize, width as usize), input).unwrap()
//
// }
// fn rgb162array(img:ImageBuffer<Rgb<u16>, Vec<u16>>)->Array3<u16>{
//     let (width, height) = img.dimensions();
//     let input= img.into_raw();
//     Array3::from_shape_vec((height as usize, width as usize, 3), input).unwrap()
// } rgb and gray u16
fn gray_img_open(path:&Path)->Array2<u8>{
    let img = image::open(path).unwrap().into_luma8();
    luma2array(img)
}
fn rgb_img_open(path:&Path)->Array3<u8>{
    let img = image::open(path).unwrap().into_rgb8();
    rgb2array(img)
}
fn rgb_img_openf32(path:&Path)->Array3<f32>{
    let img = image::open(path).unwrap().into_rgb8();
    rgb2arrayf32(img)
}
fn luma_img_openf32(path:&Path)->Array2<f32>{
    let img = image::open(path).unwrap().into_luma8();
    luma2arrayf32(img)
}


#[pyfunction]
pub fn save(
    input: PyReadonlyArrayDyn<u8>,
    out_path:String)
{
    // Saves images to the selected path.

    let array = input.as_array().to_owned();
    let shape = array.shape();
    match shape.len() {
        2 => {
            let img = array_gray2image(array.clone(),&shape);
            img.save(Path::new(&out_path)).expect("Not Save");
        }
        3 => {
            match shape[2]{
                3=>{let img = array_rgb2image(array.clone(),&shape);
                img.save(Path::new(&out_path)).expect("Not Save");}
                1=>{
                    let array = array.clone().remove_axis(Axis(2)).into_owned();
                let img = array_gray2image(array,&shape);
                img.save(Path::new(&out_path)).expect("Not Save");
                }
                _=>{panic!("color channel error")}
        }}

        _ => {panic!("The array must be 2D or 3D")}
    }

}
#[pyfunction]
pub fn save32(
    input: PyReadonlyArrayDyn<f32>,
    out_path:String)
{
    // Saves images to the selected path.

    let array = input.as_array().to_owned();
    let shape = array.shape();
    match shape.len() {
        2 => {
            let img = array_grayf32_to_image(array.clone(),&shape);
            img.save(Path::new(&out_path)).expect("Not Save");
        }
        3 => {
            match shape[2]{
                3=>{let img = array_rgbf32_to_image(array.clone(),&shape);
                    img.save(Path::new(&out_path)).expect("Not Save");}
                1=>{
                    let array = array.clone().remove_axis(Axis(2)).into_owned();
                    let img = array_grayf32_to_image(array.clone(),&shape);
                    img.save(Path::new(&out_path)).expect("Not Save");
                }
                _=>{panic!("color channel error")}
            }}

        _ => {panic!("The array must be 2D or 3D")}
    }

}
#[pyfunction]
pub fn read<'py>(path: String, mode: Option<u8>, py: Python) -> PyResult<Py<PyArray<u8, IxDyn>>> {
    // reads the image using one of the modes, currently 0-gray8 mode=>1 rgb8
    let mode = mode.unwrap_or(1u8);
    let array = match mode {
        0 => gray_img_open(Path::new(&path)).into_dyn(),
        _ => rgb_img_open(Path::new(&path)).into_dyn(),
    };
    Ok(array.to_pyarray(py).into())
}
#[pyfunction]
pub fn read32<'py>(path: String, mode: Option<u8>, py: Python) -> PyResult<Py<PyArray<f32, IxDyn>>> {
    // reads the image using one of the modes, currently 0-gray8 mode=>1 rgb8
    let mode = mode.unwrap_or(1u8);
    let array = match mode {
        0 => luma_img_openf32(Path::new(&path)).into_dyn(),
        _ => rgb_img_openf32(Path::new(&path)).into_dyn(),
    };
    Ok(array.to_pyarray(py).into())
}
