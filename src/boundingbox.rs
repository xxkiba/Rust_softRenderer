use crate::float4::Float4;

pub struct BoundingBox2D {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl BoundingBox2D {
    // construct a 2D bounding box from three vertices of a triangle in screen space
    pub fn from_triangle(a: &Float4, b: &Float4, c: &Float4,
                         viewport_width: i32, viewport_height: i32) -> Self {
        Self {
            min_x: a.x.min(b.x).min(c.x).floor().max(0.0),
            max_x: a.x.max(b.x).max(c.x).ceil().min(viewport_width as f32 - 1.0),
            min_y: a.y.min(b.y).min(c.y).floor().max(0.0),
            max_y: a.y.max(b.y).max(c.y).ceil().min(viewport_height as f32 - 1.0),
        }
    }

    // judge if the bounding box is valid (min < max)
    pub fn is_valid(&self) -> bool {
        self.min_x < self.max_x && self.min_y < self.max_y
    }
}

pub struct BoundingBox3D {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub min_z: f32,
    pub max_z: f32,
}

impl BoundingBox3D {
    // construct a 3D bounding box from three vertices (world space/model space)
    pub fn from_triangle(a: &Float4, b: &Float4, c: &Float4) -> Self {
        Self {
            min_x: a.x.min(b.x).min(c.x),
            max_x: a.x.max(b.x).max(c.x),
            min_y: a.y.min(b.y).min(c.y),
            max_y: a.y.max(b.y).max(c.y),
            min_z: a.z.min(b.z).min(c.z),
            max_z: a.z.max(b.z).max(c.z),
        }
    }

    // construct a 3D bounding box from multiple vertices (used for the entire mesh)
    pub fn from_vertices(vertices: &[Float4]) -> Option<Self> {
        // return None if there are no vertices
        let first = vertices.first()?;
        let mut bbox = Self {
            min_x: first.x, max_x: first.x,
            min_y: first.y, max_y: first.y,
            min_z: first.z, max_z: first.z,
        };
        for v in vertices.iter().skip(1) {
            bbox.min_x = bbox.min_x.min(v.x);
            bbox.max_x = bbox.max_x.max(v.x);
            bbox.min_y = bbox.min_y.min(v.y);
            bbox.max_y = bbox.max_y.max(v.y);
            bbox.min_z = bbox.min_z.min(v.z);
            bbox.max_z = bbox.max_z.max(v.z);
        }
        Some(bbox)
    }

    // judge if a point is inside the bounding box (used for coarse collision detection)
    pub fn contains(&self, point: &Float4) -> bool {
        point.x >= self.min_x && point.x <= self.max_x &&
        point.y >= self.min_y && point.y <= self.max_y &&
        point.z >= self.min_z && point.z <= self.max_z
    }

    // judge if two bounding boxes intersect
    pub fn intersects(&self, other: &BoundingBox3D) -> bool {
        self.min_x <= other.max_x && self.max_x >= other.min_x &&
        self.min_y <= other.max_y && self.max_y >= other.min_y &&
        self.min_z <= other.max_z && self.max_z >= other.min_z
    }
}