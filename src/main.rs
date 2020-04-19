use ray_tracer::ppm::{Picture, Color};
use ray_tracer::vec::Vec3;
use ray_tracer::ray::Ray;
use ray_tracer::world::{Sphere, World, Hittable};
use std::rc::Rc;
use std::cell::RefCell;
use num_traits::float::FloatCore;
use ray_tracer::render::{Camera, GammaFilter};
use rand::Rng;
use indicatif::ProgressBar;
use ray_tracer::render::filter::Filter;

/// return a vector pointing to a random direction
/// within a unit sphere
/// The distribution used here is Lambertian distribution
/// which has a distribution of cos(\phi)
fn rand_in_unit_sphere(rng: &mut impl Rng) -> Vec3<f64> {
    let a: f64 = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
    let z: f64 = rng.gen_range(-1.0, 1.0);
    let r: f64 = (1.0 - z * z).sqrt();
    Vec3(r * a.cos(), r * a.sin(), z)
}

fn ray_color(r: &Ray, w: &World, rng: &mut impl Rng, depth: u8) -> Color {
    const LOW: Color = Color{0: 1.0, 1: 1.0, 2: 1.0};
    const HIGH: Color = Color{0: 0.5, 1: 0.7, 2: 1.0};
    if depth == 0 {
        return Color::zero();
    }
    if let Some(t) = w.hit(&r, 0.001, f64::infinity()) {
        // it hit something
        // assume diffuse material here
        let new_ray = Ray {
            orig: t.p,
            dir: t.normal + rand_in_unit_sphere(rng)
        };
        ray_color(&new_ray, w, rng, depth - 1) * 0.5
    } else {
        // sky box
        let unit = r.direction().unit_vector();
        let t = 0.5 * (unit.y() + 1.0) as f64;
        LOW * (1.0 - t) + HIGH * t
    }
}

fn main() {
    let width = 200;
    let height = 100;

    let mut p = Picture::new(width, height);

    let origin = Vec3(0.0, 0.0, 0.0);
    let viewport_start = Vec3(-2.0, -1.0, -1.0);
    let hlength = 4.0;
    let vlength = 2.0;

    // initialize the world
    let mut world = World::new();
    let camera = Camera::new(origin, viewport_start, hlength, vlength);
    let sphere1: Rc<RefCell<dyn Hittable>> = Rc::from(RefCell::new(Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5
    }));
    let sphere2: Rc<RefCell<dyn Hittable>> = Rc::from(RefCell::new(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0
    }));
    world.add_hittable(&sphere1);
    world.add_hittable(&sphere2);

    let sample_per_pixel = 128;
    let mut rng = rand::thread_rng();

    let pb = ProgressBar::new(height as u64);

    // neg-y axis is i, pos-x axis is j
    for i in 0..height {
        for j in 0..width {
            let mut c: Color = Color::default();
            for _k in 0..sample_per_pixel {
                let v = (rng.gen::<f64>() + (height - i - 1) as f64) / height as f64;
                let u = (rng.gen::<f64>() + j as f64) / width as f64;
                c += ray_color(&camera.get_ray(u, v), &world, &mut rng,64);
            }
            c /= sample_per_pixel as f64;
            p.data[(i * width + j) as usize] = c;
        }
        pb.inc(1);
    }

    pb.finish();
    println!("Undergoing gamma correction...");
    let filter = GammaFilter { gamma: 2.0 };
    filter.filter(&mut p);
    println!("Writing to out.ppm...");
    p.write_to_file("out.ppm").expect("Failed to write file.");
}
