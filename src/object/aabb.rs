use std::mem::swap;

use crate::utils::{Ray, Vec3};

#[derive(Default, Clone)]
pub struct AABB {
    pub min: Vec3<f64>,
    pub max: Vec3<f64>,
}

impl AABB {
    #[inline(always)]
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        let mut t0;
        let mut t1;
        // TODO improve this
        // One cannot iterate over tuple, so manually unroll the loop here...
        let d = &r.dir;
        let o = &r.orig;
        t0 = (self.min.x - o.x) / d.x;
        t1 = (self.max.x - o.x) / d.x;
        if d.x < 0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max <= t_min {
            return false;
        }
        t0 = (self.min.y - o.y) / d.y;
        t1 = (self.max.y - o.y) / d.y;
        if d.y < 0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max <= t_min {
            return false;
        }
        t0 = (self.min.z - o.z) / d.z;
        t1 = (self.max.z - o.z) / d.z;
        if d.z < 0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max <= t_min {
            return false;
        }
        return true;
    }

    #[inline(always)]
    pub fn union(&self, b: &AABB) -> AABB {
        let v0 = Vec3::new(
            self.min.x.min(b.min.x),
            self.min.y.min(b.min.y),
            self.min.z.min(b.min.z),
        );
        let v1 = Vec3::new(
            self.max.x.max(b.max.x),
            self.max.y.max(b.max.y),
            self.max.z.max(b.max.z),
        );
        AABB { min: v0, max: v1 }
    }
}
