//! contain scene configurators
use crate::object::World;
use crate::render::Camera;

pub mod perlin_sphere;
pub mod random_spheres;

pub trait SceneConfig {
    fn get_camera(&self) -> Camera;
    fn get_world(&self) -> World;
}
