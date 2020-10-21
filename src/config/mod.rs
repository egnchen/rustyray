use std::collections::HashMap;

use crate::object::World;
use crate::render::Camera;

pub mod cornell_box;
pub mod next_week_final_scene;
pub mod random_spheres;
pub mod random_spheres_night;
pub mod two_spheres;

pub trait SceneConfig {
    fn get_camera(&self) -> Camera;
    fn get_world(&self) -> World;
}
