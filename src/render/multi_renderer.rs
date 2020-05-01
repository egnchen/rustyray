use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::cmp::min;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time;

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

pub struct MultiRenderer {
    width: usize,
    height: usize,
    sample_per_unit: usize,
    recursion_depth: usize,
    camera: Arc<Option<Camera>>,
    world: Arc<Option<World>>,
    use_gamma_correction: bool,
    thread_count: usize,
}

impl MultiRenderer {
    pub fn new(width: usize, height: usize) -> MultiRenderer {
        MultiRenderer {
            width,
            height,
            camera: Arc::new(None),
            world: Arc::new(None),
            sample_per_unit: 128,
            recursion_depth: 16,
            use_gamma_correction: true,
            thread_count: num_cpus::get(),
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = Arc::new(Some(camera));
    }

    pub fn set_world(&mut self, world: World) {
        self.world = Arc::new(Some(world));
    }

    pub fn set_pixel_sample(&mut self, sample: usize) {
        self.sample_per_unit = sample;
    }

    // essentially the same as DefaultRenderer here
    fn ray_color(world: &World, r: Ray, depth: usize) -> Color {
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
        for _i in 0..depth {
            if let Some(h) = world.hit(&r, 0.001, f64::infinity()) {
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

impl Renderer for MultiRenderer {
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

        let t = time::SystemTime::now();

        // use scoped thread here
        let result = crossbeam::thread::scope(|s| -> Picture {
            println!("Initializing threads... Thread count = {}", self.thread_count);
            let mut p = Picture::new(self.width, self.height);

            let rx = {
                let (tx, rx) = mpsc::channel();
                let block_height = self.height / self.thread_count / 5;
                // divide job into blocks
                for thread_id in 0..self.thread_count {
                    let mut offset = thread_id * block_height;
                    let txc = tx.clone();
                    s.spawn(move |_| {
                        println!("Thread {} initiated", thread_id);
                        let mut rng = thread_rng();
                        let world = Option::as_ref(&self.world).unwrap();
                        let cam = Option::as_ref(&self.camera).unwrap();
                        let d1 = Uniform::from(0.0..(1.0 / self.height as f64));
                        let d2 = Uniform::from(0.0..(1.0 / self.width as f64));
                        // offset is copied into this closure
                        while offset < self.height {
                            let height = min(block_height, self.height - offset);
                            let mut buffer = Box::new(Picture::new(self.width, height));
                            for i in 0..height {
                                for j in 0..self.width {
                                    let mut c: Color = Color::default();
                                    let bv = (self.height - i - offset - 1) as f64 / self.height as f64;
                                    let bu = j as f64 / self.width as f64;
                                    for _k in 0..self.sample_per_unit {
                                        let v = bv + d1.sample(&mut rng);
                                        let u = bu + d2.sample(&mut rng);
                                        c += MultiRenderer::ray_color(world, cam.get_ray(u, v), self.recursion_depth);
                                    }
                                    c /= self.sample_per_unit as f64;
                                    buffer.data[i * self.width + j] = c;
                                }
                            }
                            if self.use_gamma_correction {
                                let filter = GammaFilter { gamma: 2.0 };
                                filter.filter(buffer.borrow_mut());
                            }
                            txc.send(PictureBuffer(offset, height, buffer)).unwrap();
                            offset += self.thread_count * block_height;
                        }
                        println!("Thread {} exit.", thread_id);
                    });
                }
                rx
            };   // tx at this point should be invalidated
            // receive buffers and copy them back
            for buf in rx.iter() {
                let offset = buf.0;
                let hmax = buf.1;
                println!("Received buffer, offset = {}, height = {}", offset, hmax);
                let content = &buf.2;
                for i in offset..(offset + hmax) {
                    for j in 0..self.width {
                        p.data[i * self.width + j] = content.data[(i - offset) * self.width + j];
                    }
                }
            }
            p
        });
        println!("Done, time elapsed = {:?}", time::SystemTime::now().duration_since(t).unwrap());
        if result.is_ok() {
            Ok(result.unwrap())
        } else {
            Err("Failed")
        }
    }
}
