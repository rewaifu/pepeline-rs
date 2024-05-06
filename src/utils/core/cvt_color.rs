use crate::utils::core::enums::CvtType;

// float32
fn rgb_to_gray_average(rgb: &[f32]) -> f32 {
    rgb[0] * 0.33 + rgb[1] * 0.34 + rgb[2] * 0.33
}

fn rgb_to_gray_ntsc(rgb: &[f32]) -> f32 {
    rgb[0] * 0.299 + rgb[1] * 0.587 + rgb[2] * 0.114
}

fn rgb_to_gray_bt709(rgb: &[f32]) -> f32 {
    rgb[0] * 0.2126 + rgb[1] * 0.7152 + rgb[2] * 0.0722
}

fn rgb_to_gray_bt2020(rgb: &[f32]) -> f32 {
    rgb[0] * 0.2627 + rgb[1] * 0.6780 + rgb[2] * 0.0593
}

fn cmyk_to_rgb(cmyk: &[f32]) -> [f32; 3] {
    let k = 1.0 - cmyk[3];
    let r = (1.0 - cmyk[0]) * k;
    let g = (1.0 - cmyk[1]) * k;
    let b = (1.0 - cmyk[2]) * k;
    [r, g, b]
}

fn rgb_to_cmyk(rgb: &[f32]) -> [f32; 4] {
    if rgb == [0.0, 0.0, 0.0] {
        return [0_f32, 0_f32, 0_f32, 1_f32];
    }
    let c = 1.0 - rgb[0];
    let m = 1.0 - rgb[1];
    let y = 1.0 - rgb[2];

    // extract out k [0, 1]
    let min_cmy = c.min(m).min(y);
    let c = (c - min_cmy) / (1.0 - min_cmy);
    let m = (m - min_cmy) / (1.0 - min_cmy);
    let y = (y - min_cmy) / (1.0 - min_cmy);
    let k = min_cmy;

    [c, m, y, k]
}

fn rgb_to_ycbcr_bt709(rgb: &[f32]) -> [f32; 3] {
    let y = rgb_to_gray_bt709(rgb);
    let cb = (rgb[2] - y) / 1.8556;
    let cr = (rgb[0] - y) / 1.5748;
    [y, cb, cr]
}

fn ycbcr_bt709_to_rgb(ycbcr: &[f32]) -> [f32; 3] {
    let y = ycbcr[0];
    let r = y + 1.5748 * ycbcr[2];
    let g = y - 0.468 * ycbcr[2] - 0.187 * ycbcr[1];
    let b = y + 1.8556 * ycbcr[1];
    [r, g, b]
}

fn rgb_to_ycbcr_bt601(rgb: &[f32]) -> [f32; 3] {
    let y = rgb_to_gray_ntsc(rgb);
    let cb = (rgb[2] - y) * 0.713;
    let cr = (rgb[0] - y) * 0.564;
    [y, cb, cr]
}

fn ycbcr_bt601_to_rgb(ycbcr: &[f32]) -> [f32; 3] {
    let y = ycbcr[0];
    let r = y + 1.403 * ycbcr[2];
    let g = y - 0.714 * ycbcr[2] - 0.344 * ycbcr[1];
    let b = y + 1.773 * ycbcr[1];
    [r, g, b]
}

fn ycbcr_bt2020_to_rgb(ycbcr: &[f32]) -> [f32; 3] {
    let y = ycbcr[0];
    let r = y + (1.4746) * ycbcr[2];
    let g = y - 0.571 * ycbcr[2] - 0.1645 * ycbcr[1];
    let b = y + 1.8814 * ycbcr[1];
    [r, g, b]
}

fn rgb_to_ycbcr_bt2020(rgb: &[f32]) -> [f32; 3] {
    let y = rgb_to_gray_bt2020(rgb);
    let cb = (rgb[2] - y) / 1.8817;
    let cr = (rgb[0] - y) / 1.4746;
    [y, cb, cr]
}

fn rgb_to_bgr(rgb: &[f32]) -> [f32; 3] {
    [rgb[2], rgb[1], rgb[0]]
}

fn gray_to_rgb(gray: &f32) -> [f32; 3] {
    [*gray, *gray, *gray]
}

pub fn rgb2gray(img: &[f32], func: fn(&[f32]) -> f32) -> Vec<f32> {
    let mut gray_img = Vec::with_capacity(img.len() / 3);
    for rgb in img.chunks(3) {
        gray_img.push(func(rgb))
    }
    gray_img
}

pub fn rgb2cmyk(img: &[f32]) -> Vec<f32> {
    let mut cmyk_img = Vec::with_capacity((img.len() / 3) * 4);
    for rgb in img.chunks(3) {
        cmyk_img.extend(rgb_to_cmyk(rgb))
    }
    cmyk_img
}

pub fn cmyk2rgb(img: &[f32]) -> Vec<f32> {
    let mut rgb_img = Vec::with_capacity((img.len() / 4) * 3);
    for cmyk in img.chunks(4) {
        rgb_img.extend(cmyk_to_rgb(cmyk))
    }
    rgb_img
}

fn rgb2_3ch(img: &[f32], func: fn(&[f32]) -> [f32; 3]) -> Vec<f32> {
    let mut rgb_img = Vec::with_capacity(img.len());
    for cmyk in img.chunks(3) {
        rgb_img.extend(func(cmyk))
    }
    rgb_img
}

fn gray2_3ch(img: &[f32], func: fn(&f32) -> [f32; 3]) -> Vec<f32> {
    let mut rgb_img = Vec::with_capacity(img.len() * 3);
    for gray in img {
        rgb_img.extend(func(gray))
    }
    rgb_img
}

pub fn cvt_color_float(img: &[f32], cvt_type: CvtType) -> Vec<f32> {
    match cvt_type {
        CvtType::RGB2GrayBt2020 => rgb2gray(img, |x: &[f32]| rgb_to_gray_bt2020(x)),
        CvtType::RGB2GrayAverage => rgb2gray(img, |x: &[f32]| rgb_to_gray_average(x)),
        CvtType::RGB2GrayBt709 => rgb2gray(img, |x: &[f32]| rgb_to_gray_bt709(x)),
        CvtType::RGB2Gray => rgb2gray(img, |x: &[f32]| rgb_to_gray_ntsc(x)),
        CvtType::RGB2CMYK => rgb2cmyk(img),
        CvtType::CMYK2RGB => cmyk2rgb(img),
        CvtType::RGB2YCbCr => rgb2_3ch(img, |x: &[f32]| rgb_to_ycbcr_bt601(x)),
        CvtType::YCbCr2RGB => rgb2_3ch(img, |x: &[f32]| ycbcr_bt601_to_rgb(x)),
        CvtType::RGB2YCvCrBt2020 => rgb2_3ch(img, |x: &[f32]| rgb_to_ycbcr_bt2020(x)),
        CvtType::YCvCr2RGBBt2020 => rgb2_3ch(img, |x: &[f32]| ycbcr_bt2020_to_rgb(x)),
        CvtType::RGB2YCvCrBt709 => rgb2_3ch(img, |x: &[f32]| rgb_to_ycbcr_bt709(x)),
        CvtType::YCvCr2RGBBt709 => rgb2_3ch(img, |x: &[f32]| ycbcr_bt709_to_rgb(x)),
        CvtType::RGB2BGR | CvtType::BGR2RGB => rgb2_3ch(img, |x: &[f32]| rgb_to_bgr(x)),
        CvtType::GRAY2RGB => gray2_3ch(img, |x| gray_to_rgb(x))
    }
}
