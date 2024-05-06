use pyo3::prelude::*;

use utils::functions::{core_funcion, img_function, screentone_function};

use crate::utils::screentone::dot::TypeDot;

mod utils;

/// A Python module implemented in Rust.
#[pymodule]
fn pepeline(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(img_function::read, m)?)?;
    m.add_function(wrap_pyfunction!(screentone_function::screentone, m)?)?;
    m.add_function(wrap_pyfunction!(core_funcion::fast_color_level, m)?)?;
    m.add_function(wrap_pyfunction!(core_funcion::noise_generate, m)?)?;
    m.add_function(wrap_pyfunction!(img_function::save, m)?)?;
    m.add_function(wrap_pyfunction!(core_funcion::crop_cord, m)?)?;
    m.add_function(wrap_pyfunction!(core_funcion::rgb2cmyk, m)?)?;
    m.add_function(wrap_pyfunction!(core_funcion::cmyk2rgb, m)?)?;
    m.add_class::<core_funcion::TypeNoise>()?;
    m.add_class::<TypeDot>()?;
    Ok(())
}
