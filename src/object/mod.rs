use std::fmt::{Display, Formatter, Result};
use std::sync::Arc;

pub use material::LambertianDiffuse;
pub use material::Metal;
pub use sphere::Sphere;
pub use world::World;

use crate::object::aabb::AABB;
use crate::object::material::Material;
use crate::object::sphere::MovingSphere;
use crate::utils::{Ray, Vec3};

pub mod aabb;
pub mod bvh;
pub mod material;
pub mod sphere;
pub mod world;

#[derive(Debug, Copy, Clone)]
pub enum Face {
    Inward,
    Outward,
}

impl Default for Face {
    fn default() -> Self {
        Face::Inward
    }
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

pub struct HitRecord {
    pub f: Face,
    pub t: f64,
    pub p: Vec3<f64>,
    pub normal: Vec3<f64>,
    pub mat: Arc<dyn Material + Send + Sync>,
}

impl Display for HitRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:?} t={} p={} normal={}",
            self.f, self.t, self.p, self.normal
        )
    }
}

pub trait Hittable {
    /// return the bounding box of the object
    /// note that some objects don't have a bounding box, like an infinite plane
    fn bounding_box(&self) -> Option<&AABB>;
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type HittableObject = Arc<dyn Hittable + Send + Sync>;
pub type MaterialObject = Arc<dyn Material + Send + Sync>;

pub fn make_material_object(a: impl Material + Send + Sync + 'static) -> MaterialObject {
    Arc::new(a)
}

pub fn make_sphere_object(center: Vec3<f64>, radius: f64, mat: &MaterialObject) -> HittableObject {
    Arc::new(Sphere::new(center, radius, &mat))
}

pub fn make_bouncing_sphere_object(
    center: Vec3<f64>,
    radius: f64,
    height: f64,
    t0: f64,
    t1: f64,
    mat: &MaterialObject,
) -> HittableObject {
    let mut c1 = center;
    c1.1 += height;
    Arc::new(MovingSphere::new(center, c1, t0, t1, radius, &mat))
}
