use std::sync::Arc;

use crate::{
    light::Light,
    render::ray::Ray,
    shape::{HitRecord, primitive::Primitive},
};

#[derive(Default)]
pub struct Scene {
    primitives: Vec<Primitive>,
    lights: Vec<Arc<dyn Light>>,
    // bvh: Option<Box<BVHNode>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn find_first_hit(&self, ray: &Ray) -> Option<HitRecord> {
        self.primitives
            .iter()
            .filter_map(|primitive| primitive.intersect(ray))
            .min_by(|p1, p2| p1.t.total_cmp(&p2.t))
    }
}
