use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use crate::math::vec4::Vec4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4([f64; 16]);

impl Default for Matrix4 {
    fn default() -> Self {
        Self::new_identity()
    }
}

impl Matrix4 {
    pub fn new(data: [f64; 16]) -> Self {
        Self(data)
    }

    pub fn data(&self) -> &[f64; 16] {
        &self.0
    }

    /// Returns the inverse of this matrix
    /// Panics if the matrix is not invertible (determinant is 0)
    #[rustfmt::skip]
    pub fn inv(&self) -> Self {
        // Calculate the cofactor matrix
        let m = &self.0;
        let mut cofactors = [0.0; 16];

        // Helper macro to get 3x3 determinant for the cofactor
        macro_rules! det3x3 {
            ($a:expr, $b:expr, $c:expr,
             $d:expr, $e:expr, $f:expr,
             $g:expr, $h:expr, $i:expr) => {
                  $a * ($e * $i - $f * $h)
                - $b * ($d * $i - $f * $g)
                + $c * ($d * $h - $e * $g)
            };
        }

        // Calculate cofactors with signs cofactors[0] = det3x3!(m[5], m[6], m[7], m[9], m[10], m[11], m[13], m[14], m[15]);
        cofactors[1] = -det3x3!(m[4], m[6], m[7], m[8], m[10], m[11], m[12], m[14], m[15]);
        cofactors[2] = det3x3!(m[4], m[5], m[7], m[8], m[9], m[11], m[12], m[13], m[15]);
        cofactors[3] = -det3x3!(m[4], m[5], m[6], m[8], m[9], m[10], m[12], m[13], m[14]);

        cofactors[4] = -det3x3!(m[1], m[2], m[3], m[9], m[10], m[11], m[13], m[14], m[15]);
        cofactors[5] = det3x3!(m[0], m[2], m[3], m[8], m[10], m[11], m[12], m[14], m[15]);
        cofactors[6] = -det3x3!(m[0], m[1], m[3], m[8], m[9], m[11], m[12], m[13], m[15]);
        cofactors[7] = det3x3!(m[0], m[1], m[2], m[8], m[9], m[10], m[12], m[13], m[14]);

        cofactors[8] = det3x3!(m[1], m[2], m[3], m[5], m[6], m[7], m[13], m[14], m[15]);
        cofactors[9] = -det3x3!(m[0], m[2], m[3], m[4], m[6], m[7], m[12], m[14], m[15]);
        cofactors[10] = det3x3!(m[0], m[1], m[3], m[4], m[5], m[7], m[12], m[13], m[15]);
        cofactors[11] = -det3x3!(m[0], m[1], m[2], m[4], m[5], m[6], m[12], m[13], m[14]);

        cofactors[12] = -det3x3!(m[1], m[2], m[3], m[5], m[6], m[7], m[9], m[10], m[11]);
        cofactors[13] = det3x3!(m[0], m[2], m[3], m[4], m[6], m[7], m[8], m[10], m[11]);
        cofactors[14] = -det3x3!(m[0], m[1], m[3], m[4], m[5], m[7], m[8], m[9], m[11]);
        cofactors[15] = det3x3!(m[0], m[1], m[2], m[4], m[5], m[6], m[8], m[9], m[10]);

        // Calculate determinant using first row cofactor expansion
        let det = m[0] * cofactors[0] + m[1] * cofactors[4] + m[2] * cofactors[8] + m[3] * cofactors[12];

        assert!(det.abs() > f64::EPSILON, "Matrix is not invertible");

        // Divide adjugate matrix by determinant to get inverse
        let inv_det = 1.0 / det;
        for cofactor in &mut cofactors {
            *cofactor *= inv_det;
        }

        // Transpose to get final inverse
        Matrix4::new([
            cofactors[0], cofactors[4], cofactors[8], cofactors[12],
            cofactors[1], cofactors[5], cofactors[9], cofactors[13],
            cofactors[2], cofactors[6], cofactors[10], cofactors[14],
            cofactors[3], cofactors[7], cofactors[11], cofactors[15],
        ])
    }

