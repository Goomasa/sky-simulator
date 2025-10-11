use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::constant::PI;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub type Point3 = Vec3;

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Vec3 {
    pub fn new(c: f64) -> Self {
        Vec3(c, c, c)
    }

    pub fn zero() -> Self {
        Vec3(0., 0., 0.)
    }

    pub fn length_sq(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }
}

pub fn multiply(v: Vec3, w: Vec3) -> Vec3 {
    Vec3(v.0 * w.0, v.1 * w.1, v.2 * w.2)
}

pub fn dot(v: Vec3, w: Vec3) -> f64 {
    v.0 * w.0 + v.1 * w.1 + v.2 * w.2
}

pub fn cross(v: Vec3, w: Vec3) -> Vec3 {
    Vec3(
        v.1 * w.2 - v.2 * w.1,
        v.2 * w.0 - v.0 * w.2,
        v.0 * w.1 - v.1 * w.0,
    )
}

pub fn fmax(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

pub fn fmin(a: f64, b: f64) -> f64 {
    if a > b { b } else { a }
}

pub fn is_valid(v: &Vec3) -> bool {
    if v.0.is_nan() || v.1.is_nan() || v.2.is_nan() {
        return false;
    } else if v.0 < 0. || v.1 < 0. || v.2 < 0. {
        return false;
    }
    true
}

pub fn to_radian(angle: f64) -> f64 {
    angle * PI / 180.
}
