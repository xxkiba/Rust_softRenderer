#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Float4 {

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn from_xyzw(values: (f32, f32, f32, f32)) -> Self {
        Float4 {
            x: values.0,
            y: values.1,
            z: values.2,
            w: values.3,
        }
    }

    pub fn from_xyz(values: (f32, f32, f32)) -> Self {
        Float4 {
            x: values.0,
            y: values.1,
            z: values.2,
            w: 0.0,
        }
    }

    pub fn from_array3(arr: [f32; 3]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
            w: 0.0,
        }
    }
    pub fn to_array3(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn from_array4(arr: [f32; 4]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
            w: arr[3],
        }
    }
    pub fn to_array4(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    pub fn set_xyz(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }


    pub fn xyz(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    pub fn set_xyzw(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }

    pub fn length(&self) -> f32 {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        }
    }

    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
            w: self.w.min(other.w),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
            w: self.w.max(other.w),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
            w: self.w.abs(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.length_squared() < 1e-6
    }

    pub fn is_near_zero(&self, epsilon: f32) -> bool {
        self.length_squared() < epsilon * epsilon
    }

}

impl std::ops::Add for Float4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl std::ops::AddAssign for Float4{

    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl std::ops::Sub for Float4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl std::ops::SubAssign for Float4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl std::ops::Mul<f32> for Float4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl std::ops::Mul<Float4> for f32 {
    type Output = Float4;

    fn mul(self, rhs: Float4) -> Self::Output {
        Float4 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
            w: rhs.w * self,
        }
    }
}

impl std::ops::MulAssign<f32> for Float4 {

    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
    }
}

impl std::ops::Div<f32> for Float4 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

impl std::ops::DivAssign<f32> for Float4 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
        self.w /= scalar;
    }
}

impl std::ops::Neg for Float4 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

// Float4 * Matrix4 
impl std::ops::Mul<crate::matrix4::Matrix4> for Float4 {
    type Output = Float4;

    fn mul(self, matrix: crate::matrix4::Matrix4) -> Self::Output {
        Float4 {
            x: self.x * matrix.m[0][0] + self.y * matrix.m[1][0] + self.z * matrix.m[2][0] + self.w * matrix.m[3][0],
            y: self.x * matrix.m[0][1] + self.y * matrix.m[1][1] + self.z * matrix.m[2][1] + self.w * matrix.m[3][1],
            z: self.x * matrix.m[0][2] + self.y * matrix.m[1][2] + self.z * matrix.m[2][2] + self.w * matrix.m[3][2],
            w: self.x * matrix.m[0][3] + self.y * matrix.m[1][3] + self.z * matrix.m[2][3] + self.w * matrix.m[3][3],
        }
    }
}

// Float4 * &Matrix4 
impl std::ops::Mul<&crate::matrix4::Matrix4> for Float4 {
    type Output = Float4;

    fn mul(self, matrix: &crate::matrix4::Matrix4) -> Self::Output {
        Float4 {
            x: self.x * matrix.m[0][0] + self.y * matrix.m[1][0] + self.z * matrix.m[2][0] + self.w * matrix.m[3][0],
            y: self.x * matrix.m[0][1] + self.y * matrix.m[1][1] + self.z * matrix.m[2][1] + self.w * matrix.m[3][1],
            z: self.x * matrix.m[0][2] + self.y * matrix.m[1][2] + self.z * matrix.m[2][2] + self.w * matrix.m[3][2],
            w: self.x * matrix.m[0][3] + self.y * matrix.m[1][3] + self.z * matrix.m[2][3] + self.w * matrix.m[3][3],
        }
    }
}

impl std::ops::Index<usize> for Float4{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds for Float4"),
        }
    }
}

impl std::ops::IndexMut<usize> for Float4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds for Float4"),
        }
    }
}