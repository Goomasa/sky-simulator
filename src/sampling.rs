use crate::{
    constant::{EPS, PI, PI_INV},
    math::{Vec3, cross, dot, fmax},
    random::XorRand,
};

#[derive(Debug)]
pub enum ScatteringType {
    Rayleigh,
    Mie,
}

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
    fmax(dot(*normal, *dir) * PI_INV, 0.)
}

pub fn sample_wavelength(rand: &mut XorRand) -> f64 {
    // sample 380nm - 780nm
    380. + rand.next01() * 400.
}

pub fn pdf_sample_wavelength() -> f64 {
    1. / 400.
}

fn sample_phase_rayleigh(prev_dir: &Vec3, rand: &mut XorRand) -> (Vec3, f64) {
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

fn pdf_phase_rayleigh(dir: &Vec3, prev_dir: &Vec3) -> f64 {
    let dot = dot(*prev_dir, *dir);
    3. * (1. + dot * dot) / (16. * PI)
}

fn sample_phase_mie(prev_dir: &Vec3, rand: &mut XorRand) -> (Vec3, f64) {
    let w = *prev_dir;
    let u = if w.0.abs() > EPS {
        cross(Vec3(0., 1., 0.), w).normalize()
    } else {
        cross(Vec3(1., 0., 0.), w).normalize()
    };
    let v = cross(w, u);

    let phi = 2. * PI * rand.next01();

    let g = 0.8;
    let cos_theta = {
        let r = (1. - g * g) / (1. + g - 2. * g * rand.next01());
        -1. / (2. * g) * (1. + g * g - r * r)
    };
    let sin_theta = (1. - cos_theta * cos_theta).sqrt();

    (
        u * sin_theta * phi.cos() + v * sin_theta * phi.sin() + w * cos_theta,
        1. / (4. * PI) * (1. - g * g) / (1. + g * g + 2. * g * cos_theta).powf(1.5),
    )
}

fn pdf_phase_mie(dir: &Vec3, prev_dir: &Vec3) -> f64 {
    let dot = dot(*prev_dir, *dir);
    let g = 0.8;
    1. / (4. * PI) * (1. - g * g) / (1. + g * g + 2. * g * dot).powf(1.5)
}

pub fn sample_phase(sc_type: &ScatteringType, prev_dir: &Vec3, rand: &mut XorRand) -> (Vec3, f64) {
    if let ScatteringType::Rayleigh = sc_type {
        sample_phase_rayleigh(prev_dir, rand)
    } else {
        sample_phase_mie(prev_dir, rand)
    }
}

pub fn pdf_phase(sc_type: &ScatteringType, dir: &Vec3, prev_dir: &Vec3) -> f64 {
    if let ScatteringType::Rayleigh = sc_type {
        pdf_phase_rayleigh(dir, prev_dir)
    } else {
        pdf_phase_mie(dir, prev_dir)
    }
}
