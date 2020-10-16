use rand::distributions::{Distribution, Standard};
use rand::Rng;

use super::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub orig: Vec3<f64>,
    pub dir: Vec3<f64>,
    pub t: f64,
}

impl Ray {
    pub fn origin(&self) -> Vec3<f64> {
        self.orig
    }
    pub fn direction(&self) -> Vec3<f64> {
        self.dir
    }
    pub fn time(&self) -> f64 {
        self.t
    }
    #[inline(always)]
    pub fn at(&self, t: f64) -> Vec3<f64> {
        self.orig + self.dir * t
    }
}

// for debug purposes
impl Distribution<Ray> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ray {
        // TODO make this more concise
        let (d0, d1, d2) = rng.gen();
        let (o0, o1, o2) = rng.gen();
        Ray {
            dir: Vec3::new(d0, d1, d2),
            orig: Vec3::new(o0, o1, o2),
            t: 1.0,
        }
    }
}
