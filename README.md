# raytracer

This is [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) implemented in Rust.

## TODO

### one weekend
- [x] Output an image
- [x] Vector utilities
- [x] Basic skybox rendering
- [x] Rendering a sphere in normal vectors
- [x] Rendering multiple spheres
- [x] Multi-sampling
- [x] Diffuse materials
- [x] Gamma correction filter
- [x] Reflective materials
- [x] Reflective materials with fuzziness
- [x] Dielectric materials
- [x] Positionable camera
- [x] Depth of field
- [x] Scene configuration
- [ ] Material configuration
- [x] Multi-threaded rendering

### next week

- [x] Motion blur
- [x] Bounding volume hierarchies

## Running the program

### Prerequisites

There shouldn't be any limitation on the operating system you use as long as you have **Rust toolchain** installed on it. For those who doesn't have any experience in Rust, google for `rustup`.

### Running

In the project directory, use `cargo run --release` to build the project. `release` is recommended here because it activates all compile-time optimizations, and ray traces are computationally-intensive programs.


```bash
$ cargo run --release
    Finished release [optimized] target(s) in 0.24s
     Running `target\release\ray_tracer.exe`
Configuration: Picture size = 750 * 500, sample = 128, recursion depth = 16
Initializing threads... Thread count = 4
Thread 0 initiated
Thread 1 initiated
Thread 2 initiated
Thread 3 initiated
Thread 2 exit.
Thread 1 exit.
Thread 0 exit.
Thread 3 exit.
Done, time elapsed = 31.6933429s
Writing to out.png...
```

Main binary produces a picture of lots of random spheres(configuration in `src/config/random_spheres.rs`).
![](example.png)

(~500 spheres in the scene)