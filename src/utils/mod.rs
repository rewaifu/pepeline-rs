pub(crate) mod image {
    pub mod decode;
    pub mod save;
    pub mod size_decode;
}

pub(crate) mod core {
    pub mod color_levels;
    pub mod convert;
    pub mod cvt_color_float;
    pub mod cvt_constants;
    pub mod enums;
    pub mod noise;
}

pub(crate) mod halftone {
    pub mod dot;
    // pub mod halftone_add;
    pub mod screentone_add;
    pub mod utils_halftone;
}

pub(crate) mod functions {
    pub mod color_function;
    pub mod core_funcion;
    pub mod halftone_function;
    pub mod img_function;
}
