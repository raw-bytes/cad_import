use crate::error::Error;

use super::{primitives::Primitives, vertices::Vertices};

/// A mesh is a tessellated geometry consisting of vertices and primitives.
pub struct Mesh {
    vertices: Vertices,
    primitives: Primitives,
}

impl Mesh {
    /// Creates a new mesh object from the given vertices and primitives.
    ///
    /// # Arguments
    /// * `vertices` - The vertices of the mesh
    /// * `primitives` - The mesh primitives
    pub fn new(vertices: Vertices, primitives: Primitives) -> Result<Self, Error> {
        match primitives.max_index() {
            Some(m) => {
                if m as usize >= vertices.len() {
                    return Err(Error::Indices(format!(
                        "Indices reference vertex {}, but only got {} vertices",
                        m,
                        vertices.len()
                    )));
                }
            }
            None => {}
        }

        Ok(Self {
            vertices,
            primitives,
        })
    }

    /// Returns a reference onto the vertices.
    pub fn get_vertices(&self) -> &Vertices {
        &self.vertices
    }

    /// Returns a reference onto the primitives.
    pub fn get_primitives(&self) -> &Primitives {
        &self.primitives
    }
}
