use std::cell::RefCell;

use indicatif::ProgressBar;
use num_traits::float::FloatCore;
use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Uniform};

use crate::io::{Color, Picture};
use crate::object::{Hittable, World};
use crate::render::{Camera, GammaFilter, Renderer};
use crate::render::filter::Filter;
use crate::utils::Ray;

pub struct DefaultRenderer {
    width: usize,
    height: usize,
    sample_per_unit: usize,
    recursion_depth: usize,
    camera: Option<Camera>,
    world: Option<World>,
    use_gamma_correction: bool,
}

impl DefaultRenderer {
    pub fn new(width: usize, height: usize) -> DefaultRenderer {
        DefaultRenderer {
            width,
            height,
            camera: None,
            world: None,
            sample_per_unit: 128,
            recursion_depth: 16,
            use_gamma_correction: true,
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = Some(camera);
    }

    pub fn set_world(&mut self, world: World) {
        self.world = Some(world);
    }

    pub fn set_pixel_sample(&mut self, sample: usize) { self.sample_per_unit = sample; }

    fn ray_color(&self, r: Ray) -> Color {
        const LOW: Color = Color { 0: 1.0, 1: 1.0, 2: 1.0 };
        const HIGH: Color = Color { 0: 0.5, 1: 0.7, 2: 1.0 };

        // don't do tail-recursion :)
        // calculate
        let mut r = r;
        let mut ret = Color::one();
        let w = self.world.as_ref().unwrap();
        for _i in 0..self.recursion_depth {
            if let Some(h) = w.hit(&r, 0.001, f64::infinity()) {
                // hit something
                if let Some(f) = RefCell::borrow(&h.mat).scatter(&r, &h) {
                    ret *= f.attenuation;
                    r = f.scattered;
                } else {
                    return Color::zero();
                }
            } else {
                // sky box
                let unit = r.direction().unit_vector();
                let t = 0.5 * (unit.y() + 1.0);
                return ret * (LOW * (1.0 - t) + HIGH * t);
            }
        }
        Color::zero()
    }
}

impl Renderer for DefaultRenderer {
    fn render(&self) -> Result<Picture, &'static str> {
        if self.world.is_none() {
            return Err("World not set.");
        }
        let cam = self.camera.as_ref().expect("Camera not set.");
        let mut rng = thread_rng();
        let mut p = Picture::new(self.width, self.height);
        let pb = ProgressBar::new(self.height as u64);
        println!("Configuration: Picture size {} * {}, sample = {}, recursion depth = {}",
                 self.width, self.height, self.sample_per_unit, self.recursion_depth);
        let d1 = Uniform::from(0.0..(1.0 / self.height as f64));
        let d2 = Uniform::from(0.0..(1.0 / self.width as f64));
        for i in 0..self.height {
            for j in 0..self.width {
                let mut c: Color = Color::default();
                let bv = (self.height - i - 1) as f64 / self.height as f64;
                let bu = j as f64 / self.width as f64;
                for _k in 0..self.sample_per_unit {
                    let v = bv + d1.sample(&mut rng);
                    let u = bu + d2.sample(&mut rng);
                    c += self.ray_color(cam.get_ray(u, v));
                }
                c /= self.sample_per_unit as f64;
                p.data[(i * self.width + j) as usize] = c;
            }
            pb.inc(1);
        }
        if self.use_gamma_correction {
            let filter = GammaFilter { gamma: 2.0 };
            filter.filter(&mut p);
        }
        pb.finish_and_clear();
        Ok(p)
    }
}