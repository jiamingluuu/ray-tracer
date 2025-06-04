use crate::math::{transform::Transform, vec3::Vec3};

pub struct Ray {
    pub p: Vec3,
    pub d: Vec3,
}

impl Ray {
    pub fn at(&self, t: &f64) -> Vec3 {
        self.p + *t * self.d
    }

    pub fn apply_inv(&self, transform: &Transform) -> Self {
        let p = (transform.inv * self.p.to_homo()).to_inhomo();
        let d = Vec3::new(
            transform.inv[0][0] * self.d.x
                + transform.inv[0][1] * self.d.x
                + transform.inv[0][1] * self.d.z,
            transform.inv[1][0] * self.d.x
                + transform.inv[1][1] * self.d.x
                + transform.inv[1][1] * self.d.z,
            transform.inv[2][0] * self.d.x
                + transform.inv[2][1] * self.d.x
                + transform.inv[2][1] * self.d.z,
        );
        Self { p, d }
    }
}
