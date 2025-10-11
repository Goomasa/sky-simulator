use crate::{
    constant::{E, EARTH_RAD, EARTH_TO_SUN, KARMAN_LINE, NS, PI, PN, SUN_LIGHT, SUN_RAD},
    math::{Point3, Vec3, dot, fmax},
    random::XorRand,
    ray::{HitRecord, Ray},
    sphere::{ObjectType, Sphere},
};

pub struct NeeResult {
    pub pdf: f64,
    pub value: f64,
    pub dir: Vec3,
}

impl NeeResult {
    fn new(pdf: f64, value: f64, dir: Vec3) -> Self {
        NeeResult { pdf, value, dir }
    }
}

pub struct Scene {
    pub sun: Sphere,
    pub earth: Sphere,
    pub atmosphere: Sphere,
}

impl Scene {
    pub fn new(month: u32) -> Self {
        let sun = Sphere::new(Vec3::zero(), SUN_RAD, ObjectType::Sun);

        let earth_center = {
            let earth_phi = (month - 3) as f64 * PI / 6.;
            let r = EARTH_RAD + SUN_RAD + EARTH_TO_SUN;
            Vec3(r * earth_phi.cos(), r * earth_phi.sin(), 0.)
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
        let _ = self.earth.hit(ray, record);
        let _ = self.sun.hit(ray, record);
        self.atmosphere.hit(ray, record)
    }

    pub fn in_atmosphere(&self, point: &Point3) -> bool {
        let d = (*point - self.earth.center).length() - EARTH_RAD;
        d > 0. && d < KARMAN_LINE
    }

    // wavelength: [nm]
    pub fn scattering_coeff_rayleigh(&self, point: &Point3, wavelength: f64) -> f64 {
        let h = fmax((*point - self.earth.center).length() - EARTH_RAD, 0.);
        let ior = get_ior(wavelength);

        let mu0 = {
            let w_cm = wavelength * 1e-7; // [nm] -> [cm]
            let l = 24. * PI.powi(3) / (w_cm.powi(4) * NS);
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
        let dot = dot(po.normalize(), ray.dir);

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
    ) -> (Option<HitRecord>, Point3) {
        // return (hit_record, point)
        let majorant = self.scattering_coeff_rayleigh(&self.altitude_min_point(ray), wavelength); // absorption-coeff=0
        let mut record = HitRecord::new();
        let _ = self.hit(ray, &mut record);

        let mut to_border = record.distance;
        let mut sampled_len = -rand.next01().ln() / majorant;
        let mut point = ray.org + sampled_len * ray.dir;

        to_border -= sampled_len;

        loop {
            if to_border < 0. {
                return (Some(record), point);
            }

            let ratio = self.scattering_coeff_rayleigh(&point, wavelength) / majorant;
            if rand.next01() < ratio {
                return (None, point);
            }

            sampled_len = -rand.next01().ln() / majorant;
            point = ray.org + sampled_len * ray.dir;
            to_border -= sampled_len;
        }
    }

    pub fn nee(&self, org: &Point3, wavelength: f64, rand: &mut XorRand) -> NeeResult {
        let (sample_point, pdf) = self.sun.sample(org, rand);
        let dir = (sample_point - *org).normalize();
        let ray = Ray::new(*org, dir);
        let mut record = HitRecord::new();
        record.distance = (sample_point - *org).length();
        if self.earth.hit(&ray, &mut record) {
            return NeeResult::new(0., 0., dir);
        }

        //org is in atmosphere
        if let (Some(_), _) = self.delta_tracking(&ray, wavelength, rand) {
            // transmittance=1
            return NeeResult::new(pdf, SUN_LIGHT, dir);
        }

        NeeResult::new(0., 0., dir)
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
