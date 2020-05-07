use std::sync::Arc;

use crate::object::aabb::AABB;
use crate::object::Face;
use crate::object::{HitRecord, Hittable, MaterialObject};
use crate::utils::{Ray, Vec3};

pub struct Sphere {
    pub center: Vec3<f64>,
    pub radius: f64,
    pub mat: MaterialObject,
    bounding_box: AABB,
}

impl Sphere {
    pub(crate) fn new(center: Vec3<f64>, radius: f64, mat: &MaterialObject) -> Sphere {
        Sphere {
            center,
            radius,
            mat: mat.clone(),
            bounding_box: AABB {
                // element-wise operations :)
                min: center - radius,
                max: center + radius,
            },
        }
    }
}

/// calculate if a ray will hit a sphere
/// $$t^2 \vec{\mathbf{b}}\cdot\vec{\mathbf{b}}
///     + 2t \vec{\mathbf{b}} \cdot \vec{(\mathbf{a}-\mathbf{c})}
///     + \vec{(\mathbf{a}-\mathbf{c})} \cdot \vec{(\mathbf{a}-\mathbf{c})} - R^2 = 0$$
impl Hittable for Sphere {
    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.bounding_box)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t1 = r.origin() - self.center;
        let t2 = r.direction();

        let a = t2.length_square();
        let half_b = t1.dot(t2);
        let c = t1.length_square() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        return if discriminant > 0.0 {
            let mut t = (-half_b - discriminant.sqrt()) / a;
            if t > t_max || t < t_min {
                // another root
                t += 2.0 * discriminant.sqrt() / a;
                if t > t_max || t < t_min {
                    return None;
                }
            }
            let p = r.at(t);
            let mut normal = (p - self.center) / self.radius;
            let f = Face::calc(&normal, &r);
            if let Face::Outward = f {
                normal = -normal;
            }
            Some(HitRecord {
                f,
                t,
                p,
                normal,
                mat: Arc::clone(&self.mat),
            })
        } else {
            None
        };
    }
}

/// a moving sphere
pub struct MovingSphere {
    pub c0: Vec3<f64>,
    pub c1: Vec3<f64>,
    pub t0: f64,
    pub t1: f64,
    pub radius: f64,
    pub mat: MaterialObject,
    bounding_box: AABB,
}

impl MovingSphere {
    pub fn new(
        c0: Vec3<f64>,
        c1: Vec3<f64>,
        t0: f64,
        t1: f64,
        radius: f64,
        mat: &MaterialObject,
    ) -> MovingSphere {
        // calculate bounding box
        let b0 = AABB {
            min: c0 - radius,
            max: c0 + radius,
        };
        let b1 = AABB {
            min: c1 - radius,
            max: c1 + radius,
        };
        MovingSphere {
            c0,
            c1,
            t0,
            t1,
            radius,
            mat: mat.clone(),
            bounding_box: b0.union(&b1),
        }
    }

    fn center(&self, t: f64) -> Vec3<f64> {
        self.c0 + (self.c1 - self.c0) * ((t - self.t0) / (self.t1 - self.t0))
    }
}

impl Hittable for MovingSphere {
    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.bounding_box)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let c = self.center(r.time());
        let t1 = r.origin() - c;
        let t2 = r.direction();

        let a = t2.length_square();
        let half_b = t1.dot(t2);
        let c = t1.length_square() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        return if discriminant > 0.0 {
            let mut t = (-half_b - discriminant.sqrt()) / a;
            if t > t_max || t < t_min {
                // another root
                t += 2.0 * discriminant.sqrt() / a;
                if t > t_max || t < t_min {
                    return None;
                }
            }
            let p = r.at(t);
            let mut normal = (p - c) / self.radius;
            let f = Face::calc(&normal, &r);
            if let Face::Outward = f {
                normal = -normal;
            }
            Some(HitRecord {
                f,
                t,
                p,
                normal,
                mat: Arc::clone(&self.mat),
            })
        } else {
            None
        };
    }
}
