use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use num_traits::float::FloatCore;
use rand::{thread_rng, Rng};

use ray_tracer::object::aabb::AABB;
use ray_tracer::object::texture::SolidColor;
use ray_tracer::object::{make_material_object, make_sphere_object, LambertianDiffuse};
use ray_tracer::render::Camera;
use ray_tracer::utils::Vec3;

/// camera benchmark
fn camera_benchmark(c: &mut Criterion) {
    c.bench_function("Get random point in unit disk", |b| {
        b.iter(|| Camera::get_rand_in_unit_disk());
    });
}

fn aabb_hit_benchmark(c: &mut Criterion) {
    c.bench_function("Hit on an AABB", |b| {
        let aabb = AABB {
            min: Vec3(1.0, 1.0, 1.0),
            max: Vec3(-1.0, -1.0, -1.0),
        };
        let mut rng = thread_rng();
        b.iter(move || aabb.hit(&rng.gen(), 0.001, f64::infinity()));
    });
}

fn sphere_hit_benchmark(c: &mut Criterion) {
    c.bench_function("Hit on a sphere", |b| {
        let mat = make_material_object(LambertianDiffuse {
            texture: Arc::new(SolidColor::new(0.5, 0.5, 0.5)),
        });
        let sphere = make_sphere_object(Vec3(0.0, 0.0, 0.0), 1.0, &mat);
        let mut rng = thread_rng();
        b.iter(move || sphere.hit(&rng.gen(), 0.001, f64::infinity()));
    });
}

criterion_group!(
    benches,
    camera_benchmark,
    aabb_hit_benchmark,
    sphere_hit_benchmark
);
criterion_main!(benches);
