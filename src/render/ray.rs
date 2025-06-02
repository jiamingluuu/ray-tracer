use crate::math::vec3::Vec3;

pub struct Ray {
    pub p: Vec3,
    pub d: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.p + t * self.d
    }
}
