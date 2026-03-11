#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Matrix3 {
    pub m: [[f32; 3]; 3],
}

impl Matrix3 {

    pub fn new() -> Self {
        Self {
             m:[[0.0;3];3],
        }
    }

    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }  
    }

    pub fn set_identity(&mut self) {
        self.m = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
    }

    pub fn set_scale(&mut self, x:f32, y:f32, z:f32) {
        self.m[0][0] = x;
        self.m[1][1] = y;
        self.m[2][2] = z;
    }

    pub fn determinant(&self) -> f32 {
        self.m[0][0] * (self.m[1][1] * self.m[2][2] - self.m[1][2] * self.m[2][1]) -
        self.m[0][1] * (self.m[1][0] * self.m[2][2] - self.m[1][2] * self.m[2][0]) +
        self.m[0][2] * (self.m[1][0] * self.m[2][1] - self.m[1][1] * self.m[2][0])
    }

    pub fn transpose(&self) -> Self {
        Self{
            m:[
                [self.m[0][0], self.m[1][0], self.m[2][0]],
                [self.m[0][1], self.m[1][1], self.m[2][1]],
                [self.m[0][2], self.m[1][2], self.m[2][2]],
            ],
        }
    }

    pub fn transpose_in_place(&mut self){
        let temp = self.m;
        self.m = [
            [temp[0][0], temp[1][0], temp[2][0]],
            [temp[0][1], temp[1][1], temp[2][1]],
            [temp[0][2], temp[1][2], temp[2][2]],
        ];
    }
}

impl std::ops::Mul for Matrix3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Matrix3::new();

        for i in 0..3 {
            for j in 0..3 {
                result.m[i][j] = self.m[i][0] * rhs.m[0][j]
                    + self.m[i][1] * rhs.m[1][j]
                    + self.m[i][2] * rhs.m[2][j];
            }
        }

        result
    }
}

impl std::ops::MulAssign for Matrix3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Default for Matrix3 {
    fn default() -> Self {
        Self::identity()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let m = Matrix3::identity();
        assert_eq!(m.m[0][0], 1.0);
        assert_eq!(m.m[1][1], 1.0);
        assert_eq!(m.m[2][2], 1.0);
        assert_eq!(m.m[0][1], 0.0);
    }

    #[test]
    fn test_set_identity() {
        let mut m = Matrix3::new();
        m.set_identity();
        assert_eq!(m.m[0][0], 1.0);
        assert_eq!(m.m[1][1], 1.0);
        assert_eq!(m.m[2][2], 1.0);
    }

    #[test]
    fn test_scale() {
        let mut m = Matrix3::identity();
        m.set_scale(2.0, 3.0, 4.0);
        assert_eq!(m.m[0][0], 2.0);
        assert_eq!(m.m[1][1], 3.0);
        assert_eq!(m.m[2][2], 4.0);
    }

    #[test]
    fn test_mul() {
        let a = Matrix3::identity();
        let b = Matrix3::identity();
        let c = a * b;
        assert_eq!(c.m[0][0], 1.0);
        assert_eq!(c.m[1][1], 1.0);
        assert_eq!(c.m[2][2], 1.0);
    }

    #[test]
    fn test_determinant() {
        let m = Matrix3::identity();
        assert_eq!(m.determinant(), 1.0);

        let mut scale = Matrix3::identity();
        scale.set_scale(2.0, 3.0, 4.0);
        assert_eq!(scale.determinant(), 24.0);
    }

    #[test]
    fn test_transpose() {
        let mut m = Matrix3::new();
        m.m[0][1] = 1.0;
        m.m[1][0] = 2.0;

        let t = m.transpose();
        assert_eq!(t.m[1][0], 1.0);
        assert_eq!(t.m[0][1], 2.0);
    }
}
