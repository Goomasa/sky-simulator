use crate::{
    constant::{EARTH_RAD, KARMAN_LINE, PI_INV, SUN_LIGHT},
    math::{Vec3, dot, fmax},
    random::XorRand,
    ray::{HitRecord, Ray},
    sampling::{
        ScatteringType, pdf_phase, pdf_sample_cos_hemi, reflection_dir, sample_cos_hemisphere,
        sample_phase,
    },
    scene::Scene,
    sphere::ObjectType,
};

const DEPTH: u32 = 6;
const MAX_DEPTH: u32 = 30;

pub struct Pathtracing {
    wavelength: f64,
    now_ray: Ray,
    record: HitRecord,
    obj_normal: Vec3,
    texture_normal: Vec3,
    throughput: f64,
    total_pdf: f64,
    pdf_sample_pt: f64,
    value: f64,
}

impl Pathtracing {
    pub fn new(ray: Ray, wavelength: f64) -> Self {
        Pathtracing {
            wavelength,
            now_ray: ray,
            record: HitRecord::new(),
            obj_normal: Vec3::zero(),
            texture_normal: Vec3::zero(),
            throughput: 1.,
            total_pdf: 1.,
            pdf_sample_pt: -1.,
            value: 0.,
        }
    }

    fn roullete(&mut self, time: u32, rand: &mut XorRand) -> bool {
        let roullete_prob = if time > MAX_DEPTH {
            0.5 / (2_i32.pow(time - MAX_DEPTH)) as f64
        } else if time <= DEPTH {
            1.
        } else {
            0.5
        };

        if rand.next01() < roullete_prob {
            self.total_pdf *= roullete_prob;
            true
        } else {
            false
        }
    }

    fn trace_earth(&mut self, scene: &Scene, rand: &mut XorRand) {
        // lambertian model
        let new_dir = sample_cos_hemisphere(&self.texture_normal, rand);
        let new_org = self.record.hitpoint + 0.00001 * self.obj_normal;
        self.now_ray = Ray::new(new_org, new_dir);
        self.throughput *= scene
            .earth
            .get_reflectance(&self.record.hitpoint, self.wavelength);

        let coeff_rayleigh = scene.scattering_coeff_rayleigh(&new_org, self.wavelength);
        let coeff_mie = scene.coeff_mie(&new_org);
        let sc_type =
            if rand.next01() < coeff_rayleigh / (coeff_rayleigh + coeff_mie.0 + coeff_mie.1) {
                ScatteringType::Rayleigh
            } else {
                ScatteringType::Mie
            };

        let nee_result = scene.nee(&new_org, self.wavelength, &sc_type, rand);
        if nee_result.pdf != 0. {
            let pdf_pt = pdf_sample_cos_hemi(&self.texture_normal, &nee_result.dir);
            let cosine = fmax(dot(self.texture_normal, nee_result.dir), 0.);
            let mis_weight = 1. / (pdf_pt + nee_result.pdf);
            //transmittance=1
            self.value +=
                self.throughput * nee_result.value * PI_INV * cosine * mis_weight / self.total_pdf;
        }

        self.pdf_sample_pt = pdf_sample_cos_hemi(&self.texture_normal, &new_dir);
    }

    fn trace_earth_specular(&mut self, scene: &Scene) {
        self.throughput *= scene
            .earth
            .get_reflectance(&self.record.hitpoint, self.wavelength);

        let new_dir = reflection_dir(&self.now_ray.dir, &self.texture_normal);
        let new_org = self.record.hitpoint + self.obj_normal * 0.00001;
        self.now_ray = Ray::new(new_org, new_dir);
        self.pdf_sample_pt = 1.;
    }

    fn freepath_sample(&mut self, scene: &Scene, rand: &mut XorRand) -> bool {
        let coeff_rayleigh = scene.scattering_coeff_rayleigh(&self.now_ray.org, self.wavelength);
        let coeff_mie = scene.coeff_mie(&self.now_ray.org);

        let single_albedo;
        let sc_type;
        if rand.next01() < coeff_rayleigh / (coeff_rayleigh + coeff_mie.0 + coeff_mie.1) {
            single_albedo = 1.;
            sc_type = ScatteringType::Rayleigh;
        } else {
            single_albedo = coeff_mie.0 / (coeff_mie.0 + coeff_mie.1);
            sc_type = ScatteringType::Mie;
        }

        let tracking_result = scene.delta_tracking(&self.now_ray, self.wavelength, &sc_type, rand);
        if let (None, point) = tracking_result {
            let (new_dir, pdf_phase_pt) = sample_phase(&sc_type, &-self.now_ray.dir, rand);
            self.throughput *= single_albedo;

            let nee_result = scene.nee(&point, self.wavelength, &sc_type, rand);
            if nee_result.pdf != 0. {
                let pdf_phase_nee = pdf_phase(&sc_type, &nee_result.dir, &-self.now_ray.dir);
                let mis_weight = 1. / (pdf_phase_nee + nee_result.pdf);
                self.value += self.throughput * nee_result.value * pdf_phase_nee * mis_weight
                    / self.total_pdf;
            }

            self.now_ray = Ray::new(point, new_dir);
            self.pdf_sample_pt = pdf_phase_pt;
            return false;
        } else if let (Some(record), _) = tracking_result {
            self.record = record;
        }

        // hit to obj
        true
    }

    pub fn integrate(&mut self, scene: &Scene, rand: &mut XorRand) -> f64 {
        let mut in_atmosphere = scene.in_atmosphere(&self.now_ray.org);
        for time in 0.. {
            if !self.roullete(time, rand) {
                break;
            }

            if in_atmosphere {
                if !self.freepath_sample(scene, rand) {
                    continue;
                }
            } else {
                self.record = HitRecord::new();
                if !scene.hit(&self.now_ray, &mut self.record) {
                    break;
                }
            }

            self.obj_normal = if dot(self.now_ray.dir, self.record.normal) < 0. {
                self.record.normal
            } else {
                -self.record.normal
            };

            match self.record.obj_type {
                ObjectType::Sun => {
                    if self.pdf_sample_pt < 0. {
                        self.value += self.throughput * SUN_LIGHT / self.total_pdf;
                    } else {
                        let pdf_nee = scene.sun.pdf_sampling(&self.now_ray.org);
                        let mis_weight = self.pdf_sample_pt / (self.pdf_sample_pt + pdf_nee);
                        self.value += self.throughput * SUN_LIGHT * mis_weight / self.total_pdf;
                    }
                    break;
                }
                ObjectType::Earth => {
                    let (u, v) = scene.earth.get_uv(&self.record.hitpoint);
                    let (specular, normal) = scene.earth.get_property(&self.record.hitpoint, u, v);

                    self.texture_normal = if dot(self.now_ray.dir, normal) < 0. {
                        normal
                    } else {
                        -normal
                    };

                    if specular < 120 {
                        self.trace_earth(scene, rand);
                    } else {
                        self.trace_earth_specular(scene);
                    }
                }
                ObjectType::Atmosphere => {
                    in_atmosphere = !in_atmosphere;
                    self.now_ray.org = self.record.hitpoint - self.obj_normal * 0.01;

                    let h = (self.now_ray.org - scene.earth.shape.center).length() - EARTH_RAD;

                    if in_atmosphere && h > KARMAN_LINE {
                        println!("{}, {}", h, time);
                        break;
                    } else if !in_atmosphere && h < KARMAN_LINE {
                        println!("{}, {}", h, time);
                        break;
                    }
                }
            }
        }

        self.value
    }
}
