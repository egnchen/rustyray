use std::rc::Rc;
use std::cell::RefCell;

use super::Hittable;
use crate::ray::Ray;
use crate::shape::HitRecord;

pub struct World {
    hittable_list: Vec<Rc<RefCell<dyn Hittable>>>,
}

impl World {
    pub fn new() ->World { World { hittable_list: vec![]} }

    pub fn add_hittable(&mut self, h: &Rc<RefCell<dyn Hittable>>) {
        self.hittable_list.push(Rc::clone(&h));
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
            if let Some(cur) = object.borrow().hit(r, t_min, cur_closest) {
                if cur.t < cur_closest {
                    cur_closest = cur.t;
                    ret = Some(cur);
                }
            }
        }
        ret
    }
}