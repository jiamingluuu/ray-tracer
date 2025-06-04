use super::{matrix4::Matrix4, vec3::Vec3};

/// Transfomation used to convert the coordinate between camera and
/// world coordinate
#[derive(Debug, Default)]
pub struct Transform {
    /// Camera to world matrix
    pub mat: Matrix4,
    /// World to camera matrix
    pub inv: Matrix4,
}

impl Transform {
    #[rustfmt::skip]
    pub fn new(e: Vec3, u: Vec3, v: Vec3, w: Vec3) -> Self {
        // We want an orthongonal matrix M = [u v w] so we can easily compute
        // its inverse by M^T.
        u.normalize();
        v.normalize();
        w.normalize();
        let mat = Matrix4::new(
            [u[0], v[0], w[0], e[0],
             u[1], v[1], w[1], e[1],
             u[2], v[2], w[2], e[2],
             0.0,  0.0,  0.0,  1.0]
        );
        let inv = Matrix4::new(
            [u[0],       u[1],       u[2],       0.0,
             v[0],       v[1],       v[2],       0.0,
             w[0],       w[1],       w[2],       0.0,
             -u.dot(&e), -v.dot(&e), -w.dot(&e), 1.0]
        );
        assert!(mat * inv == Matrix4::new_identity());
        Transform {
            mat,
            inv,
        }
    }

    pub fn new_identity() -> Self {
        Self {
            mat: Matrix4::new_identity(),
            inv: Matrix4::new_identity(),
        }
    }
}
