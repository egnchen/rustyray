use crate::vec::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub start_corner: Vec3<f64>,
    pub horizontal: Vec3<f64>,
    pub vertical: Vec3<f64>,
    pub origin: Vec3<f64>,
}

impl Camera {
    pub fn new(origin: Vec3<f64>, start_corner: Vec3<f64>, hlength: f64, vlength: f64) -> Camera {
        Camera {
            start_corner,
            horizontal: Vec3(hlength, 0.0, 0.0),
            vertical: Vec3(0.0, vlength, 0.0),
            origin,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.start_corner + self.horizontal * u + self.vertical * v,
        }
    }
}