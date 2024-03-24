use ndarray::Array2;

use crate::utils::screentone::dot::create_dot;

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
