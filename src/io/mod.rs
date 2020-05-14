use crate::utils::{Color, Vec3};

pub mod file;

/// Color in RGB
pub type Color24 = Vec3<u8>;
impl Color24 {
    pub fn from(c: &Color) -> Self {
        Color24 {
            0: (c.0.min(1.0) * 255.0) as u8,
            1: (c.1.min(1.0) * 255.0) as u8,
            2: (c.2.min(1.0) * 255.0) as u8,
        }
    }
}
