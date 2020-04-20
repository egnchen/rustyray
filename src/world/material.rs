use rand::{Rng, thread_rng};

use crate::utils::{Ray, Vec3};
use crate::world::HitRecord;

pub struct FilteredRay {
    pub attenuation: Vec3<f64>,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r: &Ray, h: &HitRecord) -> Option<FilteredRay>;
}

pub struct LambertianDiffuse {
    pub albedo: Vec3<f64>,
}

// helper function
fn rand_unit_vector() -> Vec3<f64> {
    let mut r = thread_rng();
    let a: f64 = r.gen_range(0.0, 2.0 * std::f64::consts::PI);
    let z: f64 = r.gen_range(-1.0, 1.0);
    let r: f64 = (1.0 - z * z).sqrt();
    Vec3(r * a.cos(), r * a.sin(), z)
}

impl Material for LambertianDiffuse {
    fn scatter(&self, _r: &Ray, h: &HitRecord) -> Option<FilteredRay> {
        Some(FilteredRay {
            attenuation: self.albedo,
            scattered: Ray {
                orig: h.p,
                dir: h.normal + rand_unit_vector()
            }
        })
    }
}

pub struct Metal {
    pub fuzziness: f64,
    pub albedo: Vec3<f64>,
}

impl Metal {
    pub fn new(fuzziness: f64, albedo: Vec3<f64>) -> Metal {
        if fuzziness > 1.0 {
            return Metal { fuzziness: 1.0, albedo };
        } else {
            return Metal { fuzziness, albedo };
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, h: &HitRecord) -> Option<FilteredRay> {
        let reflect_dir = r.dir - h.normal * (2.0 * r.direction().dot(h.normal)) +
            rand_unit_vector() * self.fuzziness;
        if reflect_dir.dot(h.normal) > 0.0 {
            return Some(FilteredRay {
                attenuation: self.albedo,
                scattered: Ray {
                    orig: h.p,
                    dir: reflect_dir
                }
            });
        } else {
            return None;
        }

    }
}