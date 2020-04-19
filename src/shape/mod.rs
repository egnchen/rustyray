mod sphere;
mod world;

pub use sphere::Sphere;
pub use world::World;
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Debug)]
pub enum Face {
    Inward,
    Outward,
}

impl Default for Face {
    fn default() -> Self { Face::Inward }
}

impl Face {
    pub fn calc(p: &Vec3<f64>, r: &Ray) -> Face {
        if p.dot(r.direction()) < 0.0 {
            Face::Inward
        } else {
            Face::Outward
        }
    }
}

#[derive(Default)]
pub struct HitRecord {
    pub f: Face,
    pub t: f64,
    pub p: Vec3<f64>,
    pub normal: Vec3<f64>,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
