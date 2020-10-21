pub use camera::Camera;
pub use filter::GammaFilter;
pub use multi_renderer::MultiRenderer;

use crate::utils::Picture;

pub mod camera;
pub mod filter;
pub mod skybox;

pub mod multi_renderer;

pub trait Renderer {
    fn render(&self) -> Result<Picture, &'static str>;
}
