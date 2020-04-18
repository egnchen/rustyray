use super::vec::Vec3;

pub struct Ray {
    orig: Vec3<f64>,
    dir: Vec3<f64>,
}

impl Ray {
    pub fn new(orig: Vec3<f64>, dir: Vec3<f64>) -> Ray {
        Ray {
            orig,
            dir,
        }
    }

    pub fn origin(&self) -> Vec3<f64> { self.orig }
    pub fn direction(&self) -> Vec3<f64> { self.dir }
    pub fn at(&self, t: f64) -> Vec3<f64> {
        self.orig + self.dir * t
    }
}