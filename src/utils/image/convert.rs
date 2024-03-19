use std::path::Path;
use image::{GrayImage, ImageBuffer, Luma, Rgb, RgbImage};
use ndarray::{Array2, Array3, ArrayD};
pub(crate)fn rgb8_to_gray8(rgb: (u8, u8, u8)) -> u8 {
    let (r, g, b) = rgb;
    let gray = (r as f32 * 0.114 + g as f32 * 0.587 + b as f32 * 0.299)as u8 ;
    gray
}
pub(crate)fn rgb8_to_gray32(rgb: (u8, u8, u8)) -> f32 {
    let (r, g, b) = rgb;
    let gray = (r as f32 * 0.114 + g as f32 * 0.587 + b as f32 * 0.299)/255.0;
    gray
}
pub(crate) fn u8_to_f32(bytes: &[u8]) -> Vec<f32> {
    let mut floats = vec![0.0; bytes.len()];
    floats.iter_mut().zip(bytes.iter()).for_each(|(f, &b)| *f = if b == 0 { b as f32 } else { b as f32 / 255.0 });

    floats
}

pub(crate)fn f32_to_u8(bytes: &[f32]) -> Vec<u8> {
    // Создаем вектор для хранения результата с заранее известной ёмкостью
    let mut floats = vec![0; bytes.len()];
    floats.iter_mut().zip(bytes.iter()).for_each(|(f, &b)| *f = if b == 0.0 {b as u8} else { (b* 255.0) as u8 });
    // Выполняем преобразование, нормализуя значения в диапазоне от 0 до 1
    floats.extend(bytes.iter().map(|&byte| (byte * 255.0) as u8));

    floats
}

pub(crate)fn array_gray2image(array:ArrayD<u8>,shape:&[usize])->ImageBuffer<Luma<u8>,Vec<u8>>{
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
pub(crate)fn array_rgb2image(array: ArrayD<u8>,shape:&[usize]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {

    let (w, h) = (shape[1] as u32, shape[0] as u32);
    let raw = array.into_raw_vec();

    RgbImage::from_raw(w , h, raw)
        .expect("container should have the right size for the image dimensions")


}
pub(crate)fn array_grayf32_to_image(array:ArrayD<f32>,shape:&[usize])->ImageBuffer<Luma<u8>,Vec<u8>>{
    let vec: Vec<f32>=array.into_raw_vec();
    let vec_u8:Vec<u8>=f32_to_u8(&vec);
    let (w, h) = (shape[1] as u32, shape[0] as u32);
    ImageBuffer::from_fn(w, h, move |x, y| Luma([vec_u8[(y * w + x) as usize]]))
}
pub(crate)fn array_rgbf32_to_image(array: ArrayD<f32>,shape:&[usize]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {

    let (w, h) = (shape[1] as u32, shape[0] as u32);
    let raw = array.into_raw_vec();
    let raw = f32_to_u8(&raw);

    RgbImage::from_raw(w , h, raw)
        .expect("container should have the right size for the image dimensions")


}



pub(crate)fn rgb2arrayf32(img: RgbImage) -> Array3<f32> {
    let (width, height) = img.dimensions();
    let input = img.into_raw();
    let input_f32 = u8_to_f32(&input);

    Array3::from_shape_vec((height as usize, width as usize, 3), input_f32).unwrap()

}
pub(crate)fn luma2arrayf32(img:GrayImage)->Array2<f32>{
    let (width, height) = img.dimensions();
    let input = img.into_raw();
    let input_f32 = u8_to_f32(&input);
    Array2::from_shape_vec((height as usize, width as usize), input_f32).unwrap()

}
pub(crate)fn luma2array(img:GrayImage)->Array2<u8>{
    let (width, height) = img.dimensions();
    let input = img.into_raw();
    Array2::from_shape_vec((height as usize, width as usize), input).unwrap()

}
pub(crate)fn rgb2array(img:RgbImage)->Array3<u8>{
    let (width, height) = img.dimensions();
    let input= img.into_raw();
    Array3::from_shape_vec((height as usize, width as usize, 3), input).unwrap()
}

pub(crate)fn gray_img_open(path:&Path)->Array2<u8>{
    let img = image::open(path).unwrap().into_luma8();
    luma2array(img)
}

pub(crate)fn rgb_img_open(path:&Path)->Array3<u8>{
    let img = image::open(path).unwrap().into_rgb8();
    rgb2array(img)
}
pub(crate)fn rgb_img_openf32(path:&Path)->Array3<f32>{
    let img = image::open(path).unwrap().into_rgb8();
    rgb2arrayf32(img)
}
pub(crate)fn gray_img_openf32(path:&Path)->Array2<f32>{
    let img = image::open(path).unwrap().into_luma8();
    luma2arrayf32(img)
}

