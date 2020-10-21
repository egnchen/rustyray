use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

use crate::utils::{Ray, Vec3};

#[allow(dead_code)]
pub struct Camera {
    pub start_corner: Vec3<f64>,
    pub horizontal: Vec3<f64>,
    pub vertical: Vec3<f64>,
    pub origin: Vec3<f64>,
    lens_radius: f64,
    u: Vec3<f64>,
    v: Vec3<f64>,
    w: Vec3<f64>, // this field is never used, actually
    t_range: Uniform<f64>,
}

impl Camera {
    pub fn look_from(
        origin: Vec3<f64>,
        look_at: Vec3<f64>,
        v_up: Vec3<f64>,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
        t0: f64,
        t1: f64,
    ) -> Camera {
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
            t_range: Uniform::new(t0, t1),
        }
    }

    #[inline(always)]
    pub fn get_rand_in_unit_disk() -> Vec3<f64> {
        let mut rng = thread_rng();
        let dis = Uniform::new(-1.0, 1.0);
        loop {
            let p = Vec3::new(dis.sample(&mut rng), dis.sample(&mut rng), 0.0);
            if p.length_square() < 1.0 {
                return p;
            }
        }
    }

    #[inline(always)]
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let r = Self::get_rand_in_unit_disk() * self.lens_radius;
        let offset = self.u * r.x + self.v * r.y;
        let orig = self.origin + offset;
        let mut rng = thread_rng();
        Ray {
            orig,
            dir: self.start_corner + self.horizontal * u + self.vertical * v - orig,
            t: self.t_range.sample(&mut rng),
        }
    }

    pub fn get_aspect_ratio(&self) -> f64 {
        self.horizontal.length() / self.vertical.length()
    }
}
