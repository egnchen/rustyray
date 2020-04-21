use rand::{Rng, thread_rng};

use crate::object::{Face, HitRecord};
use crate::utils::{Ray, Vec3};

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
            },
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
        return if reflect_dir.dot(h.normal) > 0.0 {
            Some(FilteredRay {
                attenuation: self.albedo,
                scattered: Ray {
                    orig: h.p,
                    dir: reflect_dir,
                },
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub eta: f64,
}

impl Dielectric {
    fn refract(ru: Vec3<f64>, normal: Vec3<f64>, eta_ratio: f64) -> Vec3<f64> {
        let cos_theta = -ru.dot(normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        return if sin_theta * eta_ratio > 1.0 {
            // cannot refract, must reflect
            ru - normal * (2.0 * ru.dot(normal))
        } else {
            // can refract
            let r_parallel = (ru + normal * cos_theta) * eta_ratio;
            if r_parallel.length_square() > 1.0 {
                println!("ru={}\nnormal={}", ru, normal);
                println!("eta_ratio={}\ncos_theta={}\nr_parallel={}", eta_ratio, cos_theta, r_parallel);
                panic!("");
            }
            let r_perp = normal * (-((1.0 - r_parallel.length_square()).sqrt()));
            r_parallel + r_perp
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, h: &HitRecord) -> Option<FilteredRay> {
        // calculate refraction
        let er = match h.f {
            Face::Inward => self.eta,
            Face::Outward => 1.0 / self.eta,
        };
        let dir = Self::refract(r.direction().unit_vector(), h.normal, er);
        Some(FilteredRay {
            attenuation: Vec3::one(),
            scattered: Ray {
                orig: h.p,
                dir,
            },
        })
    }
}