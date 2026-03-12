use std::fs::File;
use std::io::{Read};
use std::slice;
use std::mem;

#[derive(Clone, Copy, Default, Debug)]
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
        unsafe {
            let dest_bytes = slice::from_raw_parts_mut(
                mesh.vertices.as_mut_ptr() as *mut u8,
                vertex_count as usize * std::mem::size_of::<Vertex>(),
            );
            file.read_exact(dest_bytes)?;
        }

        // skip name
        file.read_exact(&mut buf)?;
        let name_len = i32::from_le_bytes(buf);
        let mut name = vec![0u8; name_len as usize];
        file.read_exact(&mut name)?;

        // read indices
        file.read_exact(&mut buf)?;
        let index_count = i32::from_le_bytes(buf);
        mesh.indices.resize(index_count as usize, 0);
        unsafe {
            let dest_bytes = slice::from_raw_parts_mut(
                mesh.indices.as_mut_ptr() as *mut u8,
                index_count as usize * std::mem::size_of::<u32>(),
            );
            file.read_exact(dest_bytes)?;
        }

        Ok(mesh)
    }
}