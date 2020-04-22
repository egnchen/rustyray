use std::cell::RefCell;
use std::rc::Rc;
use std::time;

use rand::{Rng, thread_rng};

use ray_tracer::object::{Hittable, Sphere, World};
use ray_tracer::object::material::{Dielectric, LambertianDiffuse, Material, Metal};
use ray_tracer::render::{Camera, Renderer};
use ray_tracer::render::renderer::DefaultRenderer;
use ray_tracer::utils::Vec3;

/// initialize camera with default parameters
fn init_camera() -> Camera {
    let look_from = Vec3(13.0, 2.0, 4.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    Camera::look_from(look_from, look_at, Vec3(0.0, 1.0, 0.0),
                      20.0, 1.5, 0.05, (look_at - look_from).length())
}

/// initialize the world, fill it with objects
fn init_world() -> World {
    let mut world = World::new();

    let mat_ground: Rc<RefCell<dyn Material>> = Rc::from(RefCell::new(LambertianDiffuse {
        albedo: Vec3(0.7, 0.7, 0.7),
    }));
    let sphere_ground: Rc<RefCell<dyn Hittable>> = Rc::from(RefCell::new(Sphere {
        center: Vec3(0.0, -1000.0, -1.0),
        radius: 1000.0,
        mat: Rc::clone(&mat_ground),
    }));
    world.add_hittable(&sphere_ground);

    let mut rng = thread_rng();
    for i in -5..=10 {
        for j in -4..=4 {
            if j == 0 {
                continue
            }
            let center = Vec3(i as f64 * 1.2 + rng.gen_range(-0.5, 0.5),
                              0.25,
                              j as f64 * 1.2 + rng.gen_range(-0.5, 0.5));
            let rand = rng.gen::<f64>();
            let m: Rc<RefCell<dyn Material>> = if rand < 0.7 {
                // diffuse material
                Rc::from(RefCell::new(LambertianDiffuse {
                    albedo: Vec3::random(0.0, 1.0),
                }))
            } else if rand < 0.9 {
                // metal material
                Rc::from(RefCell::new(Metal {
                    fuzziness: rng.gen_range(0.0, 0.5),
                    albedo: Vec3::random(0.5, 1.0),
                }))
            } else {
                // glass material
                Rc::from(RefCell::new(Dielectric::new(
                    1.33, Vec3::one())))
            };
            let b: Rc<RefCell<dyn Hittable>> = Rc::from(RefCell::new(Sphere {
                center,
                radius: 0.25,
                mat: m,
            }));
            world.add_hittable(&b);
        }
    }

    // add three giant balls!
    let m1: Rc<RefCell<dyn Material>> = Rc::from(RefCell::new(LambertianDiffuse {
        albedo: Vec3::random(0.0, 1.0),
    }));
    let m2: Rc<RefCell<dyn Material>> = Rc::from(RefCell::new(Dielectric::new(
        1.33, Vec3::one())));
    let m3: Rc<RefCell<dyn Material>> = Rc::from(RefCell::new(Metal {
        fuzziness: 0.0,
        albedo: Vec3(0.7, 0.6, 0.5),
    }));
    let b1: Rc<RefCell<dyn Hittable>> = Rc::from(RefCell::new(Sphere {
        center: Vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: m1,
    }));
    let b2: Rc<RefCell<dyn Hittable>> = Rc::from(RefCell::new(Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: m2,
    }));
    let b3: Rc<RefCell<dyn Hittable>> = Rc::from(RefCell::new(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: m3,
    }));
    world.add_hittable(&b1);
    world.add_hittable(&b2);
    world.add_hittable(&b3);

    world
}

fn main() {
    let width = 750;
    let height = 500;

    // set up the renderer and fire it up
    let mut r = DefaultRenderer::new(width, height);
    r.set_camera(init_camera());
    r.set_world(init_world());
    r.set_pixel_sample(64);

    let t = time::SystemTime::now();
    let p = r.render().unwrap_or_else(|s| { panic!("Render failed, {}", s) });
    println!("Finished, time = {}ms.",
             time::SystemTime::now().duration_since(t).unwrap().as_millis());
    println!("Writing to out.png...");
    p.write_to_png("out.png");
}
