pub use ray::Ray;
pub use vec::Vec3;

pub mod perlin;
pub mod ray;
pub mod vec;

/// Color
pub type Color = Vec3<f32>;

pub struct Picture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl Picture {
    pub fn new(width: usize, height: usize) -> Picture {
        Picture {
            width,
            height,
            data: vec![Color::zero(); width * height],
        }
    }

    #[inline]
    pub fn at(&self, x: usize, y: usize) -> Color {
        self.data[y * self.width + x]
    }
}
