use crate::utils::core::cvt_constants::*;
use crate::utils::core::enums::CvtType;

// float32
const FLOAT_BIAS: f32 = 0.5;

fn gamma_correction(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn rgb_to_luma(rgb: &[f32]) -> f32 {
    let (r, g, b) = (
        gamma_correction(rgb[0]),
        gamma_correction(rgb[1]),
        gamma_correction(rgb[2]),
    );
    let y = r * 0.212671 + g * 0.715160 + b * 0.072169; //XYZ only Y ,
    if y <= (216_f32 / 24389_f32) {
        ((24389_f32 / 27_f32) * y) / 100_f32
    } else {
        (116_f32 * (y.powf(1_f32 / 3_f32)) - 16_f32) / 100_f32
    }
}

fn rgb_to_gray(rgb: &[f32], kr: f32, kg: f32, kb: f32) -> f32 {
    rgb[0] * kr + rgb[1] * kg + rgb[2] * kb
}

fn rgb_to_ycbcr(rgb: &[f32], kr: f32, kg: f32, kb: f32, kd: f32, ke: f32) -> [f32; 3] {
    let y = rgb_to_gray(rgb, kr, kg, kb);
    let cb = ((rgb[2] - y) / kd) + FLOAT_BIAS;
    let cr = ((rgb[0] - y) / ke) + FLOAT_BIAS;
    [y, cb, cr]
}

fn ycbcr_to_rgb(ycbcr: &[f32], kd: f32, ke: f32, crg: f32, cbg: f32) -> [f32; 3] {
    let cb = ycbcr[1] - FLOAT_BIAS;
    let cr = ycbcr[2] - FLOAT_BIAS;
    let y = ycbcr[0];
    let r = (y + ke * cr).max(0f32).min(1f32);
    let g = (y - crg * cr - cbg * cb).max(0f32).min(1f32);
    let b = (y + kd * cb).max(0f32).min(1f32);
    [r, g, b]
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
        CvtType::RGB2GrayBt2020 => {
            rgb2gray(img, |x: &[f32]| rgb_to_gray(x, KR_2020, KG_2020, KB_2020))
        }
        CvtType::RGB2GrayAverage => {
            rgb2gray(img, |x: &[f32]| rgb_to_gray(x, AVERAGE, AVERAGE, AVERAGE))
        }
        CvtType::RGB2GrayBt709 => rgb2gray(img, |x: &[f32]| rgb_to_gray(x, KR_709, KG_709, KB_709)),
        CvtType::RGB2Gray => rgb2gray(img, |x: &[f32]| rgb_to_gray(x, KR_601, KG_601, KB_601)),
        CvtType::RGB2CMYK => rgb2cmyk(img),
        CvtType::CMYK2RGB => cmyk2rgb(img),
        CvtType::RGB2YCbCr => rgb2_3ch(img, |x: &[f32]| {
            rgb_to_ycbcr(x, KR_601, KG_601, KB_601, KD_601, KE_601)
        }),
        CvtType::YCbCr2RGB => rgb2_3ch(img, |x: &[f32]| {
            ycbcr_to_rgb(x, KD_601, KE_601, KCRG_601, KCBG_601)
        }),
        CvtType::RGB2YCvCrBt2020 => rgb2_3ch(img, |x: &[f32]| {
            rgb_to_ycbcr(x, KR_2020, KG_2020, KB_2020, KD_2020, KE_2020)
        }),
        CvtType::YCvCr2RGBBt2020 => rgb2_3ch(img, |x: &[f32]| {
            ycbcr_to_rgb(x, KD_2020, KE_2020, KCRG_2020, KCBG_2020)
        }),
        CvtType::RGB2YCvCrBt709 => rgb2_3ch(img, |x: &[f32]| {
            rgb_to_ycbcr(x, KR_709, KG_709, KB_709, KD_709, KE_709)
        }),
        CvtType::YCvCr2RGBBt709 => rgb2_3ch(img, |x: &[f32]| {
            ycbcr_to_rgb(x, KD_709, KE_709, KCRG_709, KCBG_709)
        }),
        CvtType::RGB2BGR | CvtType::BGR2RGB => rgb2_3ch(img, |x: &[f32]| rgb_to_bgr(x)),
        CvtType::GRAY2RGB => gray2_3ch(img, |x| gray_to_rgb(x)),
        CvtType::RGB2Luma => rgb2gray(img, |x: &[f32]| rgb_to_luma(x)),
    }
}
