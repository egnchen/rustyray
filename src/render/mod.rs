pub use camera::Camera;
pub use filter::GammaFilter;

use crate::io::Picture;

pub mod camera;
pub mod filter;

pub mod default_renderer;

pub trait Renderer {
    fn render(&self) -> Result<Picture, &'static str>;
}