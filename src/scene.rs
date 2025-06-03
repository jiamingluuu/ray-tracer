use std::sync::Arc;

use crate::{
    light::Light,
    render::ray::Ray,
    shape::{HitRecord, Shape},
};

pub struct Scene {
    shapes: Vec<Arc<dyn Shape>>,
    lights: Vec<Arc<dyn Light>>,
    // bvh: Option<Box<BVHNode>>,
    // pub camera: Camera,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn find_first_hit(&self, ray: &Ray) -> Option<HitRecord> {
        self.shapes
            .iter()
            .filter_map(|shape| shape.intersect(ray))
            .min_by(|r1, r2| r1.t.total_cmp(&r2.t))
    }

    pub fn add_shape(&mut self, shape: Arc<dyn Shape>) {
        self.shapes.push(shape.clone());
    }
}
