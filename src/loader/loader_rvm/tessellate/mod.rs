use nalgebra_glm::{Mat3, Vec3};

use crate::{
    loader::TessellationOptions,
    structure::{IndexData, Mesh, Point3D, Primitives, Vertices},
    Error,
};

use super::primitive::BoxData;

/// The tessellate trait is used to convert a CAD model to a mesh.
pub trait Tessellate {
    /// Tessellates the CAD model to a mesh for the provided options.
    ///
    /// # Arguments
    /// * `options` - The options for tessellating the CAD model.
    fn tessellate(&self, options: &TessellationOptions) -> Result<Mesh, Error>;
}

/// Extracts the translation from the given matrix.
///
/// # Arguments
/// * `matrix` - The matrix from which the translation is extracted.
#[inline]
fn extract_translation(matrix: &[f32; 12]) -> Vec3 {
    Vec3::from_column_slice(&matrix[9..12])
}

/// Extracts the 3x3 transformation matrix from the given 3x4 matrix.
///
/// # Arguments
/// * `matrix` - The matrix from which the transformation matrix is extracted.
fn extract_transformation(matrix: &[f32; 12]) -> Mat3 {
    Mat3::from_column_slice(&matrix[..9])
}

impl Tessellate for BoxData {
    fn tessellate(&self, _: &TessellationOptions) -> Result<Mesh, Error> {
        let dx = self.inner[0] / 2.0;
        let dy = self.inner[1] / 2.0;
        let dz = self.inner[2] / 2.0;

        let indices = vec![
            // Front
            0, 1, 2, 2, 3, 0, // Back
            4, 5, 6, 6, 7, 4, // Left
            8, 9, 10, 10, 11, 8, // Right
            12, 13, 14, 14, 15, 12, // Top
            16, 17, 18, 18, 19, 16, // Bottom
            20, 21, 22, 22, 23, 20,
        ];
        let positions = vec![
            // Front
            Point3D::new(dx, dy, dz),
            Point3D::new(-dx, dy, dz),
            Point3D::new(-dx, -dy, dz),
            Point3D::new(dx, -dy, dz),
            // Back
            Point3D::new(-dx, dy, -dz),
            Point3D::new(dx, dy, -dz),
            Point3D::new(dx, -dy, -dz),
            Point3D::new(-dx, -dy, -dz),
            // Left
            Point3D::new(-dx, dy, dz),
            Point3D::new(-dx, dy, -dz),
            Point3D::new(-dx, -dy, -dz),
            Point3D::new(-dx, -dy, dz),
            // Right
            Point3D::new(dx, dy, -dz),
            Point3D::new(dx, dy, dz),
            Point3D::new(dx, -dy, dz),
            Point3D::new(dx, -dy, -dz),
            // Top
            Point3D::new(dx, dy, -dz),
            Point3D::new(-dx, dy, -dz),
            Point3D::new(-dx, dy, dz),
            Point3D::new(dx, dy, dz),
            // Bottom
            Point3D::new(-dx, -dy, -dz),
            Point3D::new(dx, -dy, -dz),
            Point3D::new(dx, -dy, dz),
            Point3D::new(-dx, -dy, dz),
        ];
        let normals = vec![
            // Front
            Point3D::new(0f32, 0f32, 1f32),
            Point3D::new(0f32, 0f32, 1f32),
            Point3D::new(0f32, 0f32, 1f32),
            Point3D::new(0f32, 0f32, 1f32),
            // Back
            Point3D::new(0f32, 0f32, -1f32),
            Point3D::new(0f32, 0f32, -1f32),
            Point3D::new(0f32, 0f32, -1f32),
            Point3D::new(0f32, 0f32, -1f32),
            // Left
            Point3D::new(-1f32, 0f32, 0f32),
            Point3D::new(-1f32, 0f32, 0f32),
            Point3D::new(-1f32, 0f32, 0f32),
            Point3D::new(-1f32, 0f32, 0f32),
            // Right
            Point3D::new(1f32, 0f32, 0f32),
            Point3D::new(1f32, 0f32, 0f32),
            Point3D::new(1f32, 0f32, 0f32),
            Point3D::new(1f32, 0f32, 0f32),
            // Top
            Point3D::new(0f32, 1f32, 0f32),
            Point3D::new(0f32, 1f32, 0f32),
            Point3D::new(0f32, 1f32, 0f32),
            Point3D::new(0f32, 1f32, 0f32),
            // Bottom
            Point3D::new(0f32, -1f32, 0f32),
            Point3D::new(0f32, -1f32, 0f32),
            Point3D::new(0f32, -1f32, 0f32),
            Point3D::new(0f32, -1f32, 0f32),
        ];

        let index_data = IndexData::Indices(indices);
        let mut vertices = Vertices::from_positions(positions);
        vertices.set_normals(normals).unwrap();
        let primitives =
            Primitives::new(index_data, crate::structure::PrimitiveType::Triangles).unwrap();
        let mesh = Mesh::new(vertices, primitives).unwrap();

        Ok(mesh)
    }
}

#[cfg(test)]
mod test {
    use crate::structure::PrimitiveType;

    use super::*;

    /// Tests the tessellated box and checks if the normals and faces are correct.
    #[test]
    fn test_tessellate_box() {
        let box_data = BoxData {
            inner: [2.0, 2.0, 2.0],
        };
        let options = TessellationOptions::default();
        let mesh = box_data.tessellate(&options).unwrap();

        assert_eq!(mesh.get_vertices().get_positions().len(), 24);
        assert_eq!(mesh.get_vertices().get_normals().unwrap().len(), 24);
        assert_eq!(mesh.get_primitives().get_raw_index_data().num_indices(), 36);

        let primitives = mesh.get_primitives();
        assert_eq!(primitives.get_primitive_type(), PrimitiveType::Triangles);

        let indices = primitives.get_raw_index_data().get_indices_ref().unwrap();

        // iterate over the triangles by 3 indices at a time
        let mut total_area = 0f32;
        let positions = mesh.get_vertices().get_positions();
        let normals = mesh.get_vertices().get_normals().unwrap();
        for triangle in indices.windows(3).step_by(3) {
            let c = positions[triangle[0] as usize].0;
            let a = positions[triangle[1] as usize].0 - c;
            let b = positions[triangle[2] as usize].0 - c;

            let n0 = normals[triangle[0] as usize].0;
            let n1 = normals[triangle[1] as usize].0;
            let n2 = normals[triangle[2] as usize].0;

            let cross = a.cross(&b);
            total_area += cross.norm() / 2f32;

            let normal = cross.normalize();

            assert_eq!(normal, n0);
            assert_eq!(normal, n1);
            assert_eq!(normal, n2);
        }

        assert_eq!(total_area, 24f32);
    }
}
