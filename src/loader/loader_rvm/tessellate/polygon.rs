use nalgebra_glm::{Mat3, Vec3};

use crate::{
    loader::{loader_rvm::primitive::PolygonsData, TessellationOptions},
    structure::{IndexData, Mesh, Normal, Normals, Point3D, Positions, Primitives, Vertices},
    Length,
};

/// The polygons tessellation operator is used to tessellate a list of polygons defined by inner
/// and outer contours based on the specified tessellation options.
pub struct PolygonsTessellationOperator<'a> {
    polygon_data: &'a PolygonsData,

    max_edge_length_mm: Option<f32>,

    positions: Positions,
    normals: Normals,
    indices: Vec<u32>,
}

impl<'a> PolygonsTessellationOperator<'a> {
    /// Creates a new polygons tessellation operator. That is, an operator that tessellates a list
    /// of polygons with the given tessellation options.
    ///
    /// # Arguments
    /// * `polygon_data` - The polygons to be tessellated.
    /// * `tessellation_options` - The tessellation options to use for the tessellation.
    pub fn new(polygon_data: &'a PolygonsData, tessellation_options: &TessellationOptions) -> Self {
        let max_edge_length_mm = tessellation_options
            .max_length
            .map(|l| l.get_unit_in_meters() as f32 * 1e3f32);

        Self {
            polygon_data: polygon_data,
            max_edge_length_mm,
            positions: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Tessellates the polygon data for the specified transformation and translation.
    /// Function may only be called once.
    ///
    /// # Arguments
    /// * `transform` - The transformation matrix to apply to the polygon.
    /// * `translation` - The translation vector to apply to the polygon.
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

        todo!("Implement tessellation of polygons");

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
}

#[cfg(test)]
mod test {
    use super::*;
}
