use crate::{
    constant::EPS,
    math::{Point3, dot},
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
}
