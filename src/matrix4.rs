use crate::{float4::Float4, matrix3::Matrix3};

// use row vector convention, right-handed coordinate system
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4 {
    pub m: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn new() -> Self {
        Self {
            m: [[0.0; 4]; 4],
        }
    }

    
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn set_identity(&mut self) {
        *self = Self::identity();
    }

    
    pub fn set_upper_left_3x3(&mut self, m3: &Matrix3) {
        for i in 0..3 {
            for j in 0..3 {
                self.m[i][j] = m3.m[i][j];
            }
        }
    }

    pub fn get_upper_left_3x3(&self) -> Matrix3 {
        let mut result = Matrix3::new();
        for i in 0..3 {
            for j in 0..3 {
                result.m[i][j] = self.m[i][j];
            }
        }
        result
    }

    
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.m[3][0] = x; // _41
        self.m[3][1] = y; // _42
        self.m[3][2] = z; // _43
    }

    
    pub fn perspective(&mut self, fov_in_angle: f32, aspect: f32, near: f32, far: f32) {
        let half_angle_radians = fov_in_angle * 0.5 * std::f32::consts::PI / 180.0;
        let t = half_angle_radians.tan() * near;
        let r = t * aspect;

        self.m = [[0.0; 4]; 4];
        self.m[0][0] = near / r;                            // _11
        self.m[1][1] = near / t;                            // _22
        self.m[2][2] = (near + far) / (near - far);         // _33
        self.m[3][2] = (2.0 * near * far) / (near - far);  // _43
        self.m[2][3] = -1.0;                                // _34
    }

    
    pub fn look_at(&mut self, eye: (f32, f32, f32), target: (f32, f32, f32), up: (f32, f32, f32)) {
        let e = Float4::from_xyz((eye.0, eye.1, eye.2));
        let t = Float4::from_xyz((target.0, target.1, target.2));
        let up_vec = Float4::from_xyz((up.0, up.1, up.2));

        // z = cameraPosition - targetPosition because camera looks towards -z in view space
        let mut z = Float4::from_xyz((e.x - t.x, e.y - t.y, e.z - t.z));
        z.normalize();

        // x = up × z
        let mut x = up_vec.cross(&z);
        x.normalize();

        // y = z × x
        let mut y = z.cross(&x);
        y.normalize();

        // row vector convention: 4x4 matrix with rotation part in upper-left 3x3, translation in last row
        let mut rotate = Matrix4::identity();
        rotate.m[0][0] = x.x; rotate.m[0][1] = x.y; rotate.m[0][2] = x.z; // _11,_12,_13
        rotate.m[1][0] = y.x; rotate.m[1][1] = y.y; rotate.m[1][2] = y.z; // _21,_22,_23
        rotate.m[2][0] = z.x; rotate.m[2][1] = z.y; rotate.m[2][2] = z.z; // _31,_32,_33

        // translate matrix move eye to origin
        let mut translate = Matrix4::identity();
        translate.translate(eye.0, eye.1, eye.2);

        // C++：*this = (rotateMatrix * translateMatrix).Invert()
        *self = (rotate * translate).invert().unwrap_or(Matrix4::identity());
    }

    pub fn get_element(&self, row: usize, column: usize) -> f32 {
        self.m[row][column]
    }

    pub fn set_element(&mut self, row: usize, column: usize, value: f32) {
        self.m[row][column] = value;
    }

    // get the 3x3 minor matrix by excluding the specified row and column
    pub fn get_minor_3x3(&self, exclude_row: usize, exclude_col: usize) -> Matrix3 {
        let mut result = Matrix3::new();
        let mut r = 0;
        for i in 0..4 {
            if i == exclude_row { continue; }
            let mut c = 0;
            for j in 0..4 {
                if j == exclude_col { continue; }
                result.m[r][c] = self.m[i][j];
                c += 1;
            }
            r += 1;
        }
        result
    }

    // transpose in place
    pub fn transpose_in_place(&mut self) {
        let temp = self.m;
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] = temp[j][i];
            }
        }
    }

    pub fn transpose(&self) -> Self {
        let mut result = Self::new();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[j][i];
            }
        }
        result
    }

    // determinant
    // expand along the first row: det = _11*M11 - _12*M12 + _13*M13 - _14*M14
    pub fn determinant(&self) -> f32 {
        self.m[0][0] * self.get_minor_3x3(0, 0).determinant() -
        self.m[0][1] * self.get_minor_3x3(0, 1).determinant() +
        self.m[0][2] * self.get_minor_3x3(0, 2).determinant() -
        self.m[0][3] * self.get_minor_3x3(0, 3).determinant()
    }

    // inverse using adjugate and determinant: M^-1 = 1/det * adj(M)
    // adj(M) is the transpose of the cofactor matrix, where cofactor Cij = (-1)^(i+j) * Mij
    pub fn invert(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < 1e-6 {
            return None;
        }

        let mut result = Matrix4::new();
        let inv_det = 1.0 / det;

        // 计算代数余子式矩阵（注意C++里是先填result再Transpose）
        for i in 0..4 {
            for j in 0..4 {
                let sign = if (i + j) % 2 == 0 { 1.0f32 } else { -1.0f32 };
                // C++版：ret._ij = GetMIJ(i,j).Determinant()，然后整体Transpose
                // 所以这里直接写转置后的位置：result.m[j][i]
                result.m[j][i] = sign * self.get_minor_3x3(i, j).determinant() * inv_det;
            }
        }

        Some(result)
    }

    pub fn mul_scalar(&self, scalar: f32) -> Self {
        let mut result = *self;
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] *= scalar;
            }
        }
        result
    }
}

