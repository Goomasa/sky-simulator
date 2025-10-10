use crate::{
    constant::SUN_LIGHT,
    math::{Vec3, dot},
    random::XorRand,
    ray::{HitRecord, Ray},
    sampling::{
        pdf_phase_rayleigh, pdf_sample_cos_hemi, sample_cos_hemisphere, sample_phase_rayleigh,
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
    orienting_normal: Vec3,
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
            orienting_normal: Vec3::zero(),
            throughput: 1.,
            total_pdf: 1.,
            pdf_sample_pt: -1.,
            value: 0.,
        }
    }

    fn roullete(&mut self, time: u32, rand: &mut XorRand) -> bool {
        let roullete_prob = if time > MAX_DEPTH {
            0.5 * (2_i32.pow(time - MAX_DEPTH)) as f64
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
        // lambertian model, albedo=0.5
        let new_dir = sample_cos_hemisphere(&self.orienting_normal, rand);
        let new_org = self.record.hitpoint + 0.00001 * self.orienting_normal;
        self.now_ray = Ray::new(new_org, new_dir);
        self.throughput *= 0.5;

        let nee_result = scene.nee(&new_org, self.wavelength, rand);
        if nee_result.pdf != 0. {
            let pdf_pt = pdf_sample_cos_hemi(&self.orienting_normal, &nee_result.dir);
            let cosine = dot(self.orienting_normal, nee_result.dir);
            let mis_weight = 1. / (pdf_pt + nee_result.pdf);
            //transmittance=1
            self.value += self.throughput * nee_result.value * cosine * mis_weight / self.total_pdf;
        }

        self.pdf_sample_pt = pdf_sample_cos_hemi(&self.orienting_normal, &new_dir);
    }

    fn freepath_sample(&mut self, scene: &Scene, rand: &mut XorRand) -> bool {
        let tracking_result = scene.delta_tracking(&self.now_ray, self.wavelength, rand);
        if let (None, point) = tracking_result {
            let (new_dir, pdf_phase_pt) = sample_phase_rayleigh(&self.now_ray.dir, rand);
            self.throughput *= 1.; // now, scattering_coeff=extinction_coeff

            let nee_result = scene.nee(&point, self.wavelength, rand);
            if nee_result.pdf != 0. {
                let pdf_phase_nee = pdf_phase_rayleigh(&nee_result.dir, &self.now_ray.dir);
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
                if self.freepath_sample(scene, rand) {
                    continue;
                }
            } else {
                self.record = HitRecord::new();
                if !scene.hit(&self.now_ray, &mut self.record) {
                    break;
                }
            }

            self.orienting_normal = if dot(self.now_ray.dir, self.record.normal) < 0. {
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
                    self.trace_earth(scene, rand);
                }
                ObjectType::Atmosphere => {
                    in_atmosphere = !in_atmosphere;
                    self.now_ray.org = self.record.hitpoint - 0.00001 * self.orienting_normal;
                }
            }
        }

        self.value
    }
}
