use crate::math::vec3::Vec3;
use crate::render::colour::Colour;
use crate::shape::HitRecord;

#[derive(Clone, Debug, Default)]
pub struct Phong {
    pub ka: f64,
    pub kd: f64,
    pub ks: f64,
}

// Material properties for surface interaction
#[derive(Clone, Debug, Default)]
pub struct Material {
    pub albedo: Vec3,
    pub roughness: f64,
    pub metallic: f64,
    pub emission: Vec3,
}

// Surface interaction result
pub struct SurfaceInteraction {
    pub scattered_direction: Option<Vec3>,
    pub attenuation: Vec3,
    pub pdf: f64,
    pub colour: Colour,
}

impl Material {
    pub fn interact(&self, hit: &HitRecord) -> SurfaceInteraction {
        todo!()
    }
}
