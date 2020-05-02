use criterion::{criterion_group, criterion_main, Criterion};

use ray_tracer::render::Camera;

/// camera benchmark
fn camera_benchmark(c: &mut Criterion) {
    c.bench_function("Get random point in unit disk", |b| {
        b.iter(|| Camera::get_rand_in_unit_disk());
    });
}

criterion_group!(benches, camera_benchmark);
criterion_main!(benches);
