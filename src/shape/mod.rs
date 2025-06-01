pub mod cube;
pub mod cylinder;
pub mod sphere;
pub mod triangle;

use crate::math::point3::Point3;

trait Shape {
    fn intersect(&self) -> Option<Point3>;
}
