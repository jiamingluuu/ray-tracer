use crate::math::vec3::Vec3;

// Material properties for surface interaction
#[derive(Clone, Debug)]
pub struct Material {
    pub albedo: Vec3,
    pub roughness: f64,
    pub metallic: f64,
    pub emission: Vec3,
}

