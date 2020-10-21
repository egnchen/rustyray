use std::sync::{mpsc, Arc};
use std::time;

use num_traits::float::FloatCore;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

use crate::object::{Hittable, World};
use crate::render::filter::Filter;
use crate::render::multi_renderer::PresetLevel::{Low, Medium};
use crate::render::{Camera, GammaFilter, Renderer};
use crate::utils::{Color, Picture, Ray};

/// multi-threaded renderer
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

pub enum PresetLevel {
    Low,
    Medium,
    High,
    Ultra,
}

impl PresetLevel {
    pub fn from(num: usize) -> Option<Self> {
        match num {
            0 => Some(PresetLevel::Low),
            1 => Some(PresetLevel::Medium),
            2 => Some(PresetLevel::High),
            3 => Some(PresetLevel::Ultra),
            _ => None,
        }
    }
}

impl MultiRenderer {
    pub fn new() -> MultiRenderer {
        // use low preset for default
        MultiRenderer {
            width: 128,
            height: 128,
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

    pub fn set_thread_count(&mut self, thread_count: usize) {
        self.thread_count = thread_count;
    }

    pub fn set_recursion_depth(&mut self, depth: usize) {
        self.recursion_depth = depth;
    }

    pub fn set_render_preset(&mut self, preset: PresetLevel) {
        let aspect = Option::as_ref(&self.camera).unwrap().get_aspect_ratio();
        match preset {
            PresetLevel::Low => {
                self.sample_per_unit = 128;
                self.height = 128;
            }
            PresetLevel::Medium => {
                self.sample_per_unit = 512;
                self.height = 256;
            }
            PresetLevel::High => {
                self.sample_per_unit = 1024;
                self.height = 512;
            }
            PresetLevel::Ultra => {
                self.sample_per_unit = 8192;
                self.height = 1024;
            }
        }
        self.width = (self.height as f64 * aspect) as usize;
    }

    // essentially the same as DefaultRenderer here
    fn ray_color(world: &World, r: Ray, depth: usize) -> Color {
        // don't do tail-recursion :)
        // calculate
        let mut r = r;
        let mut coeff = Color::one();
        let mut ret = Color::zero();
        for _i in 0..depth {
            if let Some(h) = world.hit(&r, 0.001, f64::infinity()) {
                // emission material
                ret += coeff * h.mat.emit(h.u, h.v, h.p);
                if let Some(f) = h.mat.scatter(&r, &h) {
                    // scattering material
                    coeff *= f.attenuation;
                    r = f.scattered;
                } else {
                    // no more scattering, return value :)
                    return ret;
                }
            } else {
                // sky box
                return ret + coeff * world.get_skybox().get_color(&r);
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
            println!(
                "Initializing threads... Thread count = {}",
                self.thread_count
            );
            let _p = Picture::new(self.width, self.height);
            let sample_per_thread =
                (self.sample_per_unit + self.thread_count - 1) / self.thread_count;
            let rx = {
                let (tx, rx) = mpsc::channel();
                // divide the job by sample count per pixel
                for thread_id in 0..self.thread_count {
                    let txc = tx.clone();
                    s.spawn(move |_| {
                        println!("Thread {} initiated", thread_id);
                        let mut rng = thread_rng();
                        let world = Option::as_ref(&self.world).unwrap();
                        let cam = Option::as_ref(&self.camera).unwrap();
                        let d1 = Uniform::from(0.0..(1.0 / self.height as f64));
                        let d2 = Uniform::from(0.0..(1.0 / self.width as f64));
                        let mut buffer = Box::new(Picture::new(self.width, self.height));
                        for i in 0..self.height {
                            for j in 0..self.width {
                                let mut c: Color = Color::default();
                                let bv = (self.height - i - 1) as f64 / self.height as f64;
                                let bu = j as f64 / self.width as f64;
                                for _k in 0..sample_per_thread {
                                    let v = bv + d1.sample(&mut rng);
                                    let u = bu + d2.sample(&mut rng);
                                    c += MultiRenderer::ray_color(
                                        world,
                                        cam.get_ray(u, v),
                                        self.recursion_depth,
                                    );
                                }
                                buffer.data[i * self.width + j] = c;
                            }
                        }
                        txc.send(buffer).expect("Buffer exchanging failed.");
                        println!("Thread {} exit.", thread_id);
                    });
                }
                rx
            }; // tx at this point should be invalidated
            let mut buffer = Picture::new(self.width, self.height);
            for buf in rx {
                for (u, v) in buffer.data.iter_mut().zip(buf.data.iter()) {
                    *u += *v;
                }
            }
            for u in &mut buffer.data {
                *u /= self.sample_per_unit as f32;
            }
            buffer
        });
        if result.is_err() {
            return Err("Error occurred during multi-threaded rendering.");
        }
        let mut buffer = result.unwrap();
        if self.use_gamma_correction {
            let filter = GammaFilter { gamma: 2.0 };
            filter.filter(&mut buffer);
        }
        println!(
            "Done, time elapsed = {:?}",
            time::SystemTime::now().duration_since(t).unwrap()
        );
        Ok(buffer)
    }
}
