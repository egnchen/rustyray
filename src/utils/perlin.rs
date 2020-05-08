//! Perlin: generate perlin noise

use std::iter::from_fn;

use rand::distributions::Uniform;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::utils::Vec3;

pub struct Perlin {
    perm_x: Vec<u8>,
    perm_y: Vec<u8>,
    perm_z: Vec<u8>,
    random_float: Vec<f32>,
    random_vector: Vec<Vec3<f64>>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut rng = thread_rng();
        let dis = Uniform::new(0.0, 1.0);
        let random_float: Vec<f32> = rng.sample_iter(dis).take(256).collect();
        let random_vector = from_fn(|| Some(Vec3::random(-1.0, 1.0)))
            .take(256)
            .collect();
        let mut perm_x: Vec<u8> = (0..=255).collect();
        perm_x.shuffle(&mut rng);
        let mut perm_y: Vec<u8> = (0..=255).collect();
        perm_y.shuffle(&mut rng);
        let mut perm_z: Vec<u8> = (0..=255).collect();
        perm_z.shuffle(&mut rng);
        Perlin {
            perm_x,
            perm_y,
            perm_z,
            random_float,
            random_vector,
        }
    }

    pub fn noise(&self, p: Vec3<f64>, frequency: f64) -> f32 {
        let Vec3(i, j, k) = p.apply(|x| ((frequency * x) as usize) & 255);
        self.random_float[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }

    pub fn smoothed_noise(&self, p: Vec3<f64>, frequency: f64) -> f32 {
        let p = p * frequency;
        // Hermitian smoothing
        let h = |x: f64| x * x * (3.0 - 2.0 * x);
        let Vec3(u, v, w) = p.apply(|x| h(x - x.floor()));
        // get fractional part
        let Vec3(i, j, k) = p.apply(|x| x.floor() as usize);
        let mut c: [[[f32; 2]; 2]; 2] = [[[f32::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_float[(self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255])
                        as usize];
                }
            }
        }
        Perlin::trilinear_interpolate(&c, u, v, w)
    }

    pub fn smoothed_shifted_noise(&self, p: Vec3<f64>, frequency: f64) -> f32 {
        let p = p * frequency;
        let Vec3(u, v, w) = p.apply(|x| x - x.floor());
        let Vec3(i, j, k) = p.apply(|x| x.floor() as usize);
        let mut c: [[[Vec3<f64>; 2]; 2]; 2] = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_vector[(self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255])
                        as usize];
                }
            }
        }
        0.5 * (1.0 + Perlin::perlin_interpolate(&c, u, v, w))
    }

    #[inline(always)]
    fn trilinear_interpolate(c: &[[[f32; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f32 {
        let mut ret = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let val = c[i][j][k] as f64;
                    let (i, j, k) = (i as f64, j as f64, k as f64);
                    ret += (i * u + (1.0 - i) * (1.0 - u))
                        * (j * v + (1.0 - j) * (1.0 - v))
                        * (k * w + (1.0 - k) * (1.0 - w))
                        * val;
                }
            }
        }
        ret as f32
    }

    #[inline(always)]
    fn perlin_interpolate(c: &[[[Vec3<f64>; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f32 {
        let Vec3(hu, hv, hw) = Vec3(u, v, w).apply(|x| x * x * (3.0 - 2.0 * x));
        let mut ret: f64 = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let val = c[i][j][k];
                    let (i, j, k) = (i as f64, j as f64, k as f64);
                    let weight = Vec3(u - i, v - j, w - k);
                    ret += (i * hu + (1.0 - i) * (1.0 - hu))
                        * (j * hv + (1.0 - j) * (1.0 - hv))
                        * (k * hw + (1.0 - k) * (1.0 - hw))
                        * val.dot(weight);
                }
            }
        }
        ret as f32
    }
}
