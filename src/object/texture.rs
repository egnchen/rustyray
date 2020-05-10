use std::sync::Arc;

use crate::utils::perlin::Perlin;
use crate::utils::{Color, Picture, Vec3};

pub trait Texture {
    fn get_color(&self, u: f64, v: f64, p: Vec3<f64>) -> Color;
}

pub struct SolidColor {
    pub color: Color,
}

impl SolidColor {
    pub fn new(r: f32, g: f32, b: f32) -> SolidColor {
        SolidColor {
            color: Color::new(r, g, b),
        }
    }

    pub fn random() -> SolidColor {
        SolidColor {
            color: Vec3::random(0.0_f32, 1.0) * Vec3::random(0.0_f32, 1.0),
        }
    }
}

impl Texture for SolidColor {
    fn get_color(&self, _u: f64, _v: f64, _p: Vec3<f64>) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    pub odd_color: Arc<SolidColor>,
    pub even_color: Arc<SolidColor>,
}

impl Texture for CheckerTexture {
    fn get_color(&self, u: f64, v: f64, p: Vec3<f64>) -> Color {
        let s = (10.0 * p.0).sin() * (10.0 * p.1).sin() * (10.0 * p.2).sin();
        if s < 0.0 {
            self.odd_color.get_color(u, v, p)
        } else {
            self.even_color.get_color(u, v, p)
        }
    }
}

pub struct HashTexture {
    pub generator: Arc<Perlin>,
    pub frequency: f64,
}

impl Texture for HashTexture {
    fn get_color(&self, _u: f64, _v: f64, p: Vec3<f64>) -> Color {
        Vec3::<f32>::one() * self.generator.noise(p, self.frequency)
    }
}

pub struct NoiseTexture {
    pub generator: Arc<Perlin>,
    pub frequency: f64,
    pub shifted: bool,
}

impl Texture for NoiseTexture {
    fn get_color(&self, u: f64, v: f64, p: Vec3<f64>) -> Color {
        if self.shifted {
            Vec3::<f32>::one() * self.generator.smoothed_shifted_noise(p, self.frequency)
        } else {
            Vec3::<f32>::one() * self.generator.smoothed_noise(p, self.frequency)
        }
    }
}

/// Create a marble-like texture.
///
/// * `scale`: Density(frequency) of the marble stripes. Larger this value, denser the stripes.
/// * `turbulence`: Strength of the turbulence of the sine strips. Larger this value, more turbulent the result.
pub struct MarbleTexture {
    pub generator: Arc<Perlin>,
    pub scale: f32,
    pub turbulence: f32,
}

impl Texture for MarbleTexture {
    fn get_color(&self, u: f64, v: f64, p: Vec3<f64>) -> Color {
        Vec3::<f32>::one()
            * 0.5
            * (1.0
                + (self.scale * p.z() as f32 + self.turbulence * self.generator.turbulence(p, 7))
                    .sin())
    }
}

// Spherical image texture
pub struct ImageTexture {
    pub image: Arc<Picture>,
}

impl Texture for ImageTexture {
    fn get_color(&self, u: f64, v: f64, p: Vec3<f64>) -> Color {
        // clamp u,v to image.width, image.height
        // u, v should both in range [0, 1](mathematically)
        let u = (u * self.image.width as f64) as usize;
        let v = ((1.0 - v) * self.image.height as f64) as usize;
        self.image.at(u, v)
    }
}
