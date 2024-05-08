pub(crate) mod image {
    pub mod decode;
    pub mod save;
}

pub(crate) mod core {
    pub mod color_levels;
    pub mod convert;
    pub mod cvt_color_float;
    pub mod cvt_constants;
    pub mod enums;
    pub mod noise;

    pub mod array {
        pub mod utils;
    }
}

pub(crate) mod halftone {
    pub mod dot;
    pub mod screentone_add;
    pub mod utils_halftone;
}

pub(crate) mod functions {
    pub mod color_function;
    pub mod core_funcion;
    pub mod halftone_function;
    pub mod img_function;
}
