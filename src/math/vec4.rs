use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use super::{transform::Transform, vec3::Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4 {
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan() || self.w.is_nan()
    }

    pub fn dot(&self, rhs: &Vec4) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Change from camera coordinate to world coordinate.
    pub fn to_world(&self, transform: &Transform) -> Self {
        transform.mat * self
    }

    /// Change from world coordinate to camera coordinate.
    pub fn to_camera(&self, transform: &Transform) -> Self {
        transform.inv * self
    }

    /// Convert to inhomogeneous coordinate.
    pub fn to_inhomo(&self) -> Vec3 {
        Vec3 {
            x: self.x / self.w,
            y: self.y / self.w,
            z: self.z / self.w,
        }
    }
}

impl Default for Vec4 {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<[f64; 3]> for Vec4 {
    fn from(arr: [f64; 3]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
            w: 1.0,
        }
    }
}

impl From<[f64; 4]> for Vec4 {
    fn from(arr: [f64; 4]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
            w: arr[3],
        }
    }
}

impl From<Vec4> for [f64; 4] {
    fn from(p: Vec4) -> Self {
        [p.x, p.y, p.z, p.w]
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Vec4) -> Self::Output {
        Vec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: Vec4) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: Vec4) -> Self::Output {
        Vec4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl SubAssign<Vec4> for Vec4 {
    fn sub_assign(&mut self, rhs: Vec4) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl Mul<f64> for Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Vec4> for f64 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec4 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Neg for Vec4 {
    type Output = Vec4;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}

impl Div<f64> for Vec4 {
    type Output = Vec4;
    fn div(self, rhs: f64) -> Self::Output {
        debug_assert!(rhs != 0.0, "Division by zero");
        let inv = 1.0 / rhs;
        Vec4 {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
            w: self.w,
        }
    }
}

impl DivAssign<f64> for Vec4 {
    fn div_assign(&mut self, rhs: f64) {
        debug_assert!(rhs != 0.0, "Division by zero");
        let inv = 1.0 / rhs;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
}

impl Index<usize> for Vec4 {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vec4: {}", idx),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds for Vec4: {}", idx),
        }
    }
}
