use std::sync::Arc;

use crate::object::{HitRecord, Hittable, HittableObject};
use crate::render::skybox::{ColorGradientSkyBox, SkyBox};
use crate::utils::{Color, Ray};

/// Object representing a world to render
/// Every object in the hittable list is read-only
/// to avoid RwLock overhead
pub struct World {
    hittable_list: Vec<HittableObject>,
    sky_box: Arc<dyn SkyBox + Send + Sync>,
}

impl World {
    pub fn new() -> World {
        World {
            hittable_list: vec![],
            sky_box: Arc::new(ColorGradientSkyBox {
                v1: Color::new(1.0, 1.0, 1.0),
                v2: Color::new(0.5, 0.7, 1.0),
            }),
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
