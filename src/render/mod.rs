pub use camera::Camera;
pub use default_renderer::DefaultRenderer;
pub use filter::GammaFilter;
pub use mult_renderer::MultRenderer;

use crate::io::Picture;

pub mod camera;
pub mod filter;

mod default_renderer;
mod mult_renderer;

pub trait Renderer {
    fn render(&self) -> Result<Picture, &'static str>;
}
