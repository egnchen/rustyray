use crate::utils::Vec3;

pub mod file;

/// Color in RGB
pub type Color = Vec3<f64>;
pub type Color24 = Vec3<u8>;

impl Color {
    #[inline(always)]
    pub fn new(v1: f64, v2: f64, v3: f64) -> Color {
        Color {
            0: v1,
            1: v2,
            2: v3,
        }
    }
}

impl Color24 {
    pub fn from(c: &Color) -> Self {
        Color24 {
            0: (c.0 * 255.0) as u8,
            1: (c.1 * 255.0) as u8,
            2: (c.2 * 255.0) as u8,
        }
    }
}

pub struct Picture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}
