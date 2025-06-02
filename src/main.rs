use clap::Parser;

use RayTracer::{
    common::options::Options,
    math::vec3::Vec3,
    render::{camera::Camera, image::Image},
};

fn main() {
    let options = Options::parse();

    let e = Vec3::new(6.0, -6.0, 1.0);
    let g = -e;
    let t = Vec3::new(0.0, 0.0, 1.0);
    let f = 1.0;

    let camera = Camera::new(e, g, t, f, 1.0, 1.0, (1.0, 1.0), options.resolution);
    let image = Image::new(options.resolution);
}
