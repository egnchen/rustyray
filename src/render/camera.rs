use rand::{Rng, thread_rng};

use crate::utils::{Ray, Vec3};

pub struct Camera {
    pub start_corner: Vec3<f64>,
    pub horizontal: Vec3<f64>,
    pub vertical: Vec3<f64>,
    pub origin: Vec3<f64>,
    lens_radius: f64,
    u: Vec3<f64>,
    v: Vec3<f64>,
    w: Vec3<f64>,
}

impl Camera {
    pub fn look_from(origin: Vec3<f64>, look_at: Vec3<f64>, v_up: Vec3<f64>,
                     vfov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect;

        let w = (look_at - origin).unit_vector();
        let u = w.cross(v_up).unit_vector();
        let v = u.cross(w);
        Camera {
            start_corner: origin + (w - u * half_width - v * half_height) * focus_dist,
            horizontal: u * (2.0 * half_width * focus_dist),
            vertical: v * (2.0 * half_height * focus_dist),
            origin,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    fn get_rand_unit_disk() -> Vec3<f64> {
        let theta = thread_rng().gen_range(0.0, 2.0 * std::f64::consts::PI);
        return Vec3(theta.cos(), theta.sin(), 0.0);
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let r = Self::get_rand_unit_disk() * self.lens_radius;
        let offset = self.u * r.x() + self.v * r.y();
        Ray {
            orig: self.origin + offset,
            dir: self.start_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        }
    }
}