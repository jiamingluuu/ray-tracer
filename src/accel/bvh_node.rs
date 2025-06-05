use super::aabb::AABB;

pub struct BVHNode {
    left: Box<Option<BVHNode>>,
    right: Box<Option<BVHNode>>,
    bbox: AABB,
}
