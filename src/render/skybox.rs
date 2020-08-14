use crate::utils::{Color, Ray};

pub trait SkyBox {
    fn get_color(&self, r: &Ray) -> Color;
}

pub struct ColorGradientSkyBox {
    pub v1: Color,
    pub v2: Color,
}

impl SkyBox for ColorGradientSkyBox {
    fn get_color(&self, r: &Ray) -> Color {
        let unit = r.direction().unit_vector();
        let t = (0.5 * (unit.y + 1.0)) as f32;
        self.v1 * (1.0 - t) + self.v2 * t
    }
}
