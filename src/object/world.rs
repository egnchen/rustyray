use std::borrow::Borrow;
use std::sync::{Arc, RwLock};

use crate::object::{HitRecord, Hittable, HittableObject};
use crate::utils::Ray;

/// Object representing a world to render
/// Every object in the hittable list is read-only
/// to avoid RwLock overhead
pub struct World {
    hittable_list: Vec<HittableObject>,
}

impl World {
    pub fn new() -> World {
        World {
            hittable_list: vec![],
        }
    }

    pub fn add_hittable(&mut self, h: &HittableObject) {
        self.hittable_list.push(Arc::clone(&h));
    }
    pub fn clear(&mut self) {
        self.hittable_list.clear();
    }
}

impl Hittable for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut cur_closest = t_max;
        let mut ret: Option<HitRecord> = None;
        for object in self.hittable_list.iter() {
            if let Some(cur) = object.hit(r, t_min, cur_closest) {
                if cur.t < cur_closest {
                    cur_closest = cur.t;
                    ret = Some(cur);
                }
            }
        }
        ret
    }
}
