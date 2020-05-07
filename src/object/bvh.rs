//! BVH: Binary Volume Hierarchy

use std::cmp::Ordering;
use std::sync::Arc;

use image::DynamicImage;
use rand::{thread_rng, Rng};

use crate::object::aabb::AABB;
use crate::object::{HitRecord, Hittable, HittableObject};
use crate::utils::Ray;

pub struct BVHNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn new(hittable_list: &mut [Arc<dyn Hittable>]) -> BVHNode {
        // select a random axis as comparator
        let comp = BVHNode::get_comparator();
        let (left, right) = match hittable_list.len() {
            0 => panic!("error"),
            1 => (hittable_list[0].clone(), hittable_list[0].clone()),
            2 => match comp(&hittable_list[0], &hittable_list[1]) {
                Ordering::Equal | Ordering::Less => {
                    (hittable_list[0].clone(), hittable_list[1].clone())
                }
                Ordering::Greater => (hittable_list[1].clone(), hittable_list[0].clone()),
            },
            _ => {
                // sort the list by comparator
                hittable_list.sort_by(comp);
                let (s1, s2) = hittable_list.split_at_mut(hittable_list.len() / 2);
                (
                    Arc::new(BVHNode::new(s1)) as Arc<dyn Hittable>,
                    Arc::new(BVHNode::new(s2)) as Arc<dyn Hittable>,
                )
            }
        };
        let left_box = left
            .bounding_box()
            .expect("Missing bounding box for left node");
        let right_box = left
            .bounding_box()
            .expect("Missing bounding box for right node");
        let bounding_box = left_box.union(right_box);
        BVHNode {
            left,
            right,
            bounding_box,
        }
    }

    fn get_comparator() -> fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering {
        match thread_rng().gen_range(0, 3) {
            0 => |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
                a.bounding_box()
                    .unwrap()
                    .min
                    .0
                    .partial_cmp(&b.bounding_box().unwrap().min.0)
                    .unwrap()
            },
            1 => |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
                a.bounding_box()
                    .unwrap()
                    .min
                    .1
                    .partial_cmp(&b.bounding_box().unwrap().min.1)
                    .unwrap()
            },
            _ => |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
                a.bounding_box()
                    .unwrap()
                    .min
                    .2
                    .partial_cmp(&b.bounding_box().unwrap().min.2)
                    .unwrap()
            },
        }
    }
}

impl Hittable for BVHNode {
    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.bounding_box)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, t_min, t_max) {
            None
        } else {
            match self.left.hit(r, t_min, t_max) {
                Some(rec1) => match self.right.hit(r, rec1.t, t_max) {
                    Some(rec2) => Some(rec2),
                    None => Some(rec1),
                },
                None => self.right.hit(r, t_min, t_max),
            }
        }
    }
}
