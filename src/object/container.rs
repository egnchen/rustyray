use std::ops::Deref;

use crate::object::aabb::AABB;
use crate::object::bvh::BVHNode;
use crate::object::{HitRecord, Hittable, HittableObject};
use crate::utils::Ray;

pub struct Container {
    hittables: Vec<HittableObject>,
    bounding_box: Option<AABB>,
    bvh: Option<BVHNode>,
}

impl Container {
    pub fn new() -> Self {
        Container {
            hittables: vec![],
            bounding_box: None,
            bvh: None,
        }
    }

    pub fn add_hittable(&mut self, hittable: &HittableObject) {
        self.hittables.push(hittable.clone());
    }

    pub fn add_hittables(&mut self, hittables: &[HittableObject]) {
        self.hittables.extend_from_slice(hittables);
    }

    pub fn update_metadata(&mut self) {
        if self.hittables.is_empty() {
            self.bvh = None;
            self.bounding_box = None;
            return;
        }
        self.bvh = Some(BVHNode::new(&mut self.hittables));
        let mut aabb = AABB::default();
        for obj in &self.hittables {
            if let Some(b2) = obj.bounding_box() {
                aabb = aabb.union(b2);
            }
        }
        self.bounding_box = Some(aabb);
    }

    pub fn clear(&mut self) {
        self.hittables.clear();
        self.bvh = None;
        self.bounding_box = None;
    }
}

impl Hittable for Container {
    fn bounding_box(&self) -> Option<&AABB> {
        self.bounding_box.as_ref()
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match &self.bvh {
            Some(b) => b.hit(r, t_min, t_max),
            None => None,
        }
    }
}