    #[rustfmt::skip]
    pub fn new_zero() -> Self {
        Self([
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
        ])
    }

    #[rustfmt::skip]
    pub fn new_identity() -> Self {
        Self([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }
    
    #[rustfmt::skip]
    pub fn new_translate(v: [f64; 3]) -> Self {
        Self([
            1.0, 0.0, 0.0, v[0],
            0.0, 1.0, 0.0, v[1],
            0.0, 0.0, 1.0, v[2],
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[rustfmt::skip]
    pub fn new_scale_x(x: f64) -> Self {
        let mut m = Self::new_identity();
        m.0[0] = x;
        m
    }

    #[rustfmt::skip]
    pub fn new_scale_y(y: f64) -> Self {
        let mut m = Self::new_identity();
        m.0[5] = y;
        m
    }

    #[rustfmt::skip]
    pub fn new_scale_z(z: f64) -> Self {
        let mut m = Self::new_identity();
        m.0[10] = z;
        m
    }

    #[rustfmt::skip]
    pub fn new_rotate_x(theta: f64) -> Self {
        let mut m = Self::new_identity();
        m.0[5] = theta.cos();
        m.0[6] = -theta.sin();
        m.0[9] = theta.sin();
        m.0[10] = theta.cos();
        m
    }

    #[rustfmt::skip]
    pub fn new_rotate_y(theta: f64) -> Self {
        let mut m = Self::new_identity();
        m.0[0] = theta.cos();
        m.0[2] = theta.sin();
        m.0[8] = -theta.sin();
        m.0[10] = theta.cos();
        m
    }

    #[rustfmt::skip]
    pub fn new_rotate_z(theta: f64) -> Self {
        let mut m = Self::new_identity();
        m.0[0] = theta.cos();
        m.0[1] = -theta.sin();
        m.0[4] = theta.sin();
        m.0[5] = theta.cos();
        m
    }

    /// Scale along the X-axis
    #[rustfmt::skip]
    pub fn scale_x(&mut self, x: f64) {
        let scale_matrix = Matrix4([
            x,   0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);
        *self *= scale_matrix;
    }

    /// Scale along the Y-axis
    #[rustfmt::skip]
    pub fn scale_y(&mut self, y: f64) {
        let scale_matrix = Matrix4([
            1.0, 0.0, 0.0, 0.0,
            0.0, y,   0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);
        *self *= scale_matrix;
    }

    /// Scale along the Z-axis
    #[rustfmt::skip]
    pub fn scale_z(&mut self, z: f64) {
        let scale_matrix = Matrix4([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, z,   0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);
        *self *= scale_matrix;
    }

    /// Rotate around the X-axis (pitch)
    /// Angle is in radians
    #[rustfmt::skip]
    pub fn rotate_x(&mut self, angle: f64) {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let rotation_matrix = Matrix4([
            1.0, 0.0,    0.0,    0.0,
            0.0, cos_a, -sin_a,  0.0,
            0.0, sin_a,  cos_a,  0.0,
            0.0, 0.0,    0.0,    1.0,
        ]);
        *self *= rotation_matrix;
    }

    /// Rotate around the Y-axis (yaw)
    /// Angle is in radians
    #[rustfmt::skip]
    pub fn rotate_y(&mut self, angle: f64) {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let rotation_matrix = Matrix4([
            cos_a,  0.0, sin_a, 0.0,
            0.0,    1.0, 0.0,   0.0,
            -sin_a, 0.0, cos_a, 0.0,
            0.0,    0.0, 0.0,   1.0,
        ]);
        *self *= rotation_matrix;
    }

    /// Rotate around the Z-axis (roll)
    /// Angle is in radians
    #[rustfmt::skip]
    pub fn rotate_z(&mut self, angle: f64) {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let rotation_matrix = Matrix4([
            cos_a, -sin_a, 0.0, 0.0,
            sin_a,  cos_a, 0.0, 0.0,
            0.0,    0.0,   1.0, 0.0,
            0.0,    0.0,   0.0, 1.0,
        ]);
        *self *= rotation_matrix;
    }

    /// Scale uniformly along all axes
    #[rustfmt::skip]
    pub fn scale_uniform(&mut self, scale: f64) {
        self.scale_x(scale);
        self.scale_y(scale);
        self.scale_z(scale);
    }

    /// Apply scaling along all three axes at once
    #[rustfmt::skip]
    pub fn scale(&mut self, x: f64, y: f64, z: f64) {
        let scale_matrix = Matrix4([
            x,   0.0, 0.0, 0.0,
            0.0, y,   0.0, 0.0,
            0.0, 0.0, z,   0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);
        *self *= scale_matrix;
    }

    /// Apply translation
    #[rustfmt::skip]
    pub fn translate(&mut self, x: f64, y: f64, z: f64) {
        let translation_matrix = Matrix4([
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        ]);
        *self *= translation_matrix;
    }
}

impl Index<usize> for Matrix4 {
    type Output = [f64];

    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < 4, "Row index {} out of bounds for 4x4 matrix", row);
        let start = row * 4;
        &self.0[start..start + 4]
    }
}

// Implement IndexMut for write access: m[row][col] = value
impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(row < 4, "Row index {} out of bounds for 4x4 matrix", row);
        let start = row * 4;
        &mut self.0[start..start + 4]
    }
}

// Basic operators for owned values
impl Add for Matrix4 {
    type Output = Matrix4;

    fn add(self, rhs: Matrix4) -> Self::Output {
        let mut result = [0.0; 16];
        for (i, r) in result.iter_mut().enumerate() {
            *r = self.0[i] + rhs.0[i];
        }
        Matrix4(result)
    }
}

impl Sub for Matrix4 {
    type Output = Matrix4;

    fn sub(self, rhs: Matrix4) -> Self::Output {
        let mut result = [0.0; 16];
        for (i, r) in result.iter_mut().enumerate() {
            *r = self.0[i] - rhs.0[i];
        }
        Matrix4(result)
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut result = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.0[i * 4 + k] * rhs.0[k * 4 + j];
                }
                result[i * 4 + j] = sum;
            }
        }
        Matrix4(result)
    }
}

// Reference operators
impl Add<Matrix4> for &Matrix4 {
    type Output = Matrix4;

    fn add(self, rhs: Matrix4) -> Self::Output {
        *self + rhs
    }
}

impl Add<&Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn add(self, rhs: &Matrix4) -> Self::Output {
        self + *rhs
    }
}

impl Add<&Matrix4> for &Matrix4 {
    type Output = Matrix4;

