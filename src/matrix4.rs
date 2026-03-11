use crate::{float4::Float4, matrix3::Matrix3};

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
        self.m = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
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
        self.m[3][0] = x;
        self.m[3][1] = y;
        self.m[3][2] = z;
    }
    pub fn perspective(&mut self, fov_y: f32, aspect: f32, near: f32, far: f32) {
        let halfAngle=fov_y.to_radians()/2.0;
        let tanHalfAngle = halfAngle.tan();

        self.m = [
            [1.0 / (tanHalfAngle*aspect), 0.0, 0.0, 0.0],
            [0.0, 1.0/tanHalfAngle, 0.0, 0.0],
            [0.0, 0.0, far / (far - near), 1.0],
            [0.0, 0.0, -(far * near) / (far - near), 0.0],
        ];
    }

    pub fn look_at(&mut self, eye: (f32, f32, f32), center: (f32, f32, f32), up: (f32, f32, f32)) {
        let (ex, ey, ez) = eye;
        let (cx, cy, cz) = center;
        let (ux, uy, uz) = up;

        let mut e : Float4 = Float4::from_xyz((ex, ey, ez));
        let mut z : Float4 = Float4::from_xyz((cx - ex, cy - ey, cz - ez));
        z.normalize();
        let mut up : Float4 = Float4::from_xyz((ux, uy, uz));
        up.normalize();
        let mut x : Float4 = up.cross(&z);
        x.normalize();
        let mut y : Float4 = z.cross(&x);
        y.normalize();

        self.m = [
            [x.x, y.x, z.x, 0.0],
            [x.y, y.y, z.y, 0.0],
            [x.z, y.z, z.z, 0.0],
            [-x.dot(&e), -y.dot(&e), -z.dot(&e), 1.0],
        ];
    }

    pub fn get_element(&self, row: usize, column: usize) -> f32 {
        self.m[row][column]
    }

    pub fn set_element(&mut self, row: usize, column: usize, value: f32) {
        self.m[row][column] = value;
    }

    pub fn get_minor_3x3(&self, exclude_row: usize, exclude_column: usize) -> Matrix3 {
        let mut result = Matrix3::new();
        let mut r = 0;

        for i in 0..4 {
            if i == exclude_row {
                continue;
            }
            let mut c = 0;
            for j in 0..4 {
                if j == exclude_column {
                    continue;
                }
                result.m[r][c] = self.m[i][j];
                c += 1;
            }
            r += 1;
        }

        result
    }
    pub fn transpose(&mut self) {
        let temp = self.m;
        self.m = [
            [temp[0][0], temp[1][0], temp[2][0], temp[3][0]],
            [temp[0][1], temp[1][1], temp[2][1], temp[3][1]],
            [temp[0][2], temp[1][2], temp[2][2], temp[3][2]],
            [temp[0][3], temp[1][3], temp[2][3], temp[3][3]],
        ];
    }

    pub fn determinant(&self) -> f32 {
        let mut det = 0.0;
        for i in 0..4 {
            let minor = self.get_minor_3x3(0, i);
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            det += sign * self.m[0][i] * minor.determinant();
        }
        det
    }

    pub fn invert(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < 1e-6 {
            return None;
        }

        let mut result = Matrix4::new();
        let inv_det = 1.0 / det;

        for i in 0..4 {
            for j in 0..4 {
                let minor = self.get_minor_3x3(j, i);
                let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
                result.m[i][j] = sign * minor.determinant() * inv_det;
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