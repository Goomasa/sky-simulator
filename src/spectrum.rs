use crate::{constant::E, math::Vec3};

pub type XYZ = Vec3;
pub type RGB = Vec3;

fn gaussian(w: f64, m: f64, t1: f64, t2: f64) -> f64 {
    let w_m2 = (w - m) * (w - m);
    if w < m {
        E.powf(-0.5 * t1 * t1 * w_m2)
    } else {
        E.powf(-0.5 * t2 * t2 * w_m2)
    }
}

pub fn color_matching(wavelength: f64) -> XYZ {
    let x = 1.056 * gaussian(wavelength, 599.8, 0.0264, 0.0323)
        + 0.362 * gaussian(wavelength, 442., 0.0624, 0.0374)
        - 0.065 * gaussian(wavelength, 501.1, 0.049, 0.0382);

    let y = 0.821 * gaussian(wavelength, 568.8, 0.0213, 0.0247)
        + 0.286 * gaussian(wavelength, 530.9, 0.0613, 0.0322);

    let z = 1.217 * gaussian(wavelength, 437., 0.0845, 0.0278)
        + 0.681 * gaussian(wavelength, 459., 0.0385, 0.0725);

    Vec3(x, y, z)
}

pub fn convert_to_srgb(xyz: &XYZ) -> RGB {
    let r = 3.2410 * xyz.0 + (-1.5374) * xyz.1 + (-0.4986) * xyz.2;
    let g = (-0.9692) * xyz.0 + 1.876 * xyz.1 + 0.0416 * xyz.2;
    let b = 0.0556 * xyz.0 + (-0.204) * xyz.1 + 1.0507 * xyz.2;
    Vec3(r, g, b)
}

pub fn gamma(v: RGB) -> (u32, u32, u32) {
    let r = (v.0.clamp(0., 1.).powf(1. / 2.2) * 255.) as u32;
    let g = (v.1.clamp(0., 1.).powf(1. / 2.2) * 255.) as u32;
    let b = (v.2.clamp(0., 1.).powf(1. / 2.2) * 255.) as u32;
    (r, g, b)
}
