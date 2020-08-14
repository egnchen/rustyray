//! BVH: Binary Volume Hierarchy

use std::cmp::Ordering;
use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::object::aabb::AABB;
use crate::object::{HitRecord, Hittable, HittableObject};
use crate::utils::Ray;

pub struct BVHNode {
    pub left: HittableObject,
    pub right: HittableObject,
    pub bounding_box: AABB,
}

impl BVHNode {
    pub fn new(hittable_list: &mut [HittableObject]) -> BVHNode {
        match hittable_list.len() {
            0 => panic!("Hittable list shouldn't be empty."),
            1 => {
                // Exception here: We use a single BVHNode, with two children being the same HittableObject.
                BVHNode {
                    left: hittable_list[0].clone(),
                    right: hittable_list[0].clone(),
                    bounding_box: hittable_list[0]
                        .bounding_box()
                        .expect("HittableObject not valid for AABB")
                        .clone(),
                }
            }
            _ => BVHNode::build_tree(hittable_list),
        }
    }

    /// Build the binary volume hierarchy for a hittable list length greater than 1
    fn build_tree(hittable_list: &mut [HittableObject]) -> BVHNode {
        // get the comparator and sort the list by it
        let comp = BVHNode::get_comparator();
        hittable_list.sort_by(comp);
        let (s1, s2) = hittable_list.split_at_mut(hittable_list.len() / 2);
        let left = if s1.len() == 1 {
            Arc::clone(&s1[0])
        } else {
            Arc::new(BVHNode::build_tree(s1))
        };
        let right = if s2.len() == 1 {
            Arc::clone(&s2[0])
        } else {
            Arc::new(BVHNode::build_tree(s2))
        };
        let bounding_box = left
            .bounding_box()
            .expect("HittableObject not valid for AABB.")
            .union(
                right
                    .bounding_box()
                    .expect("HittableObject not valid for AABB."),
            );
        BVHNode {
            left,
            right,
            bounding_box,
        }
    }

    fn get_comparator() -> fn(&HittableObject, &HittableObject) -> Ordering {
        match thread_rng().gen_range(0, 3) {
            0 => |a: &HittableObject, b: &HittableObject| {
                a.bounding_box()
                    .unwrap()
                    .min
                    .x
                    .partial_cmp(&b.bounding_box().unwrap().min.x)
                    .unwrap()
            },
            1 => |a: &HittableObject, b: &HittableObject| {
                a.bounding_box()
                    .unwrap()
                    .min
                    .y
                    .partial_cmp(&b.bounding_box().unwrap().min.y)
                    .unwrap()
            },
            _ => |a: &HittableObject, b: &HittableObject| {
                a.bounding_box()
                    .unwrap()
                    .min
                    .z
                    .partial_cmp(&b.bounding_box().unwrap().min.z)
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
                Some(rec1) => match self.right.hit(r, t_min, rec1.t) {
                    Some(rec2) => Some(rec2),
                    None => Some(rec1),
                },
                None => self.right.hit(r, t_min, t_max),
            }
        }
    }
}
