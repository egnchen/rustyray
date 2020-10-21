use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::object::aabb::AABB;
use crate::object::material::Isotropic;
use crate::object::{Face, HitRecord, Hittable, HittableObject, MaterialObject, TextureObject};
use crate::utils::{Ray, Vec3};

pub struct ConstantMedium {
    boundary: HittableObject,
    phase_function: MaterialObject,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: &HittableObject, d: f64, t: &TextureObject) -> Self {
        ConstantMedium {
            boundary: boundary.clone(),
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic { albedo: t.clone() }),
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self) -> Option<&AABB> {
        self.boundary.bounding_box()
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        const ENABLE_DEBUGGING: bool = false;
        let debugging: bool = ENABLE_DEBUGGING && thread_rng().gen::<f64>() < 0.0001;

        // hit twice to calc enter & exit distance
        if let Some(rec1) = self
            .boundary
            .hit(r, std::f64::NEG_INFINITY, std::f64::INFINITY)
        {
            if let Some(rec2) = self.boundary.hit(r, rec1.t + 0.0001, std::f64::INFINITY) {
                if debugging {
                    println!("CONSTANT MEDIUM HIT, rec1.t={}, rec2.t={}", rec1.t, rec2.t)
                }
                let t1 = rec1.t.max(t_min).max(0.0);
                let t2 = rec2.t.min(t_max);
                if t1 > t2 {
                    return None;
                }
                let ray_v = r.dir.length();
                let distance_within = ray_v * (t2 - t1);
                let hit_distance =
                    self.neg_inv_density * (thread_rng().gen_range(0.0_f64, 1.0_f64).log2());
                if hit_distance > distance_within {
                    return None;
                }
                let t = rec1.t + hit_distance / ray_v;
                let rec = HitRecord {
                    f: Face::Outward,
                    t,
                    p: r.at(t),
                    u: 0.0,
                    v: 0.0,
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    mat: self.phase_function.clone(),
                };
                if debugging {
                    println!(
                        "hit_distance = {}, rec.t = {}, rec.p = {}",
                        hit_distance, rec.t, rec.p
                    );
                }
                Some(rec)
            } else {
                None
            }
        } else {
            None
        }
    }
}
