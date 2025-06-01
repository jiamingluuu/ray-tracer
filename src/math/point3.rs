use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use super::point4::Point4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    #[inline]
    pub fn dot(&self, rhs: &Point3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn cross(&self, rhs: &Point3) -> Point3 {
        Point3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn normalize(&self) -> Point3 {
        let len = self.length();
        debug_assert!(len > 0.0, "Cannot normalize zero-length vector");
        *self / len
    }

    #[inline]
    pub fn normalized(&self) -> Point3 {
        let len = self.length();
        debug_assert!(len > 0.0, "Cannot normalize zero-length vector");
        *self / len
    }

    #[inline]
    pub fn normalize_or_zero(&self) -> Point3 {
        let len_sq = self.length_squared();
        if len_sq > f32::EPSILON {
            *self / len_sq.sqrt()
        } else {
            Point3::zero()
        }
    }
}

impl Default for Point3 {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl From<[f32; 3]> for Point3 {
    #[inline]
    fn from(arr: [f32; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}

impl From<Point3> for [f32; 3] {
    #[inline]
    fn from(p: Point3) -> Self {
        [p.x, p.y, p.z]
    }
}

impl From<(f32, f32, f32)> for Point3 {
    #[inline]
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self::new(x, y, z)
    }
}

impl From<Point3> for (f32, f32, f32) {
    #[inline]
    fn from(p: Point3) -> Self {
        (p.x, p.y, p.z)
    }
}

impl From<Point4> for Point3 {
    #[inline]
    fn from(p: Point4) -> Self {
        Self {
            x: p.x / p.w,
            y: p.y / p.w,
            z: p.z / p.w,
        }
    }
}

impl Add<Point3> for Point3 {
    type Output = Point3;
    #[inline]
    fn add(self, other: Point3) -> Self::Output {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign<Point3> for Point3 {
    #[inline]
    fn add_assign(&mut self, other: Point3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub<Point3> for Point3 {
    type Output = Point3;
    #[inline]
    fn sub(self, other: Point3) -> Self::Output {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign<Point3> for Point3 {
    #[inline]
    fn sub_assign(&mut self, v: Point3) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }
}

impl Mul<f32> for Point3 {
    type Output = Point3;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Point3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Point3> for f32 {
    type Output = Point3;
    #[inline]
    fn mul(self, rhs: Point3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Point3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Point3 {
    type Output = Point3;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        debug_assert!(rhs != 0.0, "Division by zero");
        let inv = 1.0 / rhs;
        Point3 {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }
}

impl DivAssign<f32> for Point3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        debug_assert!(rhs != 0.0, "Division by zero");
        let inv = 1.0 / rhs;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
}

impl Index<usize> for Point3 {
    type Output = f32;
    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Point3: {}", i),
        }
    }
}

impl IndexMut<usize> for Point3 {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Point3: {}", i),
        }
    }
}

impl std::ops::Neg for Point3 {
    type Output = Point3;
    #[inline]
    fn neg(self) -> Self::Output {
        Point3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Point3> for Point3 {
    type Output = Point3;
    #[inline]
    fn mul(self, rhs: Point3) -> Self::Output {
        Point3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<Point3> for Point3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Point3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Point3 {
    pub const X_AXIS: Point3 = Point3::new(1.0, 0.0, 0.0);
    pub const Y_AXIS: Point3 = Point3::new(0.0, 1.0, 0.0);
    pub const Z_AXIS: Point3 = Point3::new(0.0, 0.0, 1.0);
    pub const ONE: Point3 = Point3::new(1.0, 1.0, 1.0);
}
