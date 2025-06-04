use std::sync::Arc;

use crate::{
    material::Material,
    math::transform::Transform,
    render::ray::Ray,
};

use super::{Geometry, HitRecord};

pub struct Primitive {
    /// The real shape of the object wrapped in.
    pub shape: Arc<dyn Geometry>,

    /// The hierarchical transformation applied to this primitive.
    pub transform: Transform,

    /// The matieral of the object.
    pub material: Arc<Material>,
}

impl Primitive {
    pub fn new(shape: Arc<dyn Geometry>, transform: Transform, material: Arc<Material>) -> Self {
        Self {
            shape,
            transform,
            material,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let r = ray.apply_inv(&self.transform);
        self.shape.intersect_local(&r).map(|local_hit| {
            let p = ray.at(&local_hit.t);
            let n = (self.transform.inv * local_hit.n.to_homo())
                .to_inhomo()
                .normalize();

            HitRecord {
                t: local_hit.t,
                p,
                n,
                primitive: self,
                uv: local_hit.uv,
            }
        })
    }
}
