use ray_tracer::config::perlin_sphere::PerlinSphereScene;
use ray_tracer::config::SceneConfig;
use ray_tracer::io::file::write_to_png;
use ray_tracer::render::MultiRenderer;
use ray_tracer::render::Renderer;

fn main() {
    let width = 600;
    let height = 400;

    // set up the scene
    // let s = RandomSphereScene { bounce: false };
    let s = PerlinSphereScene {};
    // set up the renderer
    let mut r = MultiRenderer::new(width, height);
    r.set_camera(s.get_camera());
    r.set_world(s.get_world());
    r.set_pixel_sample(64);
    // fire it up
    let p = r
        .render()
        .unwrap_or_else(|s| panic!("Render failed, {}", s));
    println!("Writing to out.png...");
    write_to_png(&p, "out.png");
}
