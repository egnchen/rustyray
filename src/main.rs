use std::sync::{Arc, RwLock};

use rand::{Rng, thread_rng};

use ray_tracer::object::{Hittable, HittableObject, make_material_object, make_sphere_object, MaterialObject, Sphere, World};
use ray_tracer::object::material::{Dielectric, LambertianDiffuse, Material, Metal};
use ray_tracer::render::{DefaultRenderer, MultiRenderer};
use ray_tracer::render::{Camera, Renderer};
use ray_tracer::utils::Vec3;

/// initialize camera with default parameters
fn init_camera() -> Camera {
    let look_from = Vec3(13.0, 2.0, 4.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    Camera::look_from(
        look_from,
        look_at,
        Vec3(0.0, 1.0, 0.0),
        20.0,
        1.5,
        0.1,
        (look_at - look_from).length(),
    )
}

/// initialize the world, fill it with objects
fn init_world() -> World {
    let mut world = World::new();

    let mat_ground = make_material_object(LambertianDiffuse {
        albedo: Vec3(0.7, 0.7, 0.7),
    });
    let sphere_ground = make_sphere_object(Vec3(0.0, -1000.0, -1.0), 1000.0, &mat_ground);
    world.add_hittable(&sphere_ground);

    let mut rng = thread_rng();
    for i in -5..=10 {
        for j in -4..=4 {
            if j == 0 {
                continue;
            }
            let center = Vec3(
                i as f64 * 1.2 + rng.gen_range(-0.5, 0.5),
                0.25,
                j as f64 * 1.2 + rng.gen_range(-0.5, 0.5),
            );
            let rand = rng.gen::<f64>();
            let m = if rand < 0.65 {
                make_material_object(LambertianDiffuse {
                    albedo: Vec3::random(0.0, 1.0),
                })
            } else if rand < 0.9 {
                make_material_object(Metal {
                    fuzziness: rng.gen_range(0.0, 0.5),
                    albedo: Vec3::random(0.5, 1.0),
                })
            } else {
                make_material_object(Dielectric::new(1.33, Vec3::one()))
            };
            let b = make_sphere_object(center, 0.25, &m);
            world.add_hittable(&b);
        }
    }

    // add three giant balls!
    let m1 = make_material_object(LambertianDiffuse {
        albedo: Vec3::random(0.0, 1.0),
    });
    let m2 = make_material_object(Dielectric::new(1.33, Vec3::one()));
    let m3 = make_material_object(Metal {
        fuzziness: 0.1,
        albedo: Vec3(0.7, 0.6, 0.5),
    });
    let b1 = make_sphere_object(Vec3(-4.0, 1.0, 0.0), 1.0, &m1);
    let b2 = make_sphere_object(Vec3(0.0, 1.0, 0.0), 1.0, &m2);
    let b3 = make_sphere_object(Vec3(4.0, 1.0, 0.0), 1.0, &m3);
    world.add_hittable(&b1);
    world.add_hittable(&b2);
    world.add_hittable(&b3);

    world
}

fn main() {
    let width = 750;
    let height = 500;

    // set up the renderer and fire it up
    // let mut r = DefaultRenderer::new(width, height);
    let mut r = MultiRenderer::new(width, height);
    r.set_camera(init_camera());
    r.set_world(init_world());
    r.set_pixel_sample(64);

    let p = r
        .render()
        .unwrap_or_else(|s| panic!("Render failed, {}", s));
    println!("Writing to out.png...");
    p.write_to_png("out.png");
}
