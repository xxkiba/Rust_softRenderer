use std::fs::File;
use std::io::Read;
use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Default, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 4],  // model position
    pub tex_coord: [f32; 4], // UV coordinates
    pub normal: [f32; 4],    // normal
    pub tangent: [f32; 4],   // tangent
}

pub struct StaticMesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl StaticMesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut mesh = StaticMesh::new();
        let mut buf = [0u8; 4];

        // read vertices
        file.read_exact(&mut buf)?;
        let vertex_count = i32::from_le_bytes(buf);
        mesh.vertices.resize(vertex_count as usize, Vertex::default());
        let dest_bytes = bytemuck::cast_slice_mut::<Vertex, u8>(&mut mesh.vertices);
        file.read_exact(dest_bytes)?;

        // skip name
        file.read_exact(&mut buf)?;
        let name_len = i32::from_le_bytes(buf);
        let mut name = vec![0u8; name_len as usize];
        file.read_exact(&mut name)?;

        // read indices
        file.read_exact(&mut buf)?;
        let index_count = i32::from_le_bytes(buf);
        mesh.indices.resize(index_count as usize, 0);
        let dest_bytes = bytemuck::cast_slice_mut::<u32, u8>(&mut mesh.indices);
        file.read_exact(dest_bytes)?;

        Ok(mesh)
    }
}