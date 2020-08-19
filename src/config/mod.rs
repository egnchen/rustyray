use crate::object::World;
use crate::render::Camera;

pub mod cornell_box;
pub mod random_spheres;
pub mod two_spheres;

pub trait SceneConfig {
    fn get_camera(&self) -> Camera;
    fn get_world(&self) -> World;
}
