# raytracer

This is [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) implemented in Rust.

## TODO

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
- [ ] Positionable camera
- [ ] Depth of field
- [ ] Scene configuration
- [ ] Material configuration
- [ ] Multi-threaded rendering

## Running the program

### Prerequisites

There shouldn't be any limitation on the operating system you use as long as you have **Rust toolchain** installed on it. For those who doesn't have any experience in Rust, google for `rustup`.

### Running

In the project directory, use `cargo run --release` to build the project. `release` is needed here because default `debug` profile is slow, which is painful for a computationally intensive program like a ray tracer :)

```bash
$ cargo run --release
    Finished release [optimized] target(s) in 0.08s
     Running `target\release\ray_tracer.exe`
███████████████████████████████████████████████████████████████150/150
Finished, time = 2450ms.
Doing gamma correction...
Writing to out.ppm...
```

The result is in `out.ppm`. Search for ppm file viewers online :)