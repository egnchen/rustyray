use crate::vec::Vec3;
use crate::world::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::world::Face;

pub struct Sphere {
    pub center: Vec3<f64>,
    pub radius: f64,
}

/// calculate if a ray will hit a sphere
/// $$t^2 \vec{\mathbf{b}}\cdot\vec{\mathbf{b}}
//      + 2t \vec{\mathbf{b}} \cdot \vec{(\mathbf{a}-\mathbf{c})}
//      + \vec{(\mathbf{a}-\mathbf{c})} \cdot \vec{(\mathbf{a}-\mathbf{c})} - R^2 = 0$$
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t1 = r.origin() - self.center;
        let t2 = r.direction();

        let a = t2.length_square();
        let half_b = t1.dot(t2);
        let c = t1.length_square() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let t = (-half_b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.at(t);
                return Some(HitRecord {
                    f: Face::calc(&p, &r),
                    t,
                    p,
                    normal: (p - self.center).unit_vector(),
                });
            }
            let t = (-half_b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.at(t);
                return Some(HitRecord {
                    f: Face::calc(&p, &r),
                    p,
                    t,
                    normal: (p - self.center).unit_vector(),
                });
            }
        }
        None
    }
}