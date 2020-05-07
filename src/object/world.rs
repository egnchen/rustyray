use std::fmt;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::object::aabb::AABB;
use crate::object::bvh::BVHNode;
use crate::object::{HitRecord, Hittable, HittableObject};
use crate::render::skybox::{ColorGradientSkyBox, SkyBox};
use crate::utils::{Color, Ray};

/// Object representing a world to render
/// Every object in the hittable list is read-only
/// to avoid RwLock overhead
pub struct World {
    hittable_list: Vec<HittableObject>,
    sky_box: Arc<dyn SkyBox + Send + Sync>,
    bvh: Option<Arc<BVHNode>>,
}

impl World {
    pub fn new() -> World {
        World {
            hittable_list: vec![],
            sky_box: Arc::new(ColorGradientSkyBox {
                v1: Color::new(1.0, 1.0, 1.0),
                v2: Color::new(0.5, 0.7, 1.0),
            }),
            bvh: None,
        }
    }

    pub fn update_bounding_box(&mut self) {
        if !self.hittable_list.is_empty() {
            self.bvh = Some(Arc::new(BVHNode::new(&mut self.hittable_list[..])));
        }
    }

    pub fn add_hittable(&mut self, h: &HittableObject) {
        self.hittable_list.push(Arc::clone(&h));
    }

    pub fn get_skybox(&self) -> &dyn SkyBox {
        self.sky_box.as_ref()
    }

    pub fn clear(&mut self) {
        self.hittable_list.clear();
    }
}

impl Hittable for World {
    fn bounding_box(&self) -> Option<&AABB> {
        match &self.bvh {
            Some(bb) => Some(&bb.bounding_box),
            None => None,
        }
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.bvh.as_ref().unwrap().hit(r, t_min, t_max)
    }
}
