use cylinder::CylinderTessellationOperator;
use mesh_builder::MeshBuilder;
use nalgebra_glm::{Mat3, Vec3};
use polygon::PolygonsTessellationOperator;
use sphere::SphereTessellationOperator;

mod cylinder;
mod mesh_builder;
mod polygon;
mod sphere;

use crate::{
    loader::TessellationOptions,
    structure::{Mesh, Point3D},
    Error,
};

use super::primitive::{BoxData, CylinderData, PolygonsData, PyramidData, SphereData};

/// The tessellate trait is used to convert a CAD model to a mesh.
pub trait Tessellate {
    /// Tessellates the CAD model to a mesh for the provided options.
    ///
    /// # Arguments
    /// * `options` - The options for tessellating the CAD model.
    /// * `transform` - The transformation matrix to apply to the mesh.
    /// * `translation` - The translation vector to apply to the mesh.
    fn tessellate(
        &self,
        options: &TessellationOptions,
        transform: &Mat3,
        translation: &Vec3,
    ) -> Result<Mesh, Error>;
}

impl Tessellate for BoxData {
    fn tessellate(
        &self,
        _: &TessellationOptions,
        transform: &Mat3,
        translation: &Vec3,
    ) -> Result<Mesh, Error> {
        let mut mesh_builder = MeshBuilder::new();

        let dx = self.inner[0] / 2.0;
        let dy = self.inner[1] / 2.0;
        let dz = self.inner[2] / 2.0;

        let indices = [
            // Front
            0, 1, 2, 2, 3, 0, // Back
            4, 5, 6, 6, 7, 4, // Left
            8, 9, 10, 10, 11, 8, // Right
            12, 13, 14, 14, 15, 12, // Top
            16, 17, 18, 18, 19, 16, // Bottom
            20, 21, 22, 22, 23, 20,
        ];

        let positions = [
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

        let normals = [
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

        mesh_builder.add_vertices(positions, normals);
        mesh_builder.add_triangles_from_slice(&indices);
        mesh_builder.transform_vertices(transform, translation);

        let mesh = mesh_builder.into_mesh();

        Ok(mesh)
    }
}

impl Tessellate for CylinderData {
    fn tessellate(
        &self,
        t: &TessellationOptions,
        transform: &Mat3,
        translation: &Vec3,
    ) -> Result<Mesh, Error> {
        let mut tessellation_operator = CylinderTessellationOperator::new(self, t);
        tessellation_operator.tessellate(transform, translation);

        let mesh = tessellation_operator.into_mesh();

        Ok(mesh)
    }
}

impl Tessellate for SphereData {
    fn tessellate(
        &self,
        t: &TessellationOptions,
        transform: &Mat3,
        translation: &Vec3,
    ) -> Result<Mesh, Error> {
        let mut tessellation_operator = SphereTessellationOperator::new(self, t);
        tessellation_operator.tessellate(transform, translation);

        let mesh = tessellation_operator.into_mesh();

        Ok(mesh)
    }
}

impl Tessellate for PolygonsData {
    fn tessellate(
        &self,
        t: &TessellationOptions,
        transform: &Mat3,
        translation: &Vec3,
    ) -> Result<Mesh, Error> {
        let mut tessellation_operator = PolygonsTessellationOperator::new(self, t);
        tessellation_operator.tessellate(transform, translation);

        let mesh = tessellation_operator.into_mesh();

        Ok(mesh)
    }
}

/// The base positions for a pyramid.
const PYRAMID_BASE_POS: [f32; 24] = [
    0.5, 0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5,
    0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5,
];

impl Tessellate for PyramidData {
    fn tessellate(
        &self,
        t: &TessellationOptions,
        transform: &Mat3,
        translation: &Vec3,
    ) -> Result<Mesh, Error> {
        let positions: Vec<Point3D> = Vec::new();
        let normals: Vec<Point3D> = Vec::new();
        let indices: Vec<u32> = Vec::new();

        // create coordinates of the pyramid based on the base positions
        let mut points: Vec<Point3D> = PYRAMID_BASE_POS
            .chunks_exact(3)
            .enumerate()
            .map(|(i, pyramid_point)| {
                let x = if i < 4 {
                    pyramid_point[0] * self.xbottom() - self.xoffset() * 0.5f32
                } else {
                    pyramid_point[0] * self.xtop() + self.xoffset() * 0.5f32
                };

                let y = if i < 4 {
                    pyramid_point[1] * self.ybottom() - self.yoffset() * 0.5f32
                } else {
                    pyramid_point[1] * self.ytop() + self.yoffset() * 0.5f32
                };

                let z = pyramid_point[2] * self.height();

                Point3D::new(x, y, z)
            })
            .collect();

        // tessellate the sides of the pyramid
        for i in 0..4usize {}

        todo!("Implement tessellation of the pyramid.");
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
        let transform = Mat3::identity();
        let translation = Vec3::zeros();
        let mesh = box_data
            .tessellate(&options, &transform, &translation)
            .unwrap();

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

            assert_eq!(normal, n0, "Normal is not consistent.");
            assert_eq!(normal, n1, "Normal is not consistent.");
            assert_eq!(normal, n2, "Normal is not consistent.");

            // make sure the normal is pointing outwards
            assert!(c.dot(&normal) > 0f32, "Normal is not pointing outwards.");
        }

        assert_eq!(total_area, 24f32);
    }
}
