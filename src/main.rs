use race::{
    common::options::Options,
    math::{point3::Point3, point4::Point4},
    render::camera::Camera,
};

fn main() {
    let options = Options::parse();
    let u = Point3::new(1, 0, 0);
    let v = Point3::new(0, 1, 0);
    let w = Point3::new(0, 0, 1);

    let e = Point4::new(6, -6, 1);
    let g = -e;
    let t = Point4::new(0, 0, 1);
    let camera = Camera::new(e, g, t, options.f, 1.0, 1.0, options.resolution);
}
