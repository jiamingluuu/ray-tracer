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

fn main() {
    let options = Options::parse();

    let e = Vec3::new(6.0, -6.0, 1.0);
    let g = -e;
    let t = Vec3::new(0.0, 0.0, 1.0);
    let f = 1.0;

    let camera = Camera::new(e, g, t, f, 1.0, 1.0, (1.0, 1.0), options.resolution);
    let mut image = Image::new(options.resolution);

    let scene = Scene::new();
    let mut iter = CameraIterator::new(&camera);
    while let Some(r) = iter.next() {
        // TODO: Multi-sampling for the ray
        // - Antialasing
        let record = scene.find_first_hit(&r);
        if record.is_none() {
            continue;
        }
        let colour = record.unwrap().shade();
        image.set_colour(&iter.pixel, colour);
    }
}
