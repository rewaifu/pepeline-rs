pub(crate) mod image {
    pub mod decode;
    pub mod save;
}

pub(crate) mod core {
    pub mod color_levels;
    pub mod convert;
}

pub(crate) mod screentone {
    mod dot;
    pub mod screentone_add;
}

pub(crate) mod functions {
    pub mod core_funcion;
    pub mod img_function;
    pub mod screentone_function;
}
