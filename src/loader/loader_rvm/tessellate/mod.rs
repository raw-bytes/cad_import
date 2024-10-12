use arrayvec::ArrayVec;
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
    structure::{Mesh, Normal, Point3D},
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

        const INDICES: [u32; 36] = [
            0, 1, 2, 2, 3, 0, // Front
            4, 5, 6, 6, 7, 4, // Back
            8, 9, 10, 10, 11, 8, // Left
            12, 13, 14, 14, 15, 12, // Right
            16, 17, 18, 18, 19, 16, // Top
            20, 21, 22, 22, 23, 20, // Bottom
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

        const NORMALS: [Point3D; 24] = [
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

        mesh_builder.add_vertices_from_slice(&positions, &NORMALS);
        mesh_builder.add_triangles_from_slice(&INDICES);
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
        _: &TessellationOptions,
        transform: &Mat3,
        translation: &Vec3,
    ) -> Result<Mesh, Error> {
        let mut mesh_builder = MeshBuilder::new();

        // create coordinates of the pyramid based
        let points: ArrayVec<Vec3, 24> =
            ArrayVec::from_iter(PYRAMID_BASE_POS.chunks_exact(3).enumerate().map(
                |(i, pyramid_point)| {
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

                    Vec3::new(x, y, z)
                },
            ));

        // tessellate the sides of the pyramid
        for i0 in 0..4usize {
            // define the other points of the triangle
            let i1 = (i0 + 1) % 4;
            let j0 = i0 + 4;
            let j1 = i1 + 4;

            // extract the points for the current side of the pyramid
            let p0 = Point3D(points[i0]);
            let p1 = Point3D(points[i1]);
            let p2 = Point3D(points[j0]);
            let p3 = Point3D(points[j1]);

            // determine triangles should be created, i.e., the triangles should not be degenerate
            let t0 = !p0.eq(&p1) && !p1.eq(&p2) && !p0.eq(&p2);
            let t1 = !p1.eq(&p3) && !p3.eq(&p2) && !p1.eq(&p2);

            // add first triangle
            if t0 {
                let normal = Normal {
                    0: (p2.0 - p0.0).cross(&(p1.0 - p0.0)).normalize(),
                };
                let vertex_offset =
                    mesh_builder.add_vertices_from_slice(&[p0, p1, p2], &[normal; 3]);
                mesh_builder.add_triangle(&[vertex_offset, vertex_offset + 2, vertex_offset + 1]);
            }

            // add second triangle
            if t1 {
                let normal = Normal {
                    0: (p2.0 - p1.0).cross(&(p3.0 - p1.0)).normalize(),
                };

                let vertex_offset = if t0 {
                    mesh_builder.add_vertex(p3, normal) - 2
                } else {
                    mesh_builder.add_vertices_from_slice(&[p1, p2, p3], &[normal; 3])
                };

                mesh_builder.add_triangle(&[vertex_offset, vertex_offset + 1, vertex_offset + 2]);
            }
        }

        // add bottom cap
        if !points[0].eq(&points[1]) && !points[1].eq(&points[2]) && !points[0].eq(&points[2]) {
            let n0 = (points[1] - points[0])
                .cross(&(points[2] - points[0]))
                .normalize();
            let n1 = (points[2] - points[0])
                .cross(&(points[3] - points[0]))
                .normalize();
            let normal = Normal {
                0: (n0 + n1).normalize(),
            };

            let vertex_offset = mesh_builder.add_vertices_from_slice(
                &[
                    Point3D(points[0]),
                    Point3D(points[1]),
                    Point3D(points[2]),
                    Point3D(points[3]),
                ],
                &[normal; 4],
            );

            mesh_builder.add_triangle(&[vertex_offset, vertex_offset + 1, vertex_offset + 2]);
            mesh_builder.add_triangle(&[vertex_offset, vertex_offset + 2, vertex_offset + 3]);
        }

        // add top cap
        if !points[4].eq(&points[5]) && !points[5].eq(&points[6]) && !points[4].eq(&points[6]) {
            let n0 = (points[6] - points[4])
                .cross(&(points[5] - points[4]))
                .normalize();
            let n1 = (points[7] - points[4])
                .cross(&(points[6] - points[4]))
                .normalize();
            let normal = Normal {
                0: (n0 + n1).normalize(),
            };

            let vertex_offset = mesh_builder.add_vertices_from_slice(
                &[
                    Point3D(points[4]),
                    Point3D(points[5]),
                    Point3D(points[6]),
                    Point3D(points[7]),
                ],
                &[normal; 4],
            );

            mesh_builder.add_triangle(&[vertex_offset, vertex_offset + 2, vertex_offset + 1]);
            mesh_builder.add_triangle(&[vertex_offset, vertex_offset + 3, vertex_offset + 2]);
        }

        // transform the vertices
        mesh_builder.transform_vertices(transform, translation);

        // create the mesh
        let mesh = mesh_builder.into_mesh();
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

    /// Tests a simple pyramid, that is actually a box and checks if the normals and faces are
    /// correct.
    #[test]
    fn test_tessellate_pyramid1() {
        // create a pyramid that is actually a box
        let box_data = PyramidData {
            inner: [1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0],
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

        // iterate over the triangles and make sure the normals are pointing outwards and the
        // area is correct
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

            assert!(
                (1f32 - n0.dot(&normal)).abs() < 1e-6f32,
                "Normal is not consistent. Is={:?}, Should={:?}",
                n0,
                normal
            );
            assert!(
                (1f32 - n1.dot(&normal)).abs() < 1e-6f32,
                "Normal is not consistent. Is={:?}, Should={:?}",
                n1,
                normal
            );
            assert!(
                (1f32 - n2.dot(&normal)).abs() < 1e-6f32,
                "Normal is not consistent. Is={:?}, Should={:?}",
                n2,
                normal
            );

            // make sure the normal is pointing outwards
            assert!(
                c.dot(&normal) > 0f32,
                "Normal is not pointing outwards. Reference={:?}, Normal={:?}",
                c,
                normal
            );
        }

        assert_eq!(total_area, 6f32);
    }

    /// Tests a pyramid where the top is sharp.
    #[test]
    fn test_tessellate_pyramid2() {
        // create a pyramid that is actually a box
        let box_data = PyramidData {
            inner: [1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        };
        let options = TessellationOptions::default();
        let transform = Mat3::identity();
        let translation = Vec3::zeros();
        let mesh = box_data
            .tessellate(&options, &transform, &translation)
            .unwrap();

        assert_eq!(mesh.get_vertices().get_positions().len(), 16);
        assert_eq!(mesh.get_vertices().get_normals().unwrap().len(), 16);
        assert_eq!(mesh.get_primitives().get_raw_index_data().num_indices(), 18);

        let primitives = mesh.get_primitives();
        assert_eq!(primitives.get_primitive_type(), PrimitiveType::Triangles);

        let indices = primitives.get_raw_index_data().get_indices_ref().unwrap();

        // iterate over the triangles and make sure the normals are pointing outwards and the
        // area is correct
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

            assert!(
                (1f32 - n0.dot(&normal)).abs() < 1e-6f32,
                "Normal is not consistent. Is={:?}, Should={:?}",
                n0,
                normal
            );
            assert!(
                (1f32 - n1.dot(&normal)).abs() < 1e-6f32,
                "Normal is not consistent. Is={:?}, Should={:?}",
                n1,
                normal
            );
            assert!(
                (1f32 - n2.dot(&normal)).abs() < 1e-6f32,
                "Normal is not consistent. Is={:?}, Should={:?}",
                n2,
                normal
            );

            // make sure the normal is pointing outwards
            assert!(
                c.dot(&normal) > 0f32,
                "Normal is not pointing outwards. Reference={:?}, Normal={:?}",
                c,
                normal
            );
        }

        let h = 1.25f32.sqrt();
        assert_eq!(total_area, 2f32 * h + 1f32);
    }
}
