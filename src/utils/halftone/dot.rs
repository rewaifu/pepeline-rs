use ndarray::Array2;

use crate::utils::core::enums::TypeDot;

const X: f32 = 0.1;
const Y: f32 = 0.15;

//refactor
fn math(dot_size: usize) -> (f32, (f32, f32)) {
    let point = (dot_size as f32 / 2.0 + X, dot_size as f32 / 2.0 + Y);
    let step = (1.0 - 0.5) / ((dot_size as f32).powi(2) - 1.0);
    return (step, point);
}

fn line(x: f32, y: f32, h: f32) -> bool {
    let g = x + h;
    let gg = x - h;
    let mut jj = false;
    if gg < y {
        jj = g > y
    }
    return jj;
}

fn invline(x: f32, y: f32, h: f32) -> bool {
    let g = -x + h;
    let gg = -x - h;
    let mut jj = false;
    if gg < y {
        jj = g > y
    }
    return jj;
}

fn ellipse(x: f32, y: f32, h: f32) -> bool {
    let fi = 60.0_f32.to_radians();
    let cos_fi = fi.cos();
    let sin_fi = fi.sin();

    x * x + y * y - 2.0 * x * y * cos_fi < sin_fi * sin_fi * h * h
}

fn cross(x: f32, y: f32, b: f32) -> bool {
    let h = x + b;
    let g = x - b;
    let hh = -x + b;
    let gg = -x - b;
    let c = y > g;
    let hu = y > gg;
    let mut jj = false;

    if hu {
        jj = hh > y;
    }
    if c && !jj {
        jj = h > y;
    }
    jj
}

fn dot_line_inv(dot_size: usize) -> Array2<f32> {
    let mut coordinates_and_values: Vec<(usize, usize, f32)> = vec![];
    let mut dot: Array2<f32> = Array2::zeros((dot_size, dot_size));
    let step = (1.0 - 0.5) / ((dot_size as f32).powi(2) - 1.0);
    for ii in 0..dot_size * 2 {
        for i in 0..dot_size {
            for b in 0..dot_size {
                let value = line(
                    i as f32 - (dot_size / 2) as f32,
                    b as f32 - (dot_size / 2) as f32,
                    ii as f32 + 1.0,
                );
                if !value {
                    dot[[i, b]] += 1.0
                }
            }
        }
    }
    for i in 0..dot_size {
        for b in 0..dot_size {
            let value = dot[[i, b]];
            coordinates_and_values.push((i, b, value))
        }
    }
    coordinates_and_values
        .sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let mut n = 0;

    for &(i, j, _) in &coordinates_and_values {
        let s = 0.5 - (step * n as f32);
        dot[[i, j]] = s;
        n += 1;
    }
    return dot;
}

fn dot_invline_inv(dot_size: usize) -> Array2<f32> {
    let mut coordinates_and_values: Vec<(usize, usize, f32)> = vec![];
    let mut dot: Array2<f32> = Array2::zeros((dot_size, dot_size));
    let step = (1.0 - 0.5) / ((dot_size as f32).powi(2) - 1.0);
    for ii in 0..dot_size * 2 {
        for i in 0..dot_size {
            for b in 0..dot_size {
                let value = invline(
                    i as f32 - (dot_size / 2) as f32,
                    b as f32 - (dot_size / 2) as f32,
                    ii as f32 + 1.0,
                );
                if !value {
                    dot[[i, b]] += 1.0
                }
            }
        }
    }
    for i in 0..dot_size {
        for b in 0..dot_size {
            let value = dot[[i, b]];
            coordinates_and_values.push((i, b, value))
        }
    }
    coordinates_and_values
        .sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let mut n = 0;

    for &(i, j, _) in &coordinates_and_values {
        let s = 0.5 - (step * n as f32);
        dot[[i, j]] = s;
        n += 1;
    }
    return dot;
}

fn dot_ellipse_inv(dot_size: usize) -> Array2<f32> {
    let mut coordinates_and_values: Vec<(usize, usize, f32)> = vec![];
    let mut dot: Array2<f32> = Array2::zeros((dot_size, dot_size));
    let step = (1.0 - 0.5) / ((dot_size as f32).powi(2) - 1.0);
    for ii in 0..dot_size * 2 {
        for i in 0..dot_size {
            for b in 0..dot_size {
                let value = ellipse(
                    i as f32 - (dot_size / 2) as f32,
                    b as f32 - (dot_size / 2) as f32,
                    ii as f32 + 1.0,
                );
                if !value {
                    dot[[i, b]] += 1.0
                }
            }
        }
    }
    for i in 0..dot_size {
        for b in 0..dot_size {
            let value = dot[[i, b]];
            coordinates_and_values.push((i, b, value))
        }
    }
    coordinates_and_values
        .sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let mut n = 0;

    for &(i, j, _) in &coordinates_and_values {
        let s = 0.5 - (step * n as f32);
        dot[[i, j]] = s;
        n += 1;
    }
    return dot;
}

fn dot_cross_inv(dot_size: usize) -> Array2<f32> {
    let mut coordinates_and_values: Vec<(usize, usize, f32)> = vec![];
    let mut dot: Array2<f32> = Array2::zeros((dot_size, dot_size));
    let step = (1.0 - 0.5) / ((dot_size as f32).powi(2) - 1.0);
    for ii in 0..dot_size {
        for i in 0..dot_size {
            for b in 0..dot_size {
                let value = cross(
                    i as f32 - (dot_size / 2) as f32,
                    b as f32 - (dot_size / 2) as f32,
                    ii as f32 + 1.0,
                );
                if !value {
                    dot[[i, b]] += 1.0
                }
            }
        }
    }
    for i in 0..dot_size {
        for b in 0..dot_size {
            let value = dot[[i, b]];
            coordinates_and_values.push((i, b, value))
        }
    }
    coordinates_and_values
        .sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let mut n = 0;

    for &(i, j, _) in &coordinates_and_values {
        let s = 0.5 - (step * n as f32);
        dot[[i, j]] = s;
        n += 1;
    }
    return dot;
}

fn dot_circle_inv(dot_size: usize) -> Array2<f32> {
    let mut coordinates_and_values: Vec<(usize, usize, f32)> = vec![];
    let mut dot: Array2<f32> = Array2::zeros((dot_size, dot_size));
    let (step, point) = math(dot_size);
    for i in 0..dot_size {
        for b in 0..dot_size {
            let value = ((i as f32 - point.0).powi(2) + (b as f32 - point.1).powi(2)).sqrt();
            dot[[i, b]] = value;
            coordinates_and_values.push((i, b, value))
        }
    }
    coordinates_and_values
        .sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let mut n = 0;

    for &(i, j, _) in &coordinates_and_values {
        let s = 0.5 - (step * n as f32);
        dot[[i, j]] = s;
        n += 1;
    }

    return dot;
}

fn dot(dot_inv: Array2<f32>) -> Array2<f32> {
    let dot = 1.0 - dot_inv;

    return dot;
}

pub fn create_dot(dot_size: usize, dot_type: TypeDot) -> (Array2<f32>, Array2<f32>) {
    let dot_inv = match dot_type {
        TypeDot::CROSS => dot_cross_inv(dot_size),
        TypeDot::ELLIPSE => dot_ellipse_inv(dot_size),
        TypeDot::LINE => dot_line_inv(dot_size),
        TypeDot::INVLINE => dot_invline_inv(dot_size),
        _ => dot_circle_inv(dot_size),
    };
    let dot = dot(dot_inv.clone());
    let dot_inv = dot_inv + 0.003;
    return (dot, dot_inv);
}
