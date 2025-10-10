use crate::{
    constant::{EPS, PI, PI_INV},
    math::{Vec3, cross, dot},
    random::XorRand,
};

pub fn sample_cos_hemisphere(normal: &Vec3, rand: &mut XorRand) -> Vec3 {
    let w = *normal;
    let u = if w.0.abs() > EPS {
        cross(Vec3(0., 1., 0.), w).normalize()
    } else {
        cross(Vec3(1., 0., 0.), w).normalize()
    };
    let v = cross(w, u);

    let phi = 2. * PI * rand.next01();
    let sin_theta2 = rand.next01();
    let sin_theta = sin_theta2.sqrt();

    u * sin_theta * phi.cos() + v * sin_theta * phi.sin() + w * (1. - sin_theta2).sqrt()
}

pub fn pdf_sample_cos_hemi(normal: &Vec3, dir: &Vec3) -> f64 {
    dot(*normal, *dir) * PI_INV
}

pub fn sample_wavelength(rand: &mut XorRand) -> f64 {
    // sample 380nm - 779nm
    380. + (rand.nexti() % 400) as f64
}

pub fn pdf_sample_wavelength() -> f64 {
    1. / 400.
}

pub fn sample_phase_rayleigh(prev_dir: &Vec3, rand: &mut XorRand) -> (Vec3, f64) {
    let w = *prev_dir;
    let u = if w.0.abs() > EPS {
        cross(Vec3(0., 1., 0.), w).normalize()
    } else {
        cross(Vec3(1., 0., 0.), w).normalize()
    };
    let v = cross(w, u);

    let phi = 2. * PI * rand.next01();

    let tmp = {
        let r = rand.next01();
        (-4. * r + 2. + (16. * r * r - 16. * r + 5.).sqrt()).powf(1. / 3.)
    };
    let cos_theta = tmp - 1. / tmp;
    let sin_theta = (1. - cos_theta * cos_theta).sqrt();

    (
        u * sin_theta * phi.cos() + v * sin_theta * phi.sin() + w * cos_theta,
        3. * (1. + cos_theta * cos_theta) / (16. * PI),
    )
}

pub fn pdf_phase_rayleigh(dir: &Vec3, prev_dir: &Vec3) -> f64 {
    let dot = dot(*prev_dir, *dir);
    3. * (1. + dot * dot) / (16. * PI)
}
