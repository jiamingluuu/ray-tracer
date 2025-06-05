// TODO: Remove
#![allow(unused_variables)]

use crate::{
    render::ray::Ray,
    shape::{HitRecord, primitive::Primitive},
};

struct LinearBVHNode {}

pub struct BVH<'a> {
    nodes: Vec<LinearBVHNode>,
    max_prims_in_node: usize,
    primitives: Vec<&'a Primitive>,
}

impl<'a> BVH<'a> {
    pub fn new(max_prims_in_node: usize) -> Self {
        Self {
            nodes: Vec::new(),
            max_prims_in_node,
            primitives: Vec::new(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        todo!()
    }
}
