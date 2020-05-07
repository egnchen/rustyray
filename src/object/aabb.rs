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
        let inv_d = 1.0 / d.0;
        t0 = (self.min.0 - o.0) * inv_d;
        t1 = (self.max.0 - o.0) * inv_d;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }
        let inv_d = 1.0 / d.1;
        t0 = (self.min.1 - o.1) * inv_d;
        t1 = (self.max.1 - o.1) * inv_d;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }
        let inv_d = 1.0 / d.2;
        t0 = (self.min.2 - o.2) * inv_d;
        t1 = (self.max.2 - o.2) * inv_d;
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
        v0.0 = if self.min.0 < b.min.0 {
            self.min.0
        } else {
            b.min.0
        };
        v1.0 = if self.max.0 > b.max.0 {
            self.max.0
        } else {
            b.max.0
        };
        v0.1 = if self.min.1 < b.min.1 {
            self.min.1
        } else {
            b.min.1
        };
        v1.1 = if self.max.1 > b.max.1 {
            self.max.1
        } else {
            b.max.1
        };
        v0.2 = if self.min.2 < b.min.2 {
            self.min.2
        } else {
            b.min.2
        };
        v1.2 = if self.max.2 > b.max.2 {
            self.max.2
        } else {
            b.max.2
        };
        AABB { min: v0, max: v1 }
    }
}
