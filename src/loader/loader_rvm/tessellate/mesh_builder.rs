use nalgebra_glm::{Mat3, Vec3};

use crate::structure::{IndexData, Mesh, Normal, Point3D, Primitives, Vertices};

/// A builder for creating a mesh.
pub struct MeshBuilder {
    positions: Vec<Point3D>,
    normals: Vec<Normal>,
    indices: Vec<u32>,
}

impl Default for MeshBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshBuilder {
    /// Create a new mesh builder.
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Create a new mesh builder with the given capacity.
    ///
    /// # Arguments
    /// * `num_vertices` - The number of vertices to allocate space for.
    /// * `num_indices` - The number of indices to allocate space for.
    pub fn new_with_capacity(num_vertices: usize, num_indices: usize) -> Self {
        Self {
            positions: Vec::with_capacity(num_vertices),
            normals: Vec::with_capacity(num_vertices),
            indices: Vec::with_capacity(num_indices),
        }
    }

    /// Add new vertices to the mesh.
    /// Panics with an assertion error if the number of positions and normals do not match.
    /// Returns the index offset of the first vertex added.
    ///
    /// # Arguments
    /// * `positions` - The positions of the vertices.
    /// * `normals` - The normals of the vertices.
    pub fn add_vertices<P: IntoIterator<Item = Point3D>, N: IntoIterator<Item = Normal>>(
        &mut self,
        positions: P,
        normals: N,
    ) -> u32 {
        let index_offset = self.positions.len() as u32;

        self.positions.extend(positions);
        self.normals.extend(normals);

        assert_eq!(self.positions.len(), self.normals.len());

        index_offset
    }

    /// Adds a new vertex to the mesh and returns its index.
    ///
    /// # Arguments
    /// * `position` - The position of the vertex.
    /// * `normal` - The normal of the vertex.
    pub fn add_vertex(&mut self, position: Point3D, normal: Normal) -> u32 {
        let index = self.positions.len() as u32;

        self.positions.push(position);
        self.normals.push(normal);

        index
    }

    /// Add a triangle to the mesh.
    ///
    /// # Arguments
    /// * `t` - The indices of the vertices of the triangle.
    pub fn add_triangle(&mut self, t: &[u32; 3]) {
        assert!(t[0] < self.positions.len() as u32);
        assert!(t[1] < self.positions.len() as u32);
        assert!(t[2] < self.positions.len() as u32);

        self.indices.extend_from_slice(t);
    }

    /// Add triangles to the mesh.
    ///
    /// # Arguments
    /// * `triangles` - The indices of the vertices of the triangles.
    pub fn add_triangles_from_slice(&mut self, triangles: &[u32]) {
        assert_eq!(triangles.len() % 3, 0);
        assert!(
            triangles.iter().all(|&i| i < self.positions.len() as u32),
            "Index out of bounds"
        );
        self.indices.extend_from_slice(triangles);
    }

    /// Transforms the mesh using the given transform and translation.
    ///
    /// # Arguments
    /// * `transform` - The transform to apply to the vertices.Vec
    /// * `translation` - The translation to apply to the vertices.
    pub fn transform_vertices(&mut self, transform: &Mat3, translation: &Vec3) {
        // Apply the transformation and translation to the positions.
        self.positions.iter_mut().for_each(|p| {
            p.0 = transform * p.0 + translation;
        });

        // Transform the normals using the normal transformation matrix.
        let normal_mat = transform.transpose().try_inverse().unwrap();
        self.normals.iter_mut().for_each(|n| {
            n.0 = (normal_mat * n.0).normalize();
        });
    }

    /// Transforms the mesh builder into a mesh.
    pub fn into_mesh(self) -> Mesh {
        assert_eq!(self.positions.len(), self.normals.len());

        let mut vertices = Vertices::from_positions(self.positions);
        vertices.set_normals(self.normals).unwrap();

        let index_data = IndexData::Indices(self.indices);
        let primitives =
            Primitives::new(index_data, crate::structure::PrimitiveType::Triangles).unwrap();

        Mesh::new(vertices, primitives).expect("Failed to create mesh")
    }

    /// Returns the number of vertices in the mesh builder.
    #[inline]
    pub fn vertices_len(&self) -> usize {
        self.positions.len()
    }

    /// Returns the positions of the mesh builder.
    #[inline]
    pub fn positions(&self) -> &[Point3D] {
        &self.positions
    }

    /// Returns true if the mesh builder is empty.
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty() && self.normals.is_empty() && self.indices.is_empty()
    }
}
