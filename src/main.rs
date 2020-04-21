use std::cell::RefCell;
use std::rc::Rc;
use std::time;

use indicatif::ProgressBar;
use num_traits::float::FloatCore;
use rand::Rng;

use ray_tracer::io::Picture;
use ray_tracer::io::ppm::Color;
use ray_tracer::object::{Hittable, Sphere, World};
use ray_tracer::object::material::{Dielectric, LambertianDiffuse, Material, Metal};
use ray_tracer::render::{Camera, GammaFilter};
use ray_tracer::render::filter::Filter;
use ray_tracer::utils::{Ray, Vec3};

fn ray_color(r: &Ray, w: &World, depth: u8) -> Color {
    const LOW: Color = Color { 0: 1.0, 1: 1.0, 2: 1.0 };
    const HIGH: Color = Color { 0: 0.5, 1: 0.7, 2: 1.0 };
    // don't do tail-recursion :)
    // calculate sky-box color first
    let unit = r.direction().unit_vector();
    let t = 0.5 * (unit.y() + 1.0) as f64;
    let mut ret = LOW * (1.0 - t) + HIGH * t;

    // here we go
    let mut ray = *r;
    let mut trace= vec![];
    for _i in 0..depth {
        if let Some(h) = w.hit(&ray, 0.001, f64::infinity()) {
            // it hit something
            if let Some(f) = RefCell::borrow(&h.mat).scatter(&ray, &h) {
                ret *= f.attenuation;
                ray = f.scattered;
                trace.push((h.f, ray));
            } else {
                return Color::zero();
            }
        } else {
            return ret;
        }
    }
    // println!("Depth reached.");
    // for r in trace.iter() {
    //     println!("{:?}", r);
    // }
    Color::zero()
}

fn main() {
    let width = 300;
    let height = 150;

    let mut p = Picture::new(width, height);

    let origin = Vec3(0.0, 0.0, 0.0);
    let viewport_start = Vec3(-2.0, -1.0, -1.0);
    let hlength = 4.0;
    let vlength = 2.0;

    // initialize the object
    let mut world = World::new();
    let camera = Camera::new(origin, viewport_start, hlength, vlength);

    // materials
    let mat1: Rc<RefCell<dyn Material>> = Rc::from(RefCell::new(LambertianDiffuse {
        albedo: Vec3(0.6, 0.6, 0.8),
    }));
    let mat2: Rc<RefCell<dyn Material>> = Rc::from(RefCell::new(Metal {
        albedo: Vec3(0.7, 0.7, 0.7),
        fuzziness: 0.0,
    }));
    let mat3: Rc<RefCell<dyn Material>> = Rc::from(RefCell::new(Dielectric::new(1.5)));

    let sphere1: Rc<RefCell<dyn Hittable>> = Rc::from(RefCell::new(Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
        mat: Rc::clone(&mat1),
    }));
    let sphere2: Rc<RefCell<dyn Hittable>> = Rc::from(RefCell::new(Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat: Rc::clone(&mat2),
    }));
    let sphere3: Rc<RefCell<dyn Hittable>> = Rc::from(RefCell::new(Sphere {
        center: Vec3(1.0, 0.0, -1.0),
        radius: 0.5,
        mat: Rc::clone(&mat3),
    }));

    let sphere_ground: Rc<RefCell<dyn Hittable>> = Rc::from(RefCell::new(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        mat: Rc::clone(&mat1),
    }));
    world.add_hittable(&sphere1);
    world.add_hittable(&sphere2);
    world.add_hittable(&sphere3);
    world.add_hittable(&sphere_ground);

    let sample_per_pixel = 256;
    let mut rng = rand::thread_rng();

    let pb = ProgressBar::new(height as u64);
    let t = time::SystemTime::now();
    // neg-y axis is i, pos-x axis is j
    for i in 0..height {
        for j in 0..width {
            let mut c: Color = Color::default();
            for _k in 0..sample_per_pixel {
                let v = (rng.gen::<f64>() + (height - i - 1) as f64) / height as f64;
                let u = (rng.gen::<f64>() + j as f64) / width as f64;
                c += ray_color(&camera.get_ray(u, v), &world, 64);
            }
            c /= sample_per_pixel as f64;
            p.data[(i * width + j) as usize] = c;
        }
        pb.inc(1);
    }

    pb.finish();
    println!("Finished, time = {}ms.",
             time::SystemTime::now().duration_since(t).unwrap().as_millis());
    println!("Doing gamma correction...");
    let filter = GammaFilter { gamma: 2.0 };
    filter.filter(&mut p);
    println!("Writing to out.ppm...");
    p.write_to_file("out.ppm").expect("Failed to write file.");
}
