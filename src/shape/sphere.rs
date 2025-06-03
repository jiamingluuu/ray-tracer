use std::sync::Arc;

use crate::{
    material::Material,
    math::{transform::Transform, vec3::Vec3},
    render::{colour::Colour, ray::Ray},
    scene::Scene,
    shape::{HitRecord, Shape, ShapeInternal, SurfaceInteraction},
};

pub struct Sphere {
    internal: ShapeInternal,
}

impl Sphere {
    pub fn new(scene: &mut Scene, material: Material, colour: Colour) -> Arc<Self> {
        let shape = Arc::new(Self {
            internal: ShapeInternal {
                transform: Transform::new_identity(),
                material,
                colour,
            },
        });
        scene.add_shape(shape.clone());
        shape
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        todo!()
    }

    fn normal_at(&self, p: &Vec3) -> Vec3 {
        todo!()
    }

    fn interact(&self, hit: &HitRecord, incoming: &Vec3) -> SurfaceInteraction {
        todo!()
    }
}