/// 对应 C++ matrix4::operator*(const matrix4&)
/// 标准矩阵乘法：result[i][j] = row_i · col_j
impl std::ops::Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Matrix4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[i][0] * rhs.m[0][j]
                    + self.m[i][1] * rhs.m[1][j]
                    + self.m[i][2] * rhs.m[2][j]
                    + self.m[i][3] * rhs.m[3][j];
            }
        }
        result
    }
}

impl std::ops::MulAssign for Matrix4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Mul<f32> for Matrix4 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self::Output {
        self.mul_scalar(scalar)
    }
}

impl std::ops::Mul<Matrix4> for f32 {
    type Output = Matrix4;
    fn mul(self, matrix: Matrix4) -> Matrix4 {
        matrix.mul_scalar(self)
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_mul() {
        let a = Matrix4::identity();
        let b = Matrix4::identity();
        let c = a * b;
        assert_eq!(c, Matrix4::identity());
    }

    #[test]
    fn test_translate() {
        let mut m = Matrix4::identity();
        m.translate(1.0, 2.0, 3.0);
        assert_eq!(m.m[3][0], 1.0); // _41
        assert_eq!(m.m[3][1], 2.0); // _42
        assert_eq!(m.m[3][2], 3.0); // _43
    }

    #[test]
    fn test_transpose() {
        let mut m = Matrix4::identity();
        m.m[0][1] = 5.0; // _12 = 5
        m.transpose_in_place();
        assert_eq!(m.m[1][0], 5.0); // _21 应该变成5
        assert_eq!(m.m[0][1], 0.0); // _12 应该变成0
    }

    #[test]
    fn test_determinant_identity() {
        let m = Matrix4::identity();
        assert!((m.determinant() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_invert_identity() {
        let m = Matrix4::identity();
        let inv = m.invert().unwrap();
        assert_eq!(inv, Matrix4::identity());
    }

    #[test]
    fn test_invert_mul() {
        // M * M^-1 应该等于单位矩阵
        let mut m = Matrix4::identity();
        m.translate(1.0, 2.0, 3.0);
        let inv = m.invert().unwrap();
        let result = m * inv;
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((result.m[i][j] - expected).abs() < 1e-5,
                    "m[{}][{}] = {} expected {}", i, j, result.m[i][j], expected);
            }
        }
    }
}