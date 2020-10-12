use ray_tracer::config::cornell_box::CornellBoxScene;
use ray_tracer::config::random_spheres::RandomSphereScene;
use ray_tracer::config::two_spheres::TwoSpheresScene;
use ray_tracer::config::SceneConfig;
use ray_tracer::io::file::write_to_png;
use ray_tracer::render::MultiRenderer;
use ray_tracer::render::Renderer;

fn main() {
    let width = 300;
    let height = 300;

    // set up the scene
    // let s = RandomSphereScene { bounce: true };
    // let s = TwoSpheresScene {};
    let s = CornellBoxScene {};
    // set up the renderer
    let mut r = MultiRenderer::new(width, height);
    r.set_camera(s.get_camera());
    r.set_world(s.get_world());
    r.set_pixel_sample(256);
    // fire it up
    println!("Start rendering scene {}...", s.get_name());
    let p = r
        .render()
        .unwrap_or_else(|s| panic!("Render failed, {}", s));
    println!("Writing to out.png...");
    write_to_png(&p, "out.png");
}
