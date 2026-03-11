pub struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Float4 {
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
}