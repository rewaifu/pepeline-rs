use ndarray::{arr1, arr2, Array1, Array2};
use num::abs;

use crate::utils::screentone::dot::create_dot;

fn rotate_pixel_coordinates(
    x: f32,
    y: f32,
    angle: f32,
    center_x: f32,
    center_y: f32,
) -> (usize, usize) {
    let theta = angle;

    let rotation_matrix: Array2<f32> =
        arr2(&[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]]);

    let x_rel = x - center_x;
    let y_rel = y - center_y;

    let rotated_point: Array1<f32> = rotation_matrix.dot(&arr1(&[x_rel, y_rel]));

    let rotated_x = abs(rotated_point[0] + center_x) as usize;
    let rotated_y = abs(rotated_point[1] + center_y) as usize;

    (rotated_x, rotated_y)
}

pub fn screentone_rotate_add(array: &mut Array2<f32>, dot_size: usize, angle: f32) {
    let (dot, dot_inv) = create_dot(dot_size);
    let mut src_values: f32;
    let mut colum: usize;
    let (w, h) = (array.shape()[0], array.shape()[1]);
    let lx_plus = w / 2;
    let ly_plus = h / 2;
    let ww = 0..w;
    let hh = 0..h;
    for ly in ww {
        let ly2 = ly + ly_plus;

        for lx in hh.clone() {
            let value = &mut array[[ly, lx]];
            if *value > 0.0 && *value < 1.0 {
                let lx2 = lx + lx_plus;
                let rot =
                    rotate_pixel_coordinates(lx2 as f32, ly2 as f32, angle, w as f32, h as f32);
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

pub fn screentone_add(array: &mut Array2<f32>, dot_size: usize, ly_plus: usize, lx_plus: usize) {
    let (dot, dot_inv) = create_dot(dot_size);
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
