use ndarray::Array2;

use crate::utils::core::enums::TypeDot;
use crate::utils::halftone::dot::create_dot;
use crate::utils::halftone::utils_halftone::{compute_cos_sin, rotate_pixel_coordinates};

pub fn screentone_rotate_add(
    array: &mut Array2<f32>,
    dot_size: usize,
    angle: f32,
    dot_type: TypeDot,
) {
    let (dot, dot_inv) = create_dot(dot_size, dot_type);
    let mut src_values: f32;
    let mut colum: usize;
    let (w, h) = (array.shape()[0], array.shape()[1]);
    let lx_bias = w / 2;
    let ly_bias = h / 2;
    let cos_sin = compute_cos_sin(angle);
    for ly in 0..w {
        let ly2 = ly + ly_bias;
        for lx in 0..h {
            let value = &mut array[[ly, lx]];
            if *value > 0.0 && *value < 1.0 {
                let lx2 = lx + lx_bias;
                let rot = rotate_pixel_coordinates(
                    lx2 as f32, ly2 as f32, w as f32, h as f32, cos_sin[0], cos_sin[1],
                );
                colum = rot.1 / dot_size;
                src_values = if (colum + rot.0 / dot_size) % 2 == 1 {
                    dot_inv[[rot.0 % dot_size, rot.1 % dot_size]]
                } else {
                    dot[[rot.0 % dot_size, rot.1 % dot_size]]
                };
                let src_value = src_values;
                *value = if *value < src_value { 0.0 } else { 1.0 };
            }
        }
    }
}

pub fn screentone_add(array: &mut Array2<f32>, dot_size: usize, dot_type: TypeDot) {
    let (dot, dot_inv) = create_dot(dot_size, dot_type);
    let lx_bias = dot_size / 2;
    let ly_bias = dot_size / 2;
    let mut src_values: f32;
    let mut colum: usize;
    let (w, h) = (array.shape()[0], array.shape()[1]);
    for ly in 0..w {
        let ly2 = ly + ly_bias;
        colum = ly2 / dot_size;
        for lx in 0..h {
            let value = &mut array[[ly, lx]];
            if *value > 0.0 && *value < 1.0 {
                let lx2 = lx + lx_bias;
                src_values = if (colum + lx2 / dot_size) % 2 == 1 {
                    dot_inv[[lx2 % dot_size, ly2 % dot_size]]
                } else {
                    dot[[lx2 % dot_size, ly2 % dot_size]]
                };
                let src_value = src_values;
                *value = if *value < src_value { 0.0 } else { 1.0 };
            }
        }
    }
}
