use std::sync::Arc;

use crate::config::SceneConfig;
use crate::io::file::read_picture;
use crate::object::texture::{ImageTexture, MarbleTexture, NoiseTexture};
use crate::object::{
    make_material_object, make_sphere_object, make_texture_object, LambertianDiffuse, World,
};
use crate::render::Camera;
use crate::utils::perlin::Perlin;
use crate::utils::Vec3;

pub struct PerlinSphereScene {}

impl SceneConfig for PerlinSphereScene {
    fn get_camera(&self) -> Camera {
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
            0.0,
            0.01,
        )
    }

    fn get_world(&self) -> World {
        let perlin = Arc::new(Perlin::new());
        let texture = make_texture_object(MarbleTexture {
            generator: perlin.clone(),
            scale: 2.0,
            turbulence: 10.0,
        });
        let globe_texture = make_texture_object(ImageTexture {
            image: Arc::new(read_picture("assets/textures/earthmap.jpg")),
        });
        let globe_mat = make_material_object(LambertianDiffuse {
            texture: globe_texture,
        });
        let mat = make_material_object(LambertianDiffuse { texture });
        let s1 = make_sphere_object(Vec3(0.0, -1000.0, 0.0), 1000.0, &mat);
        let s2 = make_sphere_object(Vec3(0.0, 2.0, 0.0), 2.0, &globe_mat);
        let mut world = World::new();
        world.add_hittable(&s1);
        world.add_hittable(&s2);
        world.update_bounding_box();

        world
    }
}
