use std::path::Path;

use clap::Parser;

use ray_tracer::{
    common::options::Options,
    math::vec3::Vec3,
    render::{
        camera::{Camera, CameraIterator},
        image::Image,
    },
    scene::Scene,
};

fn main() -> std::io::Result<()> {
    let options = Options::parse();
    // An upright camera looking at origin with, locate at 'e'
    let e = Vec3::new(6.0, -6.0, 1.0);
    let g = -e;
    let t = Vec3::new(0.0, 0.0, 1.0);
    let f = 1.0;
    let window_top = 1.0;
    let window_left = 1.0;
    let image_size = (1.0, 1.0);
    let camera = Camera::new(e, g, t, f, window_top, window_left, image_size, options.resolution);

    let mut image = Image::new(options.resolution);
    let scene = Scene::new();
    let path = Path::new("./output.ppm");
    image.save_to_file(path)?;
    Ok(())
}
