use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use super::point3::Point3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Point4 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan() || self.w.is_nan()
    }

    #[inline]
    pub fn dot(&self, rhs: &Point4) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn to_inhomo(&self) -> Point3 {
        Point3 {
            x: self.x / self.z,
            y: self.y / self.z,
            z: self.z / self.z,
        }
    }
}

impl Default for Point4 {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl From<[f32; 4]> for Point4 {
    #[inline]
    fn from(arr: [f32; 4]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
            w: arr[3],
        }
    }
}

impl From<Point4> for [f32; 4] {
    #[inline]
    fn from(p: Point4) -> Self {
        [p.x, p.y, p.z, p.w]
    }
}

impl Add<Point4> for Point4 {
    type Output = Point4;
    #[inline]
    fn add(self, rhs: Point4) -> Self::Output {
        Point4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl AddAssign<Point4> for Point4 {
    #[inline]
    fn add_assign(&mut self, rhs: Point4) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Sub<Point4> for Point4 {
    type Output = Point4;
    #[inline]
    fn sub(self, rhs: Point4) -> Self::Output {
        Point4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl SubAssign<Point4> for Point4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Point4) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl Mul<f32> for Point4 {
    type Output = Point4;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Point4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Point4> for f32 {
    type Output = Point4;
    #[inline]
    fn mul(self, rhs: Point4) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Point4 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Point4 {
    type Output = Point4;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        debug_assert!(rhs != 0.0, "Division by zero");
        let inv = 1.0 / rhs;
        Point4 {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
            w: self.w,
        }
    }
}

impl DivAssign<f32> for Point4 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        debug_assert!(rhs != 0.0, "Division by zero");
        let inv = 1.0 / rhs;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
}

impl Index<usize> for Point4 {
    type Output = f32;
    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Point4: {}", idx),
        }
    }
}

impl IndexMut<usize> for Point4 {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds for Point4: {}", idx),
        }
    }
}
