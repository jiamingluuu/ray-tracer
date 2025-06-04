use crate::{
    render::ray::Ray,
    shape::{LocalHitRecord, Shape},
};

pub struct Sphere {
    /// Radius of the sphere.
    r: f64,
}

impl Sphere {
    pub fn new(r: f64) -> Self {
        Self { r }
    }
}

impl Shape for Sphere {
    fn intersect_local(&self, ray: &Ray) -> Option<LocalHitRecord> {
        let a = ray.d.dot(&ray.d);
        let b = 2.0 * ray.p.dot(&ray.d);
        let c = ray.p.dot(&ray.p) - self.r * self.r;
        let d = b * b - 4.0 * a * c;

        if d < 0.0 {
            return None;
        }

        let sqrt_d = d.sqrt();
        let t1 = (-b - sqrt_d) / (2.0 * a);
        let t2 = (-b + sqrt_d) / (2.0 * a);

        let t = if t1 > f64::EPSILON {
            t1
        } else if t2 > f64::EPSILON {
            t2
        } else {
            return None;
        };

        let p = ray.at(&t);
        let n = (p / self.r).normalize();

        let theta = n.z.acos();
        let phi = n.y.atan2(n.x);
        let u = phi / (2.0 * std::f64::consts::PI) + 0.5;
        let v = theta / std::f64::consts::PI;

        Some(LocalHitRecord {
            t,
            p,
            n,
            uv: (u, v),
        })
    }
}
