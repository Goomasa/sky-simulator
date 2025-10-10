use crate::{
    constant::{EARTH_RAD, KARMAN_LINE, PI, SUN_RAD},
    math::{Point3, Vec3},
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
}