    fn add(self, rhs: &Matrix4) -> Self::Output {
        *self + *rhs
    }
}

impl Sub<Matrix4> for &Matrix4 {
    type Output = Matrix4;

    fn sub(self, rhs: Matrix4) -> Self::Output {
        *self - rhs
    }
}

impl Sub<&Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn sub(self, rhs: &Matrix4) -> Self::Output {
        self - *rhs
    }
}

impl Sub<&Matrix4> for &Matrix4 {
    type Output = Matrix4;

    fn sub(self, rhs: &Matrix4) -> Self::Output {
        *self - *rhs
    }
}

impl Mul<Matrix4> for &Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        *self * rhs
    }
}

impl Mul<&Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: &Matrix4) -> Self::Output {
        self * *rhs
    }
}

impl Mul<&Matrix4> for &Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: &Matrix4) -> Self::Output {
        *self * *rhs
    }
}

// Assignment operators for owned and referenced values
impl AddAssign for Matrix4 {
    fn add_assign(&mut self, rhs: Matrix4) {
        for i in 0..16 {
            self.0[i] += rhs.0[i];
        }
    }
}

impl AddAssign<&Matrix4> for Matrix4 {
    fn add_assign(&mut self, rhs: &Matrix4) {
        for i in 0..16 {
            self.0[i] += rhs.0[i];
        }
    }
}

