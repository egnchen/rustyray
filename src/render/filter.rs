//! Filter: define filters for rendering.

use crate::ppm::{Picture, Color};

pub trait Filter {
    fn filter(&self, p: &mut Picture);
}

pub struct GammaFilter {
    pub gamma: f64,
}

impl Filter for GammaFilter {
    fn filter(&self, p: &mut Picture) {
        let v = 1.0 / self.gamma;
        for c in p.data.iter_mut() {
            c.0 = c.0.powf(v);
            c.1 = c.1.powf(v);
            c.2 = c.2.powf(v);
        }
    }
}
