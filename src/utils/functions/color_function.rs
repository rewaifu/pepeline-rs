use ndarray::{Array2, Array3};
use numpy::{PyArrayDyn, PyReadonlyArrayDyn, ToPyArray};
use pyo3::{pyfunction, Py, PyResult, Python};

use crate::utils::core::color_levels::levels;
use crate::utils::core::cvt_color::cvt_color_float;
use crate::utils::core::enums::CvtType;

#[pyfunction]
pub fn fast_color_level<'py>(
    input: PyReadonlyArrayDyn<f32>,
    in_low: Option<u8>,
    in_high: Option<u8>,
    out_low: Option<u8>,
    out_high: Option<u8>,
    gamma: Option<f32>,
    py: Python,
) -> PyResult<Py<PyArrayDyn<f32>>> {
    let in_low = in_low.unwrap_or(0u8);
    let in_high = in_high.unwrap_or(255u8);
    let out_low = out_low.unwrap_or(0u8);
    let out_high = out_high.unwrap_or(255u8);
    let gamma = gamma.unwrap_or(1.0f32);
    let array = input.as_array().to_owned();
    let array = levels(array, in_low, in_high, out_low, out_high, gamma);

    Ok(array.to_pyarray(py).to_owned())
}

#[pyfunction]
pub fn cvt_color(
    input: PyReadonlyArrayDyn<f32>,
    cvt_type: CvtType,
    py: Python,
) -> PyResult<Py<PyArrayDyn<f32>>> {
    let array = input.as_array().to_owned();
    let binding = array.clone();
    let shape = binding.shape();
    let vec = array.into_raw_vec();
    let result_vec = cvt_color_float(&vec, cvt_type.clone());
    let array = match cvt_type {
        CvtType::RGB2Gray
        | CvtType::RGB2GrayAverage
        | CvtType::RGB2GrayBt709
        | CvtType::RGB2GrayBt2020 => Array2::from_shape_vec([shape[0], shape[1]], result_vec)
            .unwrap()
            .into_dyn(),
        CvtType::CMYK2RGB
        | CvtType::RGB2YCbCr
        | CvtType::YCbCr2RGB
        | CvtType::RGB2YCvCrBt2020
        | CvtType::YCvCr2RGBBt2020
        | CvtType::RGB2YCvCrBt709
        | CvtType::YCvCr2RGBBt709
        | CvtType::RGB2BGR
        | CvtType::BGR2RGB
        | CvtType::GRAY2RGB => Array3::from_shape_vec([shape[0], shape[1], 3], result_vec)
            .unwrap()
            .into_dyn(),
        CvtType::RGB2CMYK => Array3::from_shape_vec([shape[0], shape[1], 4], result_vec)
            .unwrap()
            .into_dyn(),
    };
    Ok(array.to_pyarray(py).to_owned())
}
