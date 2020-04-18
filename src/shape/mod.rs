mod sphere;
use crate::ray::Ray;

pub use sphere::Sphere;

pub trait Shape {
    fn hit(&self, r: &Ray) -> Option<f64>;
}