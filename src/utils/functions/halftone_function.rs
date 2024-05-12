use numpy::{PyArray2, PyReadonlyArray2, ToPyArray};
use pyo3::{Py, pyfunction, PyResult, Python};

use crate::utils::core::enums::TypeDot;
// use crate::utils::halftone::halftone_add::{halftone_add, RgbHalftone};
use crate::utils::halftone::screentone_add::{screentone_add, screentone_rotate_add};

// #[pyfunction]
// pub fn halftone<'py>(
//     input: PyReadonlyArray3<f32>,
//     halftone: RgbHalftone,
//     py: Python,
// ) -> PyResult<Py<PyArray3<f32>>> {
//     // halftone overlay function:
//     //     input -> array only 2D f32 0-1
//     //     dot_size -> uint screenton size in pixels
//     //     angle -> i16 degree by which we rotate the pattern
//     let mut array = input.as_array().to_owned();
//     halftone_add(&mut array, halftone);
//
//     Ok(array.to_pyarray_bound(py).into())
// }

#[pyfunction]
pub fn screentone<'py>(
    input: PyReadonlyArray2<f32>,
    dot_size: usize,
    angle: Option<i16>,
    dot_type: Option<TypeDot>,
    py: Python,
) -> PyResult<Py<PyArray2<f32>>> {
    // halftone overlay function:
    //     input -> array only 2D f32 0-1
    //     dot_size -> uint screenton size in pixels
    //     angle -> i16 degree by which we rotate the pattern
    let angle = angle.unwrap_or(0);
    let mut array = input.as_array().to_owned();
    let dot_type = dot_type.unwrap_or(TypeDot::CIRCLE);
    if angle != 0 {
        screentone_rotate_add(&mut array, dot_size, (angle as f32).to_radians(), dot_type);
    } else {
        screentone_add(&mut array, dot_size, dot_type);
    }

    Ok(array.to_pyarray_bound(py).into())
}
