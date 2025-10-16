use crate::{
    constant::{EPS, PI},
    math::{Point3, Vec3, cross, dot},
    random::XorRand,
    ray::{HitRecord, Ray},
    spectrum::rgb_to_reflectance,
    texture::Texture,
};

#[derive(Debug, Clone, Copy)]
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
        record.normal = (record.hitpoint - self.center) / self.radius;
        record.obj_type = self.obj_type;
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

pub struct Earth<'a> {
    pub shape: Sphere,
    pub texture: &'a Texture,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl<'a> Earth<'a> {
    pub fn new(shape: Sphere, texture: &'a Texture) -> Self {
        Earth {
            shape,
            texture,
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),
        }
    }

    pub fn get_reflectance(&self, point: &Point3, wavelength: f64) -> f64 {
        let op = (*point - self.shape.center).normalize();
        let theta = dot(self.w, op).acos(); // 0 - PI
        let u_op = dot(self.u, op);

        let mut phi; // 0 - 2PI
        if u_op < EPS && u_op >= 0. {
            phi = PI / 2.;
        } else if u_op > -EPS && u_op < 0. {
            phi = 3. * PI / 2.
        } else if PI - theta < EPS || theta + PI < EPS {
            phi = 0.
        } else {
            phi = (dot(self.v, op) / u_op).atan();
            let cos_phi = u_op / theta.sin();
            if cos_phi < 0. && phi < 0. {
                phi = PI + phi;
            } else if cos_phi < 0. && phi > 0. {
                phi = PI + phi;
            } else if cos_phi > 0. && phi < 0. {
                phi = 2. * PI + phi;
            }
        }

        let rgb = self.texture.get_rgb(phi / (2. * PI), theta / PI);

        rgb_to_reflectance(&rgb, wavelength)
    }
}
