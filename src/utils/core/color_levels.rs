use ndarray::ArrayD;

pub fn levels(
    vec_img: &mut ArrayD<f32>,
    in_low: u8,
    in_high: u8,
    out_low: u8,
    out_high: u8,
    gamma: f32,
) {
    let in_low = in_low as f32 / 255.0;
    let in_high = in_high as f32 / 255.0;
    let out_low = out_low as f32 / 255.0;
    let out_high = out_high as f32 / 255.0;
    let in_range = in_high - in_low;
    let out_range = out_high - out_low;
    if gamma != 1.0 && out_range != 1.0 {
        for x in vec_img.iter_mut() {
            *x = ((*x - in_low) / in_range * out_range + out_low)
                .max(0.0)
                .min(1.0)
                .powf(gamma);
        }
    } else if gamma != 1.0 {
        for x in vec_img.iter_mut() {
            *x = ((*x - in_low) / in_range).max(0.0).min(1.0).powf(gamma);
        }
    } else if gamma == 1.0 && out_range != 1.0 {
        for x in vec_img.iter_mut() {
            *x = ((*x - in_low) / in_range * out_range + out_low)
                .max(0.0)
                .min(1.0);
        }
    } else {
        for x in vec_img.iter_mut() {
            *x = ((*x - in_low) / in_range).max(0.0).min(1.0);
        }
    }
}
