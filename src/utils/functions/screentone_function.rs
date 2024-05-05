use numpy::{PyArray2, PyReadonlyArray2, ToPyArray};
use pyo3::{pyfunction, Py, PyResult, Python};

use crate::utils::screentone::screentone_add::{screentone_add, screentone_rotate_add};

#[pyfunction]
pub fn screentone<'py>(
    input: PyReadonlyArray2<f32>,
    dot_size: usize,
    angle: Option<i16>,
    py: Python,
) -> PyResult<Py<PyArray2<f32>>> {
    // screentone overlay function:
    //     input -> array only 2D f32 0-1
    //     dot_size -> uint screenton size in pixels
    //     angle -> i16 degree by which we rotate the pattern
    let angle = angle.unwrap_or(0);
    let mut array = input.as_array().to_owned();
    if angle % 90 != 0 {
        screentone_rotate_add(&mut array, dot_size, (angle as f32).to_radians());
    } else {
        let lx_plus = dot_size / 2;
        let ly_plus = dot_size / 2;

        screentone_add(&mut array, dot_size, ly_plus, lx_plus);
    }

    Ok(array.to_pyarray(py).to_owned())
}
