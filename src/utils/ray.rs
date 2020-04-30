use super::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub orig: Vec3<f64>,
    pub dir: Vec3<f64>,
}

impl Ray {
    pub fn origin(&self) -> Vec3<f64> {
        self.orig
    }
    pub fn direction(&self) -> Vec3<f64> {
        self.dir
    }
    pub fn at(&self, t: f64) -> Vec3<f64> {
        self.orig + self.dir * t
    }
}
