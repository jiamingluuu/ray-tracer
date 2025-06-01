use super::{matrix4::Matrix4, point3::Point3};

pub struct Transform {
    pub mat: Matrix4,
    pub inv: Matrix4,
}

impl Transform {
    pub fn new(e: Point3, u: Point3, v: Point3, w: Point3) -> Self {
        let mut m = Matrix4::new_zero();
        m[0] = u[0];
        m[4] = u[1];
        m[8] = u[2];

        m[1] = v[0];
        m[5] = v[1];
        m[9] = v[2];

        m[2] = w[0];
        m[6] = w[1];
        m[10] = w[2];

        m[3] = e[0];
        m[7] = e[1];
        m[11] = e[2];

        m[15] = 1.0;

        Transform {
            mat: m,
            inv: m.inv(),
        }
    }
}
