use crate::vec::Vec3;
use crate::shape::Shape;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3<f64>,
    pub radius: f64,
}

/// calculate if a ray will hit a sphere
/// $$t^2 \vec{\mathbf{b}}\cdot\vec{\mathbf{b}}
//      + 2t \vec{\mathbf{b}} \cdot \vec{(\mathbf{a}-\mathbf{c})}
//      + \vec{(\mathbf{a}-\mathbf{c})} \cdot \vec{(\mathbf{a}-\mathbf{c})} - R^2 = 0$$
impl Shape for Sphere {
    fn hit(&self, r: &Ray) -> Option<f64> {
        let t1 = r.origin() - self.center;
        let t2 = r.direction();

        let a = t2.length2();
        let b = t1.dot(t2) * 2.0;
        let c = t1.length2() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
}