use crate::{
    constant::{AXIS, EARTH_RAD, EARTH_TO_SUN, PI, SUN_RAD},
    math::{Point3, Vec3, cross, to_radian},
    scene::Scene,
};

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Eye {
    time: f64,            // 0 - 24 [h]
    latitude: f64,        // north latitude: 0 - 90, south latitude: -90 - 0
    altitude: f64,        // [km]
    direction: Direction, // NSWE
    elevation: f64,       // -90 - 90
}

impl Eye {
    pub fn new(
        time: f64,
        latitude: f64,
        altitude: f64,
        direction: Direction,
        elevation: f64,
    ) -> Self {
        Eye {
            time,
            latitude,
            altitude,
            direction,
            elevation,
        }
    }

    fn get_position(&self, scene: &mut Scene) -> Point3 {
        let w = Vec3(0., to_radian(AXIS + 90.).cos(), to_radian(AXIS + 90.).sin());
        let u =
            (scene.earth.shape.center - scene.sun.center) / (SUN_RAD + EARTH_RAD + EARTH_TO_SUN);
        let v = cross(w, u).normalize();
        let u = cross(v, w).normalize();

        scene.earth.u = u;
        scene.earth.v = v;
        scene.earth.w = w;

        let phi = PI * self.time / 12.;
        let theta = to_radian(90. - self.latitude);
        let r = EARTH_RAD + self.altitude;

        r * (u * theta.sin() * phi.cos() + v * theta.sin() * phi.sin() + w * theta.cos())
            + scene.earth.shape.center
    }

    fn get_direction(&self, scene: &Scene, pos: &Point3) -> Vec3 {
        // can not calculate if eys is on the N/S Pole
        let w = (*pos - scene.earth.shape.center).normalize();
        let axis = Vec3(0., to_radian(AXIS + 90.).cos(), to_radian(AXIS + 90.).sin());
        let u = cross(axis, w).normalize(); // east
        let v = cross(w, u); // north

        let theta = to_radian(90. - self.elevation);
        let phi = match self.direction {
            Direction::North => to_radian(90.),
            Direction::South => to_radian(270.),
            Direction::East => 0.,
            Direction::West => to_radian(180.),
        };

        u * theta.sin() * phi.cos() + v * theta.sin() * phi.sin() + w * theta.cos()
    }
}

pub struct Camera {
    pub pixel_num_w: u32,
    pub pixel_num_h: u32,

    pub eye_pos: Point3,
    sensor_corner: Point3,
    pixel_u: Vec3,
    pixel_v: Vec3,

    pub spp: u32,
    pub sspp: u32,
}

impl Camera {
    pub fn new(
        eye: &Eye,
        scene: &mut Scene,
        pixel_num_w: u32,
        pixel_num_h: u32,
        eye_to_sensor: f64,
        sensor_w: f64,
        spp: u32,
        sspp: u32,
    ) -> Self {
        let eye_pos = eye.get_position(scene);
        let eye_dir = eye.get_direction(scene, &eye_pos);
        let sensor_h = sensor_w * pixel_num_h as f64 / pixel_num_w as f64;

        let up = (eye_pos - scene.earth.shape.center).normalize();
        let sensor_u = cross(eye_dir, up).normalize() * sensor_w;
        let sensor_v = cross(eye_dir, sensor_u).normalize() * sensor_h;
        let pixel_u = sensor_u / pixel_num_w as f64;
        let pixel_v = sensor_v / pixel_num_h as f64;
        let sensor_corner = eye_pos + eye_dir * eye_to_sensor - sensor_u / 2. - sensor_v / 2.;

        Camera {
            pixel_num_w,
            pixel_num_h,
            eye_pos,
            sensor_corner,
            pixel_u,
            pixel_v,
            spp,
            sspp,
        }
    }

    pub fn get_pixel_center(&self, u: f64, v: f64, su: f64, sv: f64) -> Point3 {
        let du = self.pixel_u * (u + (su + 0.5) / self.sspp as f64);
        let dv = self.pixel_v * (v + (sv + 0.5) / self.sspp as f64);
        self.sensor_corner + du + dv
    }
}
