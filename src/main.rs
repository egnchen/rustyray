extern crate clap;

use std::any::Any;

use clap::{App, Arg};
use rand::prelude::StdRng;
use rand::thread_rng;

use ray_tracer::config::cornell_box::CornellBoxScene;
use ray_tracer::config::next_week_final_scene::NextWeekFinalScene;
use ray_tracer::config::random_spheres::RandomSpheresScene;
use ray_tracer::config::random_spheres_night::RandomSpheresNightScene;
use ray_tracer::config::two_spheres::TwoSpheresScene;
use ray_tracer::config::SceneConfig;
use ray_tracer::io::file::write_to_png;
use ray_tracer::render::multi_renderer::PresetLevel;
use ray_tracer::render::MultiRenderer;
use ray_tracer::render::Renderer;

fn get_configuration(name: &str) -> Box<dyn SceneConfig> {
    match name {
        "CornellBoxScene" => Box::new(CornellBoxScene {}),
        "NextWeekFinalScene" => Box::new(NextWeekFinalScene {}),
        "RandomSpheresScene" => Box::new(RandomSpheresScene { bounce: true }),
        "RandomSpheresNightScene" => Box::new(RandomSpheresNightScene { bounce: false }),
        "TwoSpheresScene" => Box::new(TwoSpheresScene {}),
        _ => panic!("Invalid scene configuration name {}", name),
    }
}

fn main() {
    let matches = App::new("RustyRay ray-tracing renderer")
        .version("1.0")
        .author("Yijun Chen(github: @eyeKill)")
        .about("A simple ray tracer written in rust.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONFIG")
                .help("The configured scene to use.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("preset")
                .short("p")
                .long("preset")
                .value_name("PRESET")
                .help("The preset to use, among 0~3 standing for low, medium, high & ultra.")
                .takes_value(true),
        )
        .get_matches();

    let config = matches.value_of("config").unwrap_or("CornellBoxScene");
    let preset: usize = String::from(matches.value_of("preset").unwrap_or("0"))
        .parse()
        .unwrap();
    let preset: PresetLevel = PresetLevel::from(preset).unwrap();

    // set up the scene & the renderer
    let scene = get_configuration(config);
    let mut r = MultiRenderer::new();
    r.set_camera(scene.get_camera());
    r.set_world(scene.get_world());
    r.set_render_preset(preset);
    // fire it up
    println!("Start rendering scene {}...", config);
    let p = r
        .render()
        .unwrap_or_else(|s| panic!("Render failed, {}", s));
    println!("Writing to out.png...");
    write_to_png(&p, "out.png");
}
