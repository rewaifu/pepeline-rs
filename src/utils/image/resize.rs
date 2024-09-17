use fast_image_resize::images::{Image, ImageRef};
use fast_image_resize::{ResizeAlg, ResizeOptions, Resizer};
use std::error::Error;

pub fn resize_image(
    img: &ImageRef,
    filter: ResizeAlg,
    result_img: &mut Image,
) -> Result<(), Box<dyn Error>> {
    let mut resizer = Resizer::new();
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use fast_image_resize::CpuExtensions;
        resizer.set_cpu_extensions(CpuExtensions::Sse4_1);
    }
    resizer.resize(img, result_img, &ResizeOptions::new().resize_alg(filter))?;

    Ok(())
}
