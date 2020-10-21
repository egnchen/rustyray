use std::sync::Arc;

use image::math::Rect;
use rand::{thread_rng, Rng};

use crate::config::SceneConfig;
use crate::io::file::read_picture;
use crate::object::constant_medium::ConstantMedium;
use crate::object::container::Container;
use crate::object::cube::Cube;
use crate::object::material::DiffuseLight;
use crate::object::rect::XZRect;
use crate::object::texture::{ImageTexture, MarbleTexture};
use crate::object::{
    make_material_object, make_texture_object, Dielectric, HittableObject, LambertianDiffuse,
    MaterialObject, Metal, MovingSphere, NoiseTexture, SolidColor, Sphere, TextureObject, World,
};
use crate::render::skybox::{ColorGradientSkyBox, SkyBox};
use crate::render::Camera;
use crate::utils::perlin::Perlin;
use crate::utils::{Color, Picture, Vec3};

pub struct NextWeekFinalScene {}

impl SceneConfig for NextWeekFinalScene {
    fn get_name(&self) -> &'static str {
        "NextWeekFinalScene"
    }

    fn get_camera(&self) -> Camera {
        let look_from = Vec3::new(478.0, 278.0, -600.0);
        let look_at = Vec3::new(278.0, 278.0, 0.0);
        Camera::look_from(
            look_from,
            look_at,
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            1.0,
            0.0,
            (look_at - look_from).length(),
            0.0,
            1.0,
        )
    }

    fn get_world(&self) -> World {
        const BOXES_PER_SIDE: i32 = 20;
        let mut world = World::new();

        let mut c1 = Container::new();
        let ground: MaterialObject = Arc::new(LambertianDiffuse {
            texture: Arc::new(SolidColor::new(0.48, 0.83, 0.53)),
        });
        let mut rng = thread_rng();
        for i in 0..BOXES_PER_SIDE {
            for j in 0..BOXES_PER_SIDE {
                let w = 100.0;
                let x0 = -1000.0 + i as f64 * w;
                let z0 = -1000.0 + j as f64 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = rng.gen_range(1.0, 101.0);
                let z1 = z0 + w;
                let c: HittableObject = Arc::new(Cube::new(
                    Vec3::new(x0, y0, z0),
                    Vec3::new(x1, y1, z1),
                    &ground,
                ));
                c1.add_hittable(&c);
            }
        }
        c1.update_metadata();

        world.add_hittable(&(Arc::new(c1) as HittableObject));

        // light
        let white: TextureObject = Arc::new(SolidColor::new(1.0, 1.0, 1.0));
        let light_mat: MaterialObject = Arc::new(DiffuseLight {
            emit: white.clone(),
            brightness: 7.0,
        });

        let light: HittableObject = Arc::new(XZRect::new(
            (123.0, 147.0),
            (423.0, 412.0),
            554.0,
            &light_mat,
        ));

        world.add_hittable(&light);

        // moving sphere
        let center1 = Vec3::new(400.0, 400.0, 200.0);
        let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
        let moving_sphere_mat: MaterialObject = Arc::new(LambertianDiffuse {
            texture: Arc::new(SolidColor::new(0.7, 0.3, 0.1)),
        });
        let ms: HittableObject = Arc::new(MovingSphere::new(
            center1,
            center2,
            0.0,
            1.0,
            50.0,
            &moving_sphere_mat,
        ));
        world.add_hittable(&ms);

        // crystal ball
        let ds: HittableObject = Arc::new(Sphere::new(
            Vec3::new(260.0, 150.0, 45.0),
            50.0,
            &(Arc::new(Dielectric::new(1.5, Vec3::one())) as MaterialObject),
        ));
        world.add_hittable(&ds);

        // metal ball
        let ms: HittableObject = Arc::new(Sphere::new(
            Vec3::new(0.0, 150.0, 145.0),
            50.0,
            &(Arc::new(Metal::new(50.0, Vec3::new(0.8, 0.8, 0.9))) as MaterialObject),
        ));
        world.add_hittable(&ms);

        let b_mat: MaterialObject = Arc::new(Dielectric::new(1.5, Vec3::one()));
        // a fog sphere
        let boundary: HittableObject =
            Arc::new(Sphere::new(Vec3::new(360.0, 150.0, 45.0), 50.0, &b_mat));
        world.add_hittable(&boundary);
        let fog_tex: TextureObject = Arc::new(SolidColor::new(0.2, 0.4, 0.9));
        let cs: HittableObject = Arc::new(ConstantMedium::new(&boundary, 0.2, &fog_tex));
        world.add_hittable(&cs);
        // and the fog covering the whole scene, making a gloring effect
        let boundary: HittableObject = Arc::new(Sphere::new(Vec3::zero(), 5000.0, &b_mat));
        let fog_tex: TextureObject = Arc::new(SolidColor::new(1.0, 1.0, 1.0));
        let cs_whole: HittableObject = Arc::new(ConstantMedium::new(&boundary, 0.0001, &fog_tex));
        world.add_hittable(&cs_whole);

        // earth sphere
        let emat: MaterialObject = Arc::new(LambertianDiffuse {
            texture: Arc::new(ImageTexture {
                image: Arc::new(read_picture("assets/textures/earthmap.jpg")),
            }),
        });
        world.add_hittable(
            &(Arc::new(Sphere::new(Vec3::new(400.0, 200.0, 400.0), 100.0, &emat))
                as HittableObject),
        );

        // a cube of little balls
        let mut cc = Container::new();
        let white_mat = make_material_object(LambertianDiffuse {
            texture: white.clone(),
        });
        for j in 0..1000 {
            let c: HittableObject = Arc::new(Sphere::new(
                Vec3::random(0.0, 165.0) + Vec3::new(-100.0, 270.0, 395.0),
                10.0,
                &white_mat,
            ));
            cc.add_hittable(&c);
        }
        cc.update_metadata();
        world.add_hittable(&(Arc::new(cc) as HittableObject));

        // perlin noise ball
        let perlin = Arc::new(Perlin::new());
        let texture = make_texture_object(NoiseTexture {
            generator: perlin,
            frequency: 1.0,
            shifted: false,
        });
        let ps: HittableObject = Arc::new(Sphere::new(
            Vec3::new(220.0, 280.0, 300.0),
            80.0,
            &make_material_object(LambertianDiffuse { texture }),
        ));
        world.add_hittable(&ps);

        world.update_metadata();
        let sb: Arc<dyn SkyBox + Send + Sync> = Arc::new(ColorGradientSkyBox {
            v1: Color::zero(),
            v2: Color::zero(),
        });
        world.set_skybox(&sb);

        world
    }
}
