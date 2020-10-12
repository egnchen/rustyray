use std::sync::Arc;

use crate::object::aabb::AABB;
use crate::object::bvh::BVHNode;
use crate::object::rect::{XYRect, XZRect, YZRect};
use crate::object::{HitRecord, Hittable, HittableObject, MaterialObject};
use crate::utils::{Ray, Vec3};

pub struct Cube {
    pub sides: [HittableObject; 6],
    pub mat: MaterialObject,
    bvh: Arc<BVHNode>,
    bounding_box: AABB,
}

impl Cube {
    pub fn new(p0: Vec3<f64>, p1: Vec3<f64>, mat: &MaterialObject) -> Self {
        assert!(p0.x <= p1.x && p0.y <= p1.y && p0.z <= p1.z);
        let mut sides: [HittableObject; 6] = [
            Arc::new(XYRect::new((p0.x, p0.y), (p1.x, p1.y), p0.z, mat)),
            Arc::new(XYRect::new((p0.x, p0.y), (p1.x, p1.y), p1.z, mat)),
            Arc::new(XZRect::new((p0.x, p0.z), (p1.x, p1.z), p0.y, mat)),
            Arc::new(XZRect::new((p0.x, p0.z), (p1.x, p1.z), p1.y, mat)),
            Arc::new(YZRect::new((p0.y, p0.z), (p1.y, p1.z), p0.x, mat)),
            Arc::new(YZRect::new((p0.y, p0.z), (p1.y, p1.z), p1.x, mat)),
        ];
        let bvh = Arc::new(BVHNode::new(&mut sides));
        Cube {
            sides,
            mat: mat.clone(),
            bvh,
            bounding_box: AABB { min: p0, max: p1 },
        }
    }
}

impl Hittable for Cube {
    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.bounding_box)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.bvh.hit(r, t_min, t_max)
    }
}