impl SubAssign for Matrix4 {
    fn sub_assign(&mut self, rhs: Matrix4) {
        for i in 0..16 {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl SubAssign<&Matrix4> for Matrix4 {
    fn sub_assign(&mut self, rhs: &Matrix4) {
        for i in 0..16 {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl MulAssign for Matrix4 {
    fn mul_assign(&mut self, rhs: Matrix4) {
        *self = *self * rhs;
    }
}

impl MulAssign<&Matrix4> for Matrix4 {
    fn mul_assign(&mut self, rhs: &Matrix4) {
        *self = *self * rhs;
    }
}

// Scalar operations
impl Mul<f64> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, scalar: f64) -> Self::Output {
        let mut result: [f64; 16] = [0.0; 16];
        for (i, r) in result.iter_mut().enumerate() {
            *r = self.0[i] * scalar;
        }
        Matrix4(result)
    }
}

impl Mul<f64> for &Matrix4 {
    type Output = Matrix4;

    fn mul(self, scalar: f64) -> Self::Output {
        *self * scalar
    }
}

impl Mul<Matrix4> for f64 {
    type Output = Matrix4;

    fn mul(self, matrix: Matrix4) -> Self::Output {
        matrix * self
    }
}

impl Mul<&Matrix4> for f64 {
    type Output = Matrix4;

    fn mul(self, matrix: &Matrix4) -> Self::Output {
        *matrix * self
    }
}

impl MulAssign<f64> for Matrix4 {
    fn mul_assign(&mut self, scalar: f64) {
        for i in 0..16 {
            self.0[i] *= scalar;
        }
    }
}

// Matrix4 * Vec4 multiplication
impl Mul<Vec4> for Matrix4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let mut result = [0.0; 4];
        for i in 0..4 {
            let mut sum = 0.0;
            for j in 0..4 {
                sum += self.0[i * 4 + j]
                    * match j {
                        0 => rhs.x,
                        1 => rhs.y,
                        2 => rhs.z,
                        3 => rhs.w,
                        _ => unreachable!(),
                    };
            }
            result[i] = sum;
        }
        Vec4::new(result[0], result[1], result[2], result[3])
    }
}

impl Mul<&Vec4> for Matrix4 {
    type Output = Vec4;

    fn mul(self, rhs: &Vec4) -> Self::Output {
        self * *rhs
    }
}

impl Mul<Vec4> for &Matrix4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        *self * rhs
    }
}

impl Mul<&Vec4> for &Matrix4 {
    type Output = Vec4;

