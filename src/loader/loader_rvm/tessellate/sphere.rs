use nalgebra_glm::{Mat3, Vec3};

use crate::{
    loader::{loader_rvm::primitive::SphereData, TessellationOptions},
    structure::{IndexData, Mesh, Normals, Positions, Primitives, Vertices},
    Length,
};

/// The vertices of an icosahedron.
const ICOSAHEDRON_VERTICES: [Vec3; 12] = [
    Vec3::new(0.0, 0.8506508, 0.5257311),
    Vec3::new(0.0, 0.8506508, -0.5257311),
    Vec3::new(0.0, -0.8506508, 0.5257311),
    Vec3::new(0.0, -0.8506508, -0.5257311),
    Vec3::new(0.8506508, 0.5257311, 0.0),
    Vec3::new(0.8506508, -0.5257311, 0.0),
    Vec3::new(-0.8506508, 0.5257311, 0.0),
    Vec3::new(-0.8506508, -0.5257311, 0.0),
    Vec3::new(0.5257311, 0.0, 0.8506508),
    Vec3::new(-0.5257311, 0.0, 0.8506508),
    Vec3::new(0.5257311, 0.0, -0.8506508),
    Vec3::new(-0.5257311, 0.0, -0.8506508),
];

/// The indices of the icosahedron.
const ICOSAHEDRON_INDICES: [u32; 60] = [
    1, 0, 4, 0, 1, 6, 2, 3, 5, 3, 2, 7, 4, 5, 10, 5, 4, 8, 6, 7, 9, 7, 6, 11, 8, 9, 2, 9, 8, 0, 10,
    11, 1, 11, 10, 3, 0, 8, 4, 0, 6, 9, 1, 4, 10, 1, 11, 6, 2, 5, 8, 2, 9, 7, 3, 10, 5, 3, 7, 11,
];

/// The cylinder tessellation operator is used to tessellate a cylinder based on the specified
/// cylinder data and tessellation options.
pub struct SphereTessellationOperator {
    radius_mm: f32,

    /// In order to guarantee all tessellation options are met, it is sufficient to only define the
    /// maximum edge length.
    max_edge_length: f32,

    positions: Positions,
    normals: Normals,
    indices: Vec<u32>,
}

impl SphereTessellationOperator {
    /// Creates a new sphere tessellation operator. That is, an operator that tessellates a
    /// sphere based on the specified sphere data and tessellation options.
    ///
    /// # Arguments
    /// * `sphere_data` - The cylinder data to use for the tessellation.
    /// * `tessellation_options` - The tessellation options to use for the tessellation.
    pub fn new(sphere_data: &SphereData, tessellation_options: &TessellationOptions) -> Self {
        let radius_mm = sphere_data.diameter() / 2.0;

        let max_edge_length = Self::determine_maximum_edge_length(
            Length::new(radius_mm as f64 * 1e-3f64),
            tessellation_options,
        );

        Self {
            radius_mm,
            max_edge_length,
            positions: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Tessellates the sphere based on the specified transformation and translation.
    /// Function may only be called once.
    ///
    /// # Arguments
    /// * `transform` - The transformation matrix to apply to the cylinder.
    /// * `translation` - The translation vector to apply to the cylinder.
    pub fn tessellate(&mut self, transform: &Mat3, translation: &Vec3) {
        assert!(
            self.positions.is_empty(),
            "Tesselation has already been performed."
        );
        assert!(
            self.normals.is_empty(),
            "Tesselation has already been performed."
        );
        assert!(
            self.indices.is_empty(),
            "Tesselation has already been performed."
        );

        // Apply the transformation and translation to the positions.
        self.positions.iter_mut().for_each(|p| {
            p.0 = transform * p.0 + translation;
        });

        // Transform the normals using the normal transformation matrix.
        let normal_mat = transform.transpose().try_inverse().unwrap();
        self.normals.iter_mut().for_each(|n| {
            n.0 = (normal_mat * n.0).normalize();
        });

        assert_eq!(self.positions.len(), self.normals.len());
    }

    /// Converts the tessellated sphere into a mesh object.
    pub fn into_mesh(self) -> Mesh {
        let index_data = IndexData::Indices(self.indices);
        let mut vertices = Vertices::from_positions(self.positions);
        vertices.set_normals(self.normals).unwrap();
        let primitives =
            Primitives::new(index_data, crate::structure::PrimitiveType::Triangles).unwrap();
        Mesh::new(vertices, primitives).expect("Failed to create mesh")
    }

    /// Determines the maximum edge length based on the specified radius and tessellation options
    /// in millimeters.
    ///
    /// # Arguments
    /// * `radius` - The radius of the sphere.
    /// * `tessellation_options` - The tessellation options to use for determining the maximum edge
    fn determine_maximum_edge_length(
        radius: Length,
        tessellation_options: &TessellationOptions,
    ) -> f32 {
        let mut max_length_mm = radius.get_unit_in_meters() as f32 * 1e3f32;

        if let Some(max_length) = tessellation_options.max_length {
            max_length_mm = max_length_mm.min(max_length.get_unit_in_meters() as f32 * 1e3f32);
        }

        max_length_mm
    }
}
