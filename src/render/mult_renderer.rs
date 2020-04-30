use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::cmp::min;
use std::sync::mpsc;
use std::thread;

use num_traits::float::FloatCore;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

use crate::io::{Color, Picture};
use crate::object::{Hittable, World};
use crate::render::{Camera, GammaFilter, Renderer};
use crate::render::filter::Filter;
use crate::utils::Ray;

/// multi-threaded renderer

struct PictureBuffer(usize, usize, Box<Picture>);

pub struct MultRenderer {
    width: usize,
    height: usize,
    sample_per_unit: usize,
    recursion_depth: usize,
    camera: Option<Camera>,
    world: Option<World>,
    use_gamma_correction: bool,
    thread_count: usize,
}

impl MultRenderer {
    pub fn new(width: usize, height: usize) -> MultRenderer {
        MultRenderer {
            width,
            height,
            camera: None,
            world: None,
            sample_per_unit: 128,
            recursion_depth: 16,
            use_gamma_correction: true,
            thread_count: num_cpus::get(),
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = Some(camera);
    }

    pub fn set_world(&mut self, world: World) {
        self.world = Some(world);
    }

    pub fn set_pixel_sample(&mut self, sample: usize) {
        self.sample_per_unit = sample;
    }

    // essentially the same as DefaultRenderer here
    fn ray_color(&self, r: Ray) -> Color {
        const LOW: Color = Color {
            0: 1.0,
            1: 1.0,
            2: 1.0,
        };
        const HIGH: Color = Color {
            0: 0.5,
            1: 0.7,
            2: 1.0,
        };

        // don't do tail-recursion :)
        // calculate
        let mut r = r;
        let mut ret = Color::one();
        let w = self.world.as_ref().unwrap();
        for _i in 0..self.recursion_depth {
            if let Some(h) = w.hit(&r, 0.001, f64::infinity()) {
                // hit something
                if let Some(f) = h.mat.scatter(&r, &h) {
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

impl Renderer for MultRenderer {
    fn render(&self) -> Result<Picture, &'static str> {
        if self.world.is_none() {
            return Err("World not set.");
        }
        if self.camera.is_none() {
            return Err("Camera not set.");
        }
        println!(
            "Configuration: Picture size = {} * {}, sample = {}, recursion depth = {}",
            self.width, self.height, self.sample_per_unit, self.recursion_depth
        );

        // divide job into blocks
        let block_height = (self.height + self.thread_count) / self.thread_count;
        let mut offset: usize = 0;
        let (tx, rx) = mpsc::channel();
        while offset < self.height {
            let txc = tx.clone();
            let hmax = min(block_height, self.height - offset);
            thread::spawn(move || {
                let mut buffer = Box::new(Picture::new(self.width, hmax));
                let p: &mut Picture = buffer.borrow_mut();
                let cam = self.camera.as_ref().unwrap();
                let mut rng = thread_rng();
                let d1 = Uniform::from(0.0..(1.0 / self.height as f64));
                let d2 = Uniform::from(0.0..(1.0 / self.width as f64));
                for i in 0..hmax {
                    for j in 0..self.width {
                        let mut c: Color = Color::default();
                        let bv = (self.height - i - offset - 1) as f64 / self.height as f64;
                        let bu = j as f64 / self.width as f64;
                        for _k in 0..self.sample_per_unit {
                            let v = bv + d1.sample(&mut rng);
                            let u = bu + d2.sample(&mut rng);
                            c += self.ray_color(cam.get_ray(u, v));
                        }
                        c /= self.sample_per_unit as f64;
                        p.data[i * self.width + j] = c;
                    }
                }
                if self.use_gamma_correction {
                    let filter = GammaFilter { gamma: 2.0 };
                    filter.filter(p);
                }
                txc.send(PictureBuffer(offset, hmax, buffer)).unwrap();
            });
            offset += hmax;
        }
        let mut p = Picture::new(self.width, self.height);
        for buf in rx.iter() {
            let offset = buf.0;
            let hmax = buf.1;
            let content = &buf.2;
            for i in offset..(offset + hmax) {
                for j in 0..self.width {
                    p.data[i * self.width + j] = content.data[(i - offset) * self.width + j];
                }
            }
        }

        Ok(p)
    }
}
