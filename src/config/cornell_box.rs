use std::sync::Arc;

use crate::config::SceneConfig;
use crate::object::material::DiffuseLight;
use crate::object::rect::{XYRect, XZRect, YZRect};
use crate::object::{
    make_material_object, make_texture_object, Hittable, HittableObject, LambertianDiffuse,
    SolidColor, World,
};
use crate::render::skybox::{ColorGradientSkyBox, SkyBox};
use crate::render::Camera;
use crate::utils::{Color, Vec3};

pub struct CornellBoxScene {}

impl SceneConfig for CornellBoxScene {
    fn get_camera(&self) -> Camera {
        let look_from = Vec3::new(278.0, 278.0, 800.0);
        let look_at = Vec3::new(278.0, 278.0, 0.0);
        Camera::look_from(
            look_from,
            look_at,
            Vec3::new(0.0, 0.0, 1.0),
            40.0,
            1.0,
            0.0,
            (look_at - look_from).length(),
            0.0,
            0.01,
        )
    }

    fn get_world(&self) -> World {
        let red = make_material_object(LambertianDiffuse {
            texture: make_texture_object(SolidColor::new(0.65, 0.05, 0.05)),
        });
        let white = make_material_object(LambertianDiffuse {
            texture: make_texture_object(SolidColor::new(0.73, 0.73, 0.73)),
        });
        let green = make_material_object(LambertianDiffuse {
            texture: make_texture_object(SolidColor::new(0.12, 0.45, 0.45)),
        });
        let light = make_material_object(DiffuseLight {
            emit: make_texture_object(SolidColor::new(15.0, 15.0, 15.0)),
            brightness: 10.0,
        });
        let mut world = World::new();
        // generate walls
        let left: HittableObject = Arc::new(YZRect::new((0.0, 0.0), (555.0, 555.0), 555.0, &green));
        let right: HittableObject = Arc::new(YZRect::new((0.0, 0.0), (555.0, 555.0), 0.0, &red));
        let up: HittableObject = Arc::new(XZRect::new((0.0, 0.0), (555.0, 555.0), 555.0, &white));
        let down: HittableObject = Arc::new(XZRect::new((0.0, 0.0), (555.0, 555.0), 0.0, &white));
        let behind: HittableObject =
            Arc::new(XYRect::new((0.0, 0.0), (555.0, 555.0), 555.0, &white));
        let lamp: HittableObject =
            Arc::new(XZRect::new((213.0, 227.0), (343.0, 332.0), 550.0, &light));

        world.add_hittable(&left);
        world.add_hittable(&right);
        world.add_hittable(&down);
        world.add_hittable(&up);
        world.add_hittable(&behind);
        world.add_hittable(&lamp);

        // set up a dark skybox
        let sb: Arc<dyn SkyBox + Send + Sync> = Arc::new(ColorGradientSkyBox {
            v1: Color::zero(),
            v2: Color::zero(),
        });
        world.set_skybox(&sb);

        world.update_bounding_box();
        world
    }
}
