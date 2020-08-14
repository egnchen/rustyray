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
        let inv_d = 1.0 / d.x;
        t0 = (self.min.x - o.x) * inv_d;
        t1 = (self.max.x - o.x) * inv_d;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }
        let inv_d = 1.0 / d.y;
        t0 = (self.min.y - o.y) * inv_d;
        t1 = (self.max.y - o.y) * inv_d;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }
        let inv_d = 1.0 / d.z;
        t0 = (self.min.z - o.z) * inv_d;
        t1 = (self.max.z - o.z) * inv_d;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }
        return true;
    }

    pub fn union(&self, b: &AABB) -> AABB {
        let mut v0: Vec3<f64> = Default::default();
        let mut v1: Vec3<f64> = Default::default();
        v0.x = if self.min.x < b.min.x {
            self.min.x
        } else {
            b.min.x
        };
        v1.x = if self.max.x > b.max.x {
            self.max.x
        } else {
            b.max.x
        };
        v0.y = if self.min.y < b.min.y {
            self.min.y
        } else {
            b.min.y
        };
        v1.y = if self.max.y > b.max.y {
            self.max.y
        } else {
            b.max.y
        };
        v0.z = if self.min.z < b.min.z {
            self.min.z
        } else {
            b.min.z
        };
        v1.z = if self.max.z > b.max.z {
            self.max.z
        } else {
            b.max.z
        };
        AABB { min: v0, max: v1 }
    }
}
