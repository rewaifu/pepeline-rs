use pyo3::prelude::*;

use utils::functions::{core_funcion, img_function, screentone_function};

mod utils;

/// A Python module implemented in Rust.
#[pymodule]
fn pepeline(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(img_function::read, m)?)?;
    m.add_function(wrap_pyfunction!(screentone_function::screentone, m)?)?;
    m.add_function(wrap_pyfunction!(core_funcion::fast_color_level, m)?)?;
    m.add_function(wrap_pyfunction!(img_function::save, m)?)?;
    Ok(())
}
