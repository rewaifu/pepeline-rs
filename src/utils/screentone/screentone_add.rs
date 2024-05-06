use ndarray::Array2;

use crate::utils::screentone::dot::{create_dot, TypeDot};

fn rotate_pixel_coordinates(
    x: f32,
    y: f32,
    center_x: f32,
    center_y: f32,
    cos_theta: f32,
    sin_theta: f32,
) -> (usize, usize) {
    let x_rel = x - center_x;
    let y_rel = y - center_y;

    let rotated_x = (cos_theta * x_rel - sin_theta * y_rel + center_x) as usize;
    let rotated_y = (sin_theta * x_rel + cos_theta * y_rel + center_y) as usize;

    (rotated_x, rotated_y)
}

fn compute_cos_sin(theta: f32) -> (f32, f32) {
    let cos_theta = theta.cos();
    let sin_theta = theta.sin();
    (cos_theta, sin_theta)
}

pub fn screentone_rotate_add(array: &mut Array2<f32>, dot_size: usize, angle: f32, dot_type: TypeDot) {
    let (dot, dot_inv) = create_dot(dot_size, dot_type);
    let mut src_values: f32;
    let mut colum: usize;
    let (w, h) = (array.shape()[0], array.shape()[1]);
    let lx_plus = w / 2;
    let ly_plus = h / 2;
    let cos_sin = compute_cos_sin(angle);
    let ww = 0..w;
    let hh = 0..h;
    for ly in ww {
        let ly2 = ly + ly_plus;

        for lx in hh.clone() {
            let value = &mut array[[ly, lx]];
            if *value > 0.0 && *value < 1.0 {
                let lx2 = lx + lx_plus;
                let rot = rotate_pixel_coordinates(
                    lx2 as f32, ly2 as f32, w as f32, h as f32, cos_sin.0, cos_sin.1,
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

pub fn screentone_add(array: &mut Array2<f32>, dot_size: usize, ly_plus: usize, lx_plus: usize, dot_type: TypeDot) {
    let (dot, dot_inv) = create_dot(dot_size, dot_type);
    let mut src_values: f32;
    let mut colum: usize;
    let (w, h) = (array.shape()[0], array.shape()[1]);
    let ww = 0..w;
    let hh = 0..h;
    for ly in ww {
        let ly2 = ly + ly_plus;
        colum = ly2 / dot_size;
        for lx in hh.clone() {
            let value = &mut array[[ly, lx]];
            if *value > 0.0 && *value < 1.0 {
                let lx2 = lx + lx_plus;
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
