use crate::object::aabb::AABB;
use crate::object::{Face, HitRecord, Hittable, HittableObject};
use crate::utils::{Ray, Vec3};

pub struct RotateY {
    pub hittable: HittableObject,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<AABB>,
}

impl RotateY {
    pub fn new(hittable: &HittableObject, angle: f64) -> Self {
        // construct the new bounding box
        let radians = angle * std::f64::consts::PI / 180.0;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let new_bbox = if let Some(b) = hittable.bounding_box() {
            let mut min = Vec3::<f64>::one() * std::f64::INFINITY;
            let mut max = Vec3::<f64>::one() * std::f64::INFINITY;
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * b.max.x + (1 - i) as f64 * b.min.x;
                        let y = j as f64 * b.max.y + (1 - j) as f64 * b.min.y;
                        let z = k as f64 * b.max.z + (1 - k) as f64 * b.min.z;

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let v = Vec3::new(new_x, y, new_z);
                        min.x = min.x.min(v.x);
                        min.y = min.y.min(v.y);
                        min.z = min.z.min(v.z);
                        max.x = max.x.max(v.x);
                        max.y = max.y.max(v.y);
                        max.z = max.z.max(v.z);
                    }
                }
            }
            Some(AABB { min, max })
        } else {
            None
        };
        RotateY {
            hittable: hittable.clone(),
            sin_theta,
            cos_theta,
            bounding_box: new_bbox,
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self) -> Option<&AABB> {
        self.bounding_box.as_ref()
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut orig = r.orig;
        let mut dir = r.dir;
        orig.x = self.cos_theta * r.orig.x - self.sin_theta * r.orig.z;
        orig.z = self.sin_theta * r.orig.x + self.cos_theta * r.orig.z;
        dir.x = self.cos_theta * r.dir.x - self.sin_theta * r.dir.z;
        dir.z = self.sin_theta * r.dir.x + self.cos_theta * r.dir.z;
        let nr = Ray { orig, dir, t: r.t };
        if let Some(mut record) = self.hittable.hit(&nr, t_min, t_max) {
            let mut p = record.p;
            let mut normal = record.normal;
            p.x = self.cos_theta * record.p.x + self.sin_theta * record.p.z;
            p.z = -self.sin_theta * record.p.x + self.cos_theta * record.p.z;
            normal.x = self.cos_theta * record.normal.x + self.sin_theta * record.normal.z;
            normal.z = -self.sin_theta * record.normal.x + self.cos_theta * record.normal.z;
            record.p = p;
            record.normal = normal;
            record.f = Face::calc(&p, &nr);
            Some(record)
        } else {
            None
        }
    }
}
