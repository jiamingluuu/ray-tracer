pub mod cube;
pub mod cylinder;
pub mod sphere;
pub mod triangle;

use crate::{material::Material, math::{transform::Transform, vec3::Vec3}, render::{colour::Colour, ray::Ray}};


pub trait Shape {
    /// Test ray-shape intersection, returns closest hit within t_min..t_max
    fn intersect(&self, ray: &Ray) -> Option<HitRecord>;

    /// Get normal at a point on the surface (for cases where it's needed separately)
    fn normal_at(&self, p: &Vec3) -> Vec3;

    /// Interact with the surface (could be for importance sampling, BSDF evaluation, etc.)
    fn interact(&self, hit: &HitRecord, incoming: &Vec3) -> SurfaceInteraction;

    // Get axis-aligned bounding box for BVH construction
    // fn bounding_box(&self) -> Option<AABB>;
}

pub struct ShapeInternal {
    transform: Transform,
    material: Material,
    colour: Colour,
}

#[derive(Clone, Debug)]
pub struct HitRecord {
    /// Distance along ray
    pub t: f64,

    /// Hit point
    pub point: Vec3,

    /// Surface normal
    pub normal: Vec3,

    /// Material at hit point
    pub material: Material,

    /// Texture coordinates
    pub uv: (f64, f64),

    /// Whether ray hit front face
    pub front_face: bool,
}

// Surface interaction result
pub struct SurfaceInteraction {
    pub scattered_direction: Option<Vec3>,
    pub attenuation: Vec3,
    pub pdf: f64,
}

impl HitRecord {
    pub fn shade(&self) -> Colour {
        todo!()
    }
}
