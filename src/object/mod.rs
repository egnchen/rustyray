use std::fmt::{Display, Formatter, Result};
use std::sync::Arc;

pub use material::Dielectric;
pub use material::LambertianDiffuse;
pub use material::Material;
pub use material::Metal;
pub use sphere::MovingSphere;
pub use sphere::Sphere;
pub use texture::CheckerTexture;
pub use texture::NoiseTexture;
pub use texture::SolidColor;
pub use texture::Texture;
pub use world::World;

use crate::object::aabb::AABB;
use crate::utils::{Ray, Vec3};

pub mod aabb;
pub mod bvh;

pub mod material;
pub mod texture;

pub mod cube;
pub mod rect;
pub mod sphere;

pub mod rotate;

pub mod constant_medium;
pub mod container;
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
    /// Calculate facing by direction of the light beam and normal vector.
    pub fn calc(p: &Vec3<f64>, r: &Ray) -> Face {
        if p.dot(r.direction()) < 0.0 {
            Face::Inward
        } else {
            Face::Outward
        }
    }
}

/// Internal structure to record an actual hit.
///
/// This structure contains the facing information(inward or outward), time of hit since the light
/// beam was shoot, hit point, normal vector & material at that point.
pub struct HitRecord {
    pub f: Face,
    pub t: f64,
    pub p: Vec3<f64>,
    pub u: f64,
    pub v: f64,
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
    /// Return the bounding box of the object.
    ///
    /// The bounding box is optional since some objects don't have a finite bounding box,
    /// like an infinite plane.
    fn bounding_box(&self) -> Option<&AABB>;
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// Thread-safe, read-only objects that implement `Hittable` trait
pub type HittableObject = Arc<dyn Hittable + Send + Sync>;
/// Thread-safe, read-only objects that implement `Material` trait
pub type MaterialObject = Arc<dyn Material + Send + Sync>;
/// Thread-safe, read-only objects that implement `Texture` trait
pub type TextureObject = Arc<dyn Texture + Send + Sync>;

pub fn make_material_object(m: impl Material + Send + Sync + 'static) -> MaterialObject {
    Arc::new(m)
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
    c1.x += height;
    Arc::new(MovingSphere::new(center, c1, t0, t1, radius, &mat))
}

pub fn make_texture_object(t: impl Texture + Send + Sync + 'static) -> TextureObject {
    Arc::new(t)
}
