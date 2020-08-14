use crate::utils::{Color, Vec3};

pub mod file;

/// Color in RGB
pub type Color24 = Vec3<u8>;
impl Color24 {
    pub fn from(c: &Color) -> Self {
        Color24 {
            x: (c.x.min(1.0) * 255.0) as u8,
            y: (c.y.min(1.0) * 255.0) as u8,
            z: (c.z.min(1.0) * 255.0) as u8,
        }
    }
}
