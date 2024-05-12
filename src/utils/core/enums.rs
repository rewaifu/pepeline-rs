use pyo3::pyclass;

#[derive(Clone)]
#[pyclass]
pub enum TypeNoise {
    PERLIN = 0,
    SIMPLEX = 1,
    OPENSIMPLEX = 2,
    SUPERSIMPLEX = 3,
    PERLINSURFLET = 4,
}

#[derive(Clone)]
#[pyclass]
pub enum CvtType {
    RGB2Gray = 0, //NTSC
    RGB2GrayAverage = 1,
    RGB2GrayBt709 = 2,
    RGB2GrayBt2020 = 3,
    RGB2CMYK = 4,
    CMYK2RGB = 5,
    RGB2YCbCr = 6, //bt 601
    YCbCr2RGB = 7, //bt 601
    RGB2YCvCrBt2020 = 8,
    YCvCr2RGBBt2020 = 9,
    RGB2YCvCrBt709 = 10,
    YCvCr2RGBBt709 = 11,
    RGB2BGR = 12,
    BGR2RGB = 13,
    GRAY2RGB = 14,
    RGB2Luma = 15,
}

#[pyclass]
#[derive(Clone, Copy)]
pub enum TypeDot {
    CIRCLE = 0,
    CROSS = 1,
    ELLIPSE = 2,
    LINE = 3,
    INVLINE = 4,
}
