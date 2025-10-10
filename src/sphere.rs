use crate::{
    constant::{EPS, PI},
    math::{Point3, Vec3, cross, dot},
    random::XorRand,
    ray::{HitRecord, Ray},
};

pub enum ObjectType {
    Sun,
    Earth,
    Atmosphere,
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub obj_type: ObjectType,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, obj_type: ObjectType) -> Self {
        Sphere {
            center,
            radius,
            obj_type,
        }
    }

    pub fn hit(&self, ray: &Ray, record: &mut HitRecord) -> bool {
        let po = self.center - ray.org;
        let b = dot(po, ray.dir);
        let d = b * b - dot(po, po) + self.radius.powi(2);

        if d < 0. {
            return false;
        }

        let t1 = b - d.sqrt();
        let t2 = b + d.sqrt();

        if t1 > EPS && t1 < record.distance {
            record.distance = t1;
        } else if t2 > EPS && t2 < record.distance {
            record.distance = t2;
        } else {
            return false;
        }

        record.hitpoint = ray.org + ray.dir * record.distance;
        record.normal = (record.hitpoint - self.center).normalize();
        true
    }

    pub fn sample(&self, org: &Point3, rand: &mut XorRand) -> (Point3, f64) {
        let po = self.center - *org;
        let cos_mu = (1. - (self.radius * self.radius / po.length_sq())).sqrt();

        let w = po.normalize();
        let u = if w.0 > EPS || w.0 < (-EPS) {
            cross(w, Vec3(0., 1., 0.)).normalize()
        } else {
            cross(w, Vec3(1., 0., 0.)).normalize()
        };
        let v = cross(w, u);

        let phi = 2. * PI * rand.next01();
        let cos_theta = 1. - rand.next01() * (1. - cos_mu);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let dir = u * sin_theta * phi.cos() + v * sin_theta * phi.sin() + w * cos_theta;
        let pdf = 1. / (2. * PI * (1. - cos_mu));
        (dir, pdf)
    }

    pub fn pdf_sampling(&self, org: &Point3) -> f64 {
        let cos_mu = (1. - (self.radius * self.radius / (self.center - *org).length_sq())).sqrt();
        1. / (2. * PI * (1. - cos_mu))
    }
}
