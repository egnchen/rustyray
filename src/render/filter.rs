//! Filter: define filters for rendering.

use crate::utils::Picture;

pub trait Filter {
    fn filter(&self, p: &mut Picture);
}

pub struct GammaFilter {
    pub gamma: f32,
}

impl Filter for GammaFilter {
    fn filter(&self, p: &mut Picture) {
        let v = 1.0 / self.gamma;
        for c in p.data.iter_mut() {
            c.x = c.x.powf(v);
            c.y = c.y.powf(v);
            c.z = c.z.powf(v);
        }
    }
}
