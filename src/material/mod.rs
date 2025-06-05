#![allow(unused_variables)]

use crate::math::vec3::Vec3;
use crate::render::colour::Colour;
use crate::shape::HitRecord;

#[derive(Clone, Debug, Default)]
pub struct Phong {
    ka: f64,
    kd: f64,
    ks: f64,
}

// Material properties for surface interaction
#[derive(Clone, Debug, Default)]
pub struct Material {
    albedo: Vec3,
    roughness: f64,
    metallic: f64,
    phong: Phong,
    colour: Colour,
}

// Surface interaction result
#[derive(Debug)]
pub struct SurfaceInteraction {
    pub scattered_direction: Option<Vec3>,
    // pub attenuation: Vec3,
    // pub pdf: f64,
    pub colour: Colour,
}

impl Material {
    pub fn interact(&self, hit: &HitRecord) -> SurfaceInteraction {
        let max_depth = 10;
        let acc = Colour::new();

        todo!()
    }
}
