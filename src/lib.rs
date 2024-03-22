mod utils;
use utils::image::function::{save,read,read32,save32};
use utils::screentone::screentone;
use utils::color_level::fast_color_level;
use pyo3::prelude::*;


/// A Python module implemented in Rust.
#[pymodule]
fn pepeline(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read, m)?)?;
    m.add_function(wrap_pyfunction!(read32, m)?)?;
    m.add_function(wrap_pyfunction!(screentone, m)?)?;
    m.add_function(wrap_pyfunction!(fast_color_level, m)?)?;
    m.add_function(wrap_pyfunction!(save, m)?)?;
    m.add_function(wrap_pyfunction!(save32, m)?)?;
    Ok(())
}
