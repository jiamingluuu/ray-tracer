use crate::math::{transform::Transform, vec3::Vec3, vec4::Vec4};

use super::ray::Ray;

pub struct Camera {
    /// The location of camera center.
    pub e: Vec3,
    /// The gaze direction.
    pub g: Vec3,
    /// The up vector.
    pub t: Vec3,

    /// Focal length.
    pub f: f64,

    /// Camera-world transformation.
    pub transform: Transform,

    pub window_top: f64,
    pub window_left: f64,
    pub image_size: (f64, f64),

    pub resolution: (usize, usize),
}

impl Camera {
    /// e - the eye direction,
    /// g: the gaze direction,
    /// t: the upward direction
    /// f: focus distance
    pub fn new(
        e: Vec3,
        g: Vec3,
        t: Vec3,
        f: f64,
        window_top: f64,
        window_left: f64,
        image_size: (f64, f64),
        resolution: (usize, usize),
    ) -> Self {
        let w = -g.normalized();
        let u = t.cross(&w).normalized();
        let v = w.cross(&u);
        let transform = Transform::new(e, u, v, w);

        Self {
            e,
            g,
            t,
            f,
            transform,
            window_top,
            window_left,
            image_size,
            resolution,
        }
    }

    pub fn cast_ray(&self, i: usize, j: usize) -> Ray {
        debug_assert!(i < self.resolution.0 && j < self.resolution.1);
        todo!()
    }
}

pub struct CameraIterator<'a> {
    /// The coordinate the the camera ray in Raster Space.
    pixel: (usize, usize),
    /// The camera used for generating ray.
    camera: &'a Camera,

    du: f64,
    dv: f64,
}

impl<'a> CameraIterator<'a> {
    pub fn new(camera: &'a Camera) -> Self {
        let du = camera.image_size.0 / (camera.resolution.0 - 1) as f64;
        let dv = -1.0 * camera.image_size.1 / (camera.resolution.1 - 1) as f64;
        Self {
            pixel: (0, 0),
            camera,
            du,
            dv,
        }
    }

    pub fn next(&mut self) -> Option<Ray> {
        if self.pixel.0 + 1 == self.camera.resolution.0 {
            self.pixel.0 = 0;
            self.pixel.1 += 1;
        }
        if self.pixel.1 == self.camera.resolution.1 {
            return None;
        }

        let p = Vec4::new(
            self.camera.window_left + self.pixel.0 as f64 * self.du,
            self.camera.window_top + self.pixel.1 as f64 * self.dv,
            self.camera.f,
            1.0,
        )
        .to_world(&self.camera.transform)
        .to_inhomo()
        .normalize();
        let d = p - self.camera.e;
        Some(Ray { p, d })
    }
}
