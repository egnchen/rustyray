use std::sync::Arc;

use crate::object::aabb::AABB;
use crate::object::bvh::BVHNode;
use crate::object::container::Container;
use crate::object::{HitRecord, Hittable, HittableObject};
use crate::render::skybox::{ColorGradientSkyBox, SkyBox};
use crate::utils::{Color, Ray};

/// Object representing a world to render
/// Every object in the hittable list is read-only
/// to avoid RwLock overhead
pub struct World {
    sky_box: Arc<dyn SkyBox + Send + Sync>,
    container: Container,
}

impl World {
    pub fn new() -> World {
        World {
            container: Container::new(),
            sky_box: Arc::new(ColorGradientSkyBox {
                v1: Color::new(1.0, 1.0, 1.0),
                v2: Color::new(0.5, 0.7, 1.0),
            }),
        }
    }

    pub fn add_hittable(&mut self, h: &HittableObject) {
        self.container.add_hittable(h);
    }

    pub fn update_metadata(&mut self) {
        self.container.update_metadata();
    }

    pub fn get_skybox(&self) -> &dyn SkyBox {
        self.sky_box.as_ref()
    }

    pub fn set_skybox(&mut self, s: &Arc<dyn SkyBox + Send + Sync>) {
        self.sky_box = s.clone();
    }

    pub fn clear(&mut self) {
        self.container.clear();
    }
}

impl Hittable for World {
    #[inline(always)]
    fn bounding_box(&self) -> Option<&AABB> {
        self.container.bounding_box()
    }

    #[inline(always)]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.container.hit(r, t_min, t_max)
    }
}
