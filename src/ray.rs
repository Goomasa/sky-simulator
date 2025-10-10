use crate::{
    constant::INF,
    math::{Point3, Vec3},
    sphere::ObjectType,
};

pub struct Ray {
    pub org: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(org: Point3, dir: Vec3) -> Self {
        Ray { org, dir }
    }
}

pub struct HitRecord {
    pub hitpoint: Point3,
    pub distance: f64,
    pub normal: Vec3,
    pub obj_type: ObjectType,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            hitpoint: Vec3::zero(),
            distance: INF,
            normal: Vec3::zero(),
            obj_type: ObjectType::Sun,
        }
    }
}
