//! configuration for the random spheres scene

use std::sync::Arc;

use rand::prelude::StdRng;
use rand::{thread_rng, Rng, SeedableRng};

use crate::config::SceneConfig;
use crate::object::material::{Dielectric, DiffuseLight};
use crate::object::texture::{CheckerTexture, SolidColor};
use crate::object::{
    make_bouncing_sphere_object, make_material_object, make_sphere_object, LambertianDiffuse,
    Metal, World,
};
use crate::render::skybox::{ColorGradientSkyBox, SkyBox};
use crate::render::Camera;
use crate::utils::{Color, Vec3};

pub struct RandomSphereNightScene {
    pub bounce: bool,
}

impl SceneConfig for RandomSphereNightScene {
    fn get_name(&self) -> &'static str {
        "RandomSpheresNight"
    }

    // configure the camera
    fn get_camera(&self) -> Camera {
        let look_from = Vec3::new(12.0, 3.0, 4.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);
        Camera::look_from(
            look_from,
            look_at,
            Vec3::new(0.0, 1.0, 0.0),
            35.0,
            1.5,
            0.0,
            (look_at - look_from).length(),
            0.0,
            0.25,
        )
    }

    // configure the random sphere scene
    fn get_world(&self) -> World {
        let mut world = World::new();
        let skybox: Arc<dyn SkyBox + Send + Sync> = Arc::new(ColorGradientSkyBox {
            v1: Color::zero(),
            v2: Color::zero(),
        });
        world.set_skybox(&skybox);
        // checkered ground
        let mat_ground = make_material_object(LambertianDiffuse {
            texture: Arc::new(CheckerTexture {
                odd_color: Arc::new(SolidColor::new(1.0, 1.0, 1.0)),
                even_color: Arc::new(SolidColor::new(0.2, 0.3, 0.1)),
            }),
        });
        let sphere_ground = make_sphere_object(Vec3::new(0.0, -1000.0, -1.0), 1000.0, &mat_ground);
        world.add_hittable(&sphere_ground);

        let mut rng = StdRng::seed_from_u64(1010101);
        for i in -11..=11 {
            for j in -11..=11 {
                if j == 0 {
                    continue;
                }
                let center = Vec3::new(
                    i as f64 * 1.2 + rng.gen_range(-0.5, 0.5),
                    0.3,
                    j as f64 * 1.2 + rng.gen_range(-0.5, 0.5),
                );
                let rand = rng.gen::<f64>();
                let m = if rand < 0.25 {
                    make_material_object(LambertianDiffuse {
                        texture: Arc::new(SolidColor::random()),
                    })
                } else if rand < 0.5 {
                    make_material_object(DiffuseLight {
                        emit: Arc::new(SolidColor::random()),
                        brightness: rng.gen_range(0.5, 2.0),
                    })
                } else if rand < 0.8 {
                    make_material_object(Metal {
                        fuzziness: rng.gen_range(0.0, 0.5),
                        albedo: Vec3::random(0.5, 1.0),
                    })
                } else {
                    make_material_object(Dielectric::new(1.33, Vec3::one()))
                };

                let b = if self.bounce && m.get_type() == "LambertianDiffuse" {
                    make_bouncing_sphere_object(center, 0.3, rng.gen_range(0.0, 1.0), 0.0, 0.5, &m)
                } else {
                    make_sphere_object(center, 0.3, &m)
                };
                world.add_hittable(&b);
            }
        }

        // add three giant balls!
        let m1 = make_material_object(DiffuseLight {
            emit: Arc::new(SolidColor::random()),
            brightness: 3.0,
        });
        let m2 = make_material_object(DiffuseLight {
            emit: Arc::new(CheckerTexture {
                odd_color: Arc::new(SolidColor::new(1.0, 1.0, 1.0)),
                even_color: Arc::new(SolidColor::new(0.2, 0.3, 0.1)),
            }),
            brightness: 1.5,
        });
        let m3 = make_material_object(Dielectric::new(1.33, Vec3::one()));

        let b1 = make_sphere_object(Vec3::new(-4.0, 1.0, 0.0), 1.0, &m1);
        let b2 = make_sphere_object(Vec3::new(0.0, 1.0, 0.0), 1.0, &m2);
        let b3 = make_sphere_object(Vec3::new(4.0, 1.0, 0.0), 1.0, &m3);
        world.add_hittable(&b1);
        world.add_hittable(&b2);
        world.add_hittable(&b3);
        world.update_metadata();
        world
    }
}
