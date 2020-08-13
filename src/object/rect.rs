use crate::object::aabb::AABB;
use crate::object::{Face, HitRecord, Hittable, MaterialObject};
use crate::utils::{Ray, Vec3};

pub struct XYRect {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub z: f64,
    pub mat: MaterialObject,
    bounding_box: AABB,
}

impl XYRect {
    pub fn new(p0: (f64, f64), p1: (f64, f64), z: f64, mat: &MaterialObject) -> Self {
        XYRect {
            x0: p0.0,
            y0: p0.1,
            x1: p1.0,
            y1: p1.1,
            z,
            mat: mat.clone(),
            bounding_box: AABB {
                min: Vec3::new(p0.0, p0.1, z - 0.001),
                max: Vec3::new(p1.0, p1.1, z + 0.001),
            },
        }
    }
}

impl Hittable for XYRect {
    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.bounding_box)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.z - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut normal = Vec3::new(0.0, 0.0, 1.0);
        let f = Face::calc(&normal, r);
        if let Face::Outward = f {
            normal = -normal;
        }
        Some(HitRecord {
            f,
            t,
            p: r.at(t),
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            normal,
            mat: self.mat.clone(),
        })
    }
}