    fn mul(self, rhs: &Vec4) -> Self::Output {
        *self * *rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_matrix_inverse() {
        // Test identity matrix inversion
        let identity = Matrix4::new_identity();
        assert_eq!(identity.inv().data(), identity.data());

        // Test translation matrix inversion
        let translation = Matrix4::new_translate([2.0, -3.0, 4.0]);
        let expected = Matrix4::new_translate([-2.0, 3.0, -4.0]);
        assert_eq!(translation.inv().data(), expected.data());

        // Test scaling matrix inversion
        let scale = Matrix4::new_scale_x(3.0);
        let expected_scale_inv = Matrix4::new_scale_x(1.0 / 3.0);
        assert_eq!(scale.inv().data(), expected_scale_inv.data());

        // Test singular matrix (should return zero matrix)
        // TODO: Catch the panic in the testcase.
        let singular = Matrix4::new([
            1.0, 2.0, 3.0, 4.0, 
            2.0, 4.0, 6.0, 8.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
        ]);
        assert_eq!(singular.inv().data(), &[0.0; 16]);

        // Test rotation matrix inversion (should be transpose)
        let angle = std::f64::consts::PI / 4.0; // 45 degrees
        let c = angle.cos();
        let s = angle.sin();
        let rotation = Matrix4::new([
            c, -s, 0.0, 0.0, s, c, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]);
        let expected_rot_inv = Matrix4::new([
            c, s, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]);

        let rot_inv = rotation.inv();
        for (a, b) in rot_inv.data().iter().zip(expected_rot_inv.data().iter()) {
            assert!((a - b).abs() < 1e-6);
        }

        // Test inverse of inverse equals original matrix
        let transform = Matrix4::new([
            1.0, 2.0, 3.0, 4.0, 0.0, 1.0, 5.0, 6.0, 0.0, 0.0, 1.0, 7.0, 0.0, 0.0, 0.0, 1.0,
        ]);
        let inv_inv = transform.inv().inv();
        for (a, b) in transform.data().iter().zip(inv_inv.data().iter()) {
            assert!((a - b).abs() < 1e-6);
        }
    }

    #[test]
    fn test_addition() {
        let a = Matrix4::new([1.0; 16]);
        let b = Matrix4::new([2.0; 16]);
        let c = a + b;

        for &val in c.data() {
            assert_eq!(val, 3.0);
        }
    }

    #[test]
    fn test_subtraction() {
        let a = Matrix4::new([5.0; 16]);
        let b = Matrix4::new([2.0; 16]);
        let c = a - b;

        for &val in c.data() {
            assert_eq!(val, 3.0);
        }
    }

    #[test]
    fn test_matrix_multiplication() {
        let identity = Matrix4::new_identity();
        let a = Matrix4::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ]);

        let result = identity * a;
        assert_eq!(result, a);
    }

    #[test]
    #[rustfmt::skip]
    fn test_scalar_multiplication() {
        let a = Matrix4::new([
             1.0,  2.0,  3.0,  4.0,
             5.0,  6.0,  7.0,  8.0,
             9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0,
        ]);
        let result = a * 2.0;
        let expected = Matrix4::new([
             2.0,  4.0,  6.0,  8.0,
            10.0, 12.0, 14.0, 16.0,
            18.0, 20.0, 22.0, 24.0,
            26.0, 28.0, 30.0, 32.0,
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_assignment_operators() {
        let mut a = Matrix4::new([1.0; 16]);
        let b = Matrix4::new([2.0; 16]);

        a += &b;
        for &val in a.data() {
            assert_eq!(val, 3.0);
        }

        a -= &b;
        for &val in a.data() {
            assert_eq!(val, 1.0);
        }

        a *= 3.0;
        for &val in a.data() {
            assert_eq!(val, 3.0);
        }
    }

    #[test]
    fn test_matrix_point_multiplication() {
        // Test translation matrix
        let translation = Matrix4::new([
            1.0, 0.0, 0.0, 2.0, // translate x by 2
            0.0, 1.0, 0.0, 3.0, // translate y by 3
            0.0, 0.0, 1.0, 4.0, // translate z by 4
            0.0, 0.0, 0.0, 1.0,
        ]);
        let p = Vec4::new(1.0, 1.0, 1.0, 1.0);
        let result = translation * p;
        assert_eq!(result, Vec4::new(3.0, 4.0, 5.0, 1.0));

        // Test scaling matrix
        let scaling = Matrix4::new([
            2.0, 0.0, 0.0, 0.0, // scale x by 2
            0.0, 3.0, 0.0, 0.0, // scale y by 3
            0.0, 0.0, 4.0, 0.0, // scale z by 4
            0.0, 0.0, 0.0, 1.0,
        ]);
        let result = scaling * p;
        assert_eq!(result, Vec4::new(2.0, 3.0, 4.0, 1.0));

        // Test with references
        let result_ref = translation * p;
        assert_eq!(result_ref, Vec4::new(3.0, 4.0, 5.0, 1.0));
    }
}
