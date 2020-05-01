pub use camera::Camera;
pub use default_renderer::DefaultRenderer;
pub use filter::GammaFilter;
pub use multi_renderer::MultiRenderer;

use crate::io::Picture;

pub mod camera;
pub mod filter;
pub mod skybox;

mod default_renderer;
mod multi_renderer;

pub trait Renderer {
    fn render(&self) -> Result<Picture, &'static str>;
}