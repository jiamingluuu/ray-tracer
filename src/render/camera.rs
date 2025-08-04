use crate::{
    math::{transform::Transform, vec3::Vec3, vec4::Vec4},
    render::film::Film,
};

use super::ray::Ray;

pub struct Camera {
    /// Focal length.
    f: f64,
    e: Vec3,

    /// Camera-world transformation.
    camera_to_world: Transform,
    raster_to_camera: Transform,
    film: Film,
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
        resolution: (usize, usize),
        filename: String,
    ) -> Self {
        let w = -g.normalized();
        let u = t.cross(&w).normalized();
        let v = w.cross(&u);
        let camera_to_world = Transform::new(e, u, v, w);
        let raster_to_camera = Transform::new_identity();
        let film = Film::new(resolution, filename);
        Self {
            f,
            e,
            camera_to_world,
            raster_to_camera,
            film,
        }
    }

    pub fn get_camera_sample(&self, raster: (usize, usize)) -> Option<Ray> {
        if raster.0 >= self.film.resolution.0 || raster.1 >= self.film.resolution.1 {
            return None;
        }
        let r = Vec3::new(raster.0 as f64, raster.1 as f64 , 0.0);
        let p_camera = self.raster_to_camera.mat * r;
        let p = self.camera_to_world.mat * p_camera;
        let d = - (self.e - p);
        Some(Ray::new(p, d))
    }
}
