use crate::math::point3::Point3;

pub struct Ray {
    pub p: Point3,
    pub d: Point3
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        self.p + t * self.d
    }
}
