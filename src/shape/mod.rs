pub mod cube;
pub mod cylinder;
pub mod primitive;
pub mod sphere;
pub mod triangle;

use primitive::Primitive;

use crate::{accel::aabb::AABB, math::vec3::Vec3, render::ray::Ray};

pub trait Shape {
    /// Test ray-shape intersection in object space
    fn intersect_local(&self, ray: &Ray) -> Option<LocalHitRecord>;
}

pub trait Sampleable {
    /// Uniformly sample a position and normal on the surface using the samples passed
    fn sample_uniform(&self, samples: &(f32, f32)) -> Vec3;

    /// Sample the object using the probability density of the solid angle
    /// from `p` to the sampled point on the surface.
    /// Returns the sampled point and the surface normal at that point
    fn sample(&self, p: &Vec3, samples: &(f32, f32)) -> Vec3;

    /// Return the surface area of the shape
    fn surface_area(&self) -> f32;

    /// Compute the PDF that the ray from `p` with direction `w_i` intersects the shape
    fn pdf(&self, p: &Vec3, w_i: &Vec3) -> f32;
}

pub trait Boundable {
    fn bounds(&self) -> AABB;
}

pub trait Geometry: Shape + Sampleable + Boundable {}

/// Local space hit record (before transformation)
pub struct LocalHitRecord {
    pub t: f64,
    pub p: Vec3,
    pub n: Vec3,
    pub uv: (f64, f64),
}

pub struct HitRecord<'a> {
    /// Time of the ray propagate (always positive).
    pub t: f64,

    /// Hit point.
    pub p: Vec3,

    /// Surface normal at hit point.
    pub n: Vec3,

    /// Material at the hit point.
    pub primitive: &'a Primitive,

    /// Texture coordinates.
    pub uv: (f64, f64),
}
