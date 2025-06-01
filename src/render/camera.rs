use crate::math::{point3::Point3, transform::Transform};

use super::ray::Ray;

pub struct Camera {
    /// The location of camera center.
    e: Point3,

    u: Point3,
    v: Point3,
    w: Point3,

    /// Focal length.
    f: f32,

    /// Camera-world transformation.
    transform: Transform,

    window_top: f32,
    window_left: f32,

    window_size: (f32, f32),
    resolution: (usize, usize),
}

impl Camera {
    /// e - the eye direction,
    /// g: the gaze direction,
    /// t: the upward direction
    pub fn new(
        e: Point3,
        g: Point3,
        t: Point3,
        f: f32,
        window_top: f32,
        window_left: f32,
        window_size: (f32, f32),
        resolution: (usize, usize),
    ) -> Self {
        let w = -g.normalized();
        let u = t.cross(&w).normalized();
        let v = w.cross(&u);
        let transform = Transform::new(e, u, v, w);

        Self {
            e,
            u,
            v,
            w,
            f,
            transform,
            window_top,
            window_left,
            window_size,
            resolution,
        }
    }

    pub fn cast_ray(&self, i: usize, j: usize) -> Ray {
        debug_assert!(i < self.resolution.0 && j < self.resolution.1);
        todo!()
    }
}
