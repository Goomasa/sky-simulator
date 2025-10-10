use crate::{
    constant::{E, EARTH_RAD, KARMAN_LINE, NS, PI, PN, SUN_RAD},
    math::{Point3, Vec3, dot},
    random::XorRand,
    ray::{HitRecord, Ray},
    sphere::{ObjectType, Sphere},
};

pub struct Scene {
    pub sun: Sphere,
    pub earth: Sphere,
    pub atmosphere: Sphere,
}

impl Scene {
    pub fn new(month: u32) -> Self {
        let sun = Sphere::new(Vec3::zero(), SUN_RAD, ObjectType::Sun);

        let earth_center = {
            let earth_phi = month as f64 * PI / 6.;
            Vec3(EARTH_RAD * earth_phi.cos(), EARTH_RAD * earth_phi.sin(), 0.)
        };
        let earth = Sphere::new(earth_center, EARTH_RAD, ObjectType::Earth);

        let atmosphere = Sphere::new(
            earth_center,
            EARTH_RAD + KARMAN_LINE,
            ObjectType::Atmosphere,
        );

        Scene {
            sun,
            earth,
            atmosphere,
        }
    }

    pub fn hit(&self, ray: &Ray, record: &mut HitRecord) -> bool {
        let mut is_hit = false;
        is_hit = self.earth.hit(ray, record);
        is_hit = self.sun.hit(ray, record);
        is_hit = self.atmosphere.hit(ray, record);

        is_hit
    }

    pub fn in_atmosphere(&self, point: &Point3) -> bool {
        let d = (*point - self.earth.center).length() - EARTH_RAD;
        d > 0. && d < KARMAN_LINE
    }

    // wavelength: [nm]
    pub fn scattering_coeff_rayleigh(&self, point: &Point3, wavelength: f64) -> f64 {
        let h = (*point - self.earth.center).length() - EARTH_RAD;
        let ior = get_ior(wavelength);

        let mu0 = {
            let w_cm = wavelength * 1e-7; // [nm] -> [cm]
            let l = 24. * PI.powi(3) / (w_cm.powi(4) * NS * NS);
            let m = ((ior * ior - 1.) / (ior * ior + 2.)).powi(2);
            let r = (6. + 3. * PN) / (6. - 7. * PN);

            l * m * r
        };

        let coeff = {
            let a = 0.07771971;
            let b = 1.16364243;

            E.powf(-a * h.powf(b))
        };

        mu0 * 1e5 * coeff
    }

    pub fn altitude_min_point(&self, ray: &Ray) -> Point3 {
        let po = ray.org - self.earth.center;
        let dot = dot(po, ray.org);

        if dot > 0. {
            ray.org
        } else {
            let mut record = HitRecord::new();
            if self.earth.hit(ray, &mut record) {
                return record.hitpoint;
            }

            ray.org + ray.dir * (-dot * po.length())
        }
    }

    // delta-tracking for rayleigh-scattering
    pub fn delta_tracking(
        &self,
        ray: &Ray,
        wavelength: f64,
        rand: &mut XorRand,
    ) -> (Option<HitRecord>, Point3, f64) {
        // return (hit_record, point, pdf)
        let majorant = self.scattering_coeff_rayleigh(&self.altitude_min_point(ray), wavelength); // absorption-coeff=0
        let mut record = HitRecord::new();
        let _ = self.hit(ray, &mut record);

        let mut to_border = record.distance;
        let mut sampled_len = -rand.next01().ln() / majorant;
        let mut pdf = majorant * (-majorant * sampled_len).exp();
        let mut point = ray.org + sampled_len * ray.dir;

        to_border -= sampled_len;

        loop {
            if to_border < 0. {
                return (Some(record), point, pdf);
            }

            let ratio = self.scattering_coeff_rayleigh(&point, wavelength) / majorant;
            if rand.next01() < ratio {
                return (None, point, pdf * ratio);
            }

            pdf *= 1. - ratio;

            sampled_len = -rand.next01().ln() / majorant;
            pdf *= majorant * (-majorant * sampled_len).exp();
            point = ray.org + sampled_len * ray.dir;
            to_border -= sampled_len;
        }
    }
}

/*
Anthony Bucholtz. Rayleigh-scattering calculations for the terrestrial
atmosphere. In: Applied Optics 34.15 (May 20, 1995), pp. 2765–2773.
*/
fn get_ior(wavelength: f64) -> f64 {
    let w_inv2 = 1. / (wavelength * 1e-3).powi(2);
    let l = 5791817. / (238.0185 - w_inv2);
    let r = 167909. / (57.362 - w_inv2);
    (l + r) * 1e-8 + 1.
}
