use pyo3::prelude::*;

use utils::functions::{color_function, core_funcion, halftone_function, img_function};

use crate::utils::core::enums::{CvtType, TypeDot, TypeNoise};

mod utils;

/// A Python module implemented in Rust.
#[pymodule]
fn pepeline(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(img_function::read, m)?)?;
    m.add_function(wrap_pyfunction!(img_function::read_size, m)?)?;
    m.add_function(wrap_pyfunction!(halftone_function::screentone, m)?)?;
    // m.add_function(wrap_pyfunction!(halftone_function::halftone, m)?)?;
    m.add_function(wrap_pyfunction!(core_funcion::noise_generate, m)?)?;
    m.add_function(wrap_pyfunction!(img_function::save, m)?)?;
    m.add_function(wrap_pyfunction!(core_funcion::crop_cord, m)?)?;
    m.add_function(wrap_pyfunction!(core_funcion::best_tile, m)?)?;
    // m.add_function(wrap_pyfunction!(core_funcion::cmyk_shift, m)?)?;
    m.add_function(wrap_pyfunction!(color_function::fast_color_level, m)?)?;
    m.add_function(wrap_pyfunction!(color_function::cvt_color, m)?)?;
    m.add_class::<TypeNoise>()?;
    m.add_class::<TypeDot>()?;
    m.add_class::<CvtType>()?;
    Ok(())
}
