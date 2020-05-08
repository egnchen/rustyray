pub use ray::Ray;
pub use vec::Vec3;

pub mod perlin;
pub mod ray;
pub mod vec;

/// Color
pub type Color = Vec3<f32>;
impl Color {
    #[inline(always)]
    pub fn new(v1: f32, v2: f32, v3: f32) -> Color {
        Color {
            0: v1,
            1: v2,
            2: v3,
        }
    }
}

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
}
