use rand::prelude::StdRng;
use rand::thread_rng;

use ray_tracer::config::cornell_box::CornellBoxScene;
use ray_tracer::config::next_week_final_scene::NextWeekFinalScene;
use ray_tracer::config::random_spheres::RandomSphereScene;
use ray_tracer::config::random_spheres_night::RandomSphereNightScene;
use ray_tracer::config::two_spheres::TwoSpheresScene;
use ray_tracer::config::SceneConfig;
use ray_tracer::io::file::write_to_png;
use ray_tracer::render::MultiRenderer;
use ray_tracer::render::Renderer;

fn main() {
    let width = 600;
    let height = 600;

    // set up the scene
    // let s = RandomSphereScene { bounce: false };
    // let s = RandomSphereNightScene { bounce: false };
    // let s = TwoSpheresScene {};
    // let s = CornellBoxScene {};
    let s = NextWeekFinalScene {};
    // set up the renderer
    let mut r = MultiRenderer::new(width, height);
    r.set_camera(s.get_camera());
    r.set_world(s.get_world());
    r.set_pixel_sample(8192);
    // fire it up
    println!("Start rendering scene {}...", s.get_name());
    let p = r
        .render()
        .unwrap_or_else(|s| panic!("Render failed, {}", s));
    println!("Writing to out.png...");
    write_to_png(&p, "out.png");
}
