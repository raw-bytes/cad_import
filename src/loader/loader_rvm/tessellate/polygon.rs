use log::error;
use lyon_tessellation;
use lyon_tessellation::geometry_builder::*;
use lyon_tessellation::path::math::point;
use lyon_tessellation::path::traits::PathBuilder;
use lyon_tessellation::path::Path;
use lyon_tessellation::{FillOptions, FillTessellator};
use nalgebra_glm::{Mat3, Vec3};

use crate::{
    loader::{
        loader_rvm::primitive::{Polygon, PolygonsData, Vertex},
        TessellationOptions,
    },
    structure::{IndexData, Mesh, Normal, Normals, Point3D, Positions, Primitives, Vertices},
};

/// The polygons tessellation operator is used to tessellate a list of polygons defined by inner
/// and outer contours based on the specified tessellation options.
pub struct PolygonsTessellationOperator<'a> {
    polygon_data: &'a PolygonsData,

    polygon_normal: Vec3,

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
    pub fn new(polygon_data: &'a PolygonsData, _: &TessellationOptions) -> Self {
        Self {
            polygon_data,
            polygon_normal: Vec3::zeros(),
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

        // Tessellate all the polygons.
        for polygon in self.polygon_data.inner.iter() {
            self.tessellate_polygon(polygon);
        }

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

    /// Tessellates the given polygon and stores the tessellated data in the operator.
    ///
    /// # Arguments
    /// * `polygon` - The polygon to tessellate.
    fn tessellate_polygon(&mut self, polygon: &Polygon) {
        // first determine the normal of the polygon
        if let Some(normal) = Self::determine_polygon_normal(polygon) {
            self.polygon_normal = normal;
        } else {
            // The normal could not be determined, so we use the z-axis as the normal.
            self.polygon_normal = Vec3::z();
        }

        // determine the orthogonal coordinate system for the normal
        let (u, v) = Self::create_orthogonal_coordinate_system_for_normal(&self.polygon_normal);

        // create the transformation matrix for the polygon to project it onto the xy-plane
        let plane_to_space = Mat3::from_columns(&[u, v, self.polygon_normal]);
        let space_to_plane = plane_to_space.transpose();

        // project polygon vertices into the xy-plane and build the paths for the lyon tessellator
        let mut min_z_value = f32::MAX;
        let mut max_z_value = f32::MIN;
        let mut path_builder = Path::builder_with_attributes(3);
        for contour in polygon.contours.iter().filter(|c| c.inner.len() > 2) {
            let in_vertices = contour.inner.as_slice();

            // create the first point of the new sub-path
            let p = Self::transform_vertex_position(&space_to_plane, &in_vertices[0]);
            min_z_value = min_z_value.min(p[2]);
            max_z_value = max_z_value.max(p[2]);
            path_builder.begin(point(p[0], p[1]), in_vertices[0].normal().as_slice());

            // add the remaining points for the current sub-path
            for v in &in_vertices[1..] {
                let p = Self::transform_vertex_position(&space_to_plane, v);
                min_z_value = min_z_value.min(p[2]);
                max_z_value = max_z_value.max(p[2]);
                path_builder.line_to(point(p[0], p[1]), v.normal().as_slice());
            }

            path_builder.close();
        }

        let path = path_builder.build();

        // stop if there are no paths to tessellate, i.e., the polygon is degenerate
        if max_z_value < min_z_value {
            return;
        }

        let z_coord = (min_z_value + max_z_value) / 2f32;

        let mut buffers: VertexBuffers<(Point3D, Normal), u32> = VertexBuffers::new();
        {
            let mut vertex_builder =
                BuffersBuilder::new(&mut buffers, VertexConstructor { z_coord });

            let mut tessellator = FillTessellator::new();
            if let Err(err) = tessellator.tessellate_with_ids(
                path.id_iter(),
                &path,
                Some(&path),
                &FillOptions::default(),
                &mut vertex_builder,
            ) {
                error!("Failed to tessellate polygon: {}", err);
            } else {
                let index_offset = self.positions.len() as u32;
                self.positions.extend(
                    buffers
                        .vertices
                        .iter()
                        .map(|v| Point3D(plane_to_space * v.0 .0)),
                );
                self.normals.extend(buffers.vertices.iter().map(|v| v.1));

                self.indices
                    .extend(buffers.indices.iter().map(|i| *i + index_offset));
            }
        }
    }

    /// Tries to determine the normal of the plane in which the polygon lies.
    /// The idea for the implementation of this function is taken from the following source:
    /// https://gitlab.freedesktop.org/mesa/glu/-/blob/master/src/libtess/alg-outline
    /// from the mesa implementation of GLU.
    /// Returns the normal or None if the normal could not be determined.
    ///
    /// # Arguments
    /// * `polygon` - The polygon for which the normal should be determined.
    fn determine_polygon_normal(polygon: &Polygon) -> Option<Vec3> {
        // Compute the bounding volume of the polygon and store the minimum and maximum vertices
        // for each axis.
        let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);
        let mut min_verts: [Vec3; 3] = Default::default();
        let mut max_verts: [Vec3; 3] = Default::default();
        for contour in polygon.contours.iter() {
            for v in contour.inner.iter() {
                for i in 0usize..3usize {
                    let c = v.position()[i];
                    if c < min[i] {
                        min[i] = c;
                        min_verts[i] = Vec3::from_column_slice(v.position().as_slice());
                    }

                    if c > max[i] {
                        max[i] = c;
                        max_verts[i] = Vec3::from_column_slice(v.position().as_slice());
                    }
                }
            }
        }

        // Find two vertices separated by at least 1/sqrt(3) of the maximum distance between any
        // two vertices. That is, determine the longest axis of the bounding box. We'll use this
        // axis later, by taking a vertex at the minimum and maximum of that axis.
        let mut i = 0usize;
        if max[1] - min[1] > max[0] - min[0] {
            i = 1;
        }

        if max[2] - min[2] > max[i] - min[i] {
            i = 2;
        }

        // All vertices are the same -- normal doesn't matter
        if min[i] >= max[i] {
            return None;
        }

        // Look for a third vertex which forms the triangle with maximum area
        // Note: (Length of normal == twice the triangle area)
        let mut max_len2 = 0f32;
        let v1 = min_verts[i];
        let v2 = max_verts[i];
        let d1 = v1 - v2;
        let mut norm = Vec3::zeros();
        for contour in polygon.contours.iter() {
            for v in contour.inner.iter() {
                let d2 = Vec3::from_column_slice(v.position().as_slice()) - v2;
                let t_norm = d1.cross(&d2);
                let t_len2 = t_norm.norm_squared();
                if t_len2 > max_len2 {
                    max_len2 = t_len2;
                    norm = t_norm;
                }
            }
        }

        // check if the normal is degenerate
        if max_len2 <= 0f32 {
            return None;
        }

        Some(norm / max_len2.sqrt())
    }

    /// Determines the other two orthogonal axes for the given normal.
    ///
    /// # Arguments
    /// * `normal` - The normal for which the orthogonal axes should be determined.
    fn create_orthogonal_coordinate_system_for_normal(normal: &Vec3) -> (Vec3, Vec3) {
        // determine the axis with the smallest absolute value
        let mut axis = 0usize;
        if normal[1].abs() < normal[axis].abs() {
            axis = 1;
        }
        if normal[2].abs() < normal[axis].abs() {
            axis = 2;
        }

        // compute cross product with the smallest axis to get the first orthogonal axis
        let mut u = Vec3::zeros();
        u[axis] = 1f32;
        let u = u.cross(normal).normalize();

        // compute the second orthogonal axis and normalize it again for numerical stability
        let v = normal.cross(&u).normalize();

        (u, v)
    }

    /// Transforms the given vertex position using the given transformation matrix.
    ///
    /// # Arguments
    /// * `transform` - The transformation matrix to apply to the vertex position.
    /// * `vertex` - The vertex to transform.
    #[inline]
    fn transform_vertex_position(transform: &Mat3, vertex: &Vertex) -> Vec3 {
        let pos = Vec3::from_column_slice(vertex.position().as_slice());
        transform * pos
    }
}

/// The vertex constructor that turns lyon tessellator vertices into RVM contour vertices.
/// See the geometry_builder module for more details.
struct VertexConstructor {
    /// The z-coordinate for the vertices.
    pub z_coord: f32,
}

impl FillVertexConstructor<(Point3D, Normal)> for VertexConstructor {
    fn new_vertex(&mut self, mut vertex: lyon_tessellation::FillVertex) -> (Point3D, Normal) {
        let position = vertex.position();
        let attrs = vertex.interpolated_attributes();
        let normal = Vec3::from_column_slice(attrs).normalize();

        (
            Point3D::new(position.x, position.y, self.z_coord),
            Normal::new(normal.x, normal.y, normal.z),
        )
    }
}

#[cfg(test)]
mod test {
    use itertools::{Itertools, MinMaxResult};
    use nalgebra_glm::{vec4_to_vec3, Mat4, Vec4};

    use crate::loader::loader_rvm::primitive::{Contour, Vertex};

    use super::*;

    #[test]
    fn test_determine_polygon_normal() {
        // Test: Circle in the xy-plane
        let n = 1024;
        let verts: Vec<Vertex> = (0..n)
            .map(|i| {
                let angle = 2f32 * std::f32::consts::PI * i as f32 / n as f32;
                let x = angle.cos() * 10f32;
                let y = angle.sin() * 10f32;
                let z = 0f32;
                Vertex {
                    inner: [x, y, z, 0f32, 0f32, 1f32],
                }
            })
            .collect();

        let p = Polygon {
            contours: vec![Contour {
                inner: verts.clone(),
            }],
        };

        let normal = PolygonsTessellationOperator::determine_polygon_normal(&p).unwrap();
        assert_eq!(normal.abs().as_slice(), &[0f32, 0f32, 1f32]);

        // Test: Rotated circle
        let mut m = Mat4::new_rotation(Vec3::new(1f32, 1.5f32, 0f32));
        m.set_column(3, &Vec4::new(10f32, 5f32, 3f32, 1f32));

        let p2 = Polygon {
            contours: vec![Contour {
                inner: verts
                    .iter()
                    .map(|v| {
                        // transform position using homogeneous coordinates
                        let p = Vec4::new(v.inner[0], v.inner[1], v.inner[2], 1f32);
                        let p = m * p;
                        let pos = vec4_to_vec3(&p);
                        Vertex {
                            inner: [pos.x, pos.y, pos.z, 0f32, 0f32, 0f32],
                        }
                    })
                    .collect(),
            }],
        };

        let normal = -PolygonsTessellationOperator::determine_polygon_normal(&p2).unwrap();

        // compute the min/max of all vertices by projecting them onto the plane defined by the normal
        let range = p2.contours[0]
            .inner
            .iter()
            .map(|v| {
                let pos = Vec3::from_column_slice(v.position().as_slice());
                normal.dot(&pos)
            })
            .minmax();

        let diff = if let MinMaxResult::MinMax(a, b) = range {
            println!("Range of projected distances: {}-{}", a, b);
            b - a
        } else {
            panic!("Could not determine min/max of vertices");
        };

        println!("Normal: {:?}", normal);
        println!("Projected differences error: {:?}", diff);

        assert!(diff < 1e-5f32, "Vertices are not all located in a plane!!!");
    }

    /// Checks if the given basis vectors form an orthogonal coordinate system.
    fn check_if_system_is_orthogonal(b0: &Vec3, b1: &Vec3, b2: &Vec3) {
        assert!(
            (b0.norm() - 1f32).abs() < 1e-6f32,
            "Basis vector 0 is not normalized"
        );
        assert!(
            (b1.norm() - 1f32).abs() < 1e-6f32,
            "Basis vector 1 is not normalized"
        );
        assert!(
            (b2.norm() - 1f32).abs() < 1e-6f32,
            "Basis vector 2 is not normalized"
        );

        assert!(b0.dot(b1) < 1e-5f32);
        assert!(b0.dot(b2) < 1e-5f32);
        assert!(b1.dot(b2) < 1e-5f32);
    }

    #[test]
    fn test_create_orthogonal_coordinate_system_for_normal() {
        let normal = Vec3::new(1f32, 0f32, 0f32);
        let (u, v) =
            PolygonsTessellationOperator::create_orthogonal_coordinate_system_for_normal(&normal);
        check_if_system_is_orthogonal(&u, &v, &normal);

        let normal = Vec3::new(0f32, 1f32, 0f32);
        let (u, v) =
            PolygonsTessellationOperator::create_orthogonal_coordinate_system_for_normal(&normal);
        check_if_system_is_orthogonal(&u, &v, &normal);

        let normal = Vec3::new(0f32, 0f32, 1f32);
        let (u, v) =
            PolygonsTessellationOperator::create_orthogonal_coordinate_system_for_normal(&normal);
        check_if_system_is_orthogonal(&u, &v, &normal);

        let normal = Vec3::new(1f32, 1f32, 1f32).normalize();
        let (u, v) =
            PolygonsTessellationOperator::create_orthogonal_coordinate_system_for_normal(&normal);
        check_if_system_is_orthogonal(&u, &v, &normal);
    }

    #[test]
    fn test_polygon_tessellation() {
        // Test: Rotated circle with a hole in the middle
        let mut m = Mat4::new_rotation(Vec3::new(1f32, 1.5f32, 0f32));
        m.set_column(3, &Vec4::new(10f32, 5f32, 3f32, 1f32));

        let n = 1024;
        let verts: Vec<Vertex> = (0..n)
            .map(|i| {
                let angle = 2f32 * std::f32::consts::PI * i as f32 / n as f32;
                let x = angle.cos() * 10f32;
                let y = angle.sin() * 10f32;
                let z = 0f32;
                Vertex {
                    inner: [x, y, z, 0f32, 0f32, 1f32],
                }
            })
            .collect();

        let p = Polygon {
            contours: vec![
                Contour {
                    inner: verts.clone(),
                },
                Contour {
                    inner: verts
                        .iter()
                        .cloned()
                        .map(|v| Vertex {
                            inner: [
                                v.inner[0] * 0.1f32,
                                v.inner[1] * 0.1f32,
                                v.inner[2],
                                0f32,
                                0f32,
                                1f32,
                            ],
                        })
                        .collect(),
                },
            ],
        };

        let p2 = Polygon {
            contours: p
                .contours
                .iter()
                .map(|c| Contour {
                    inner: c
                        .inner
                        .iter()
                        .map(|v| {
                            // transform position using homogeneous coordinates
                            let p = Vec4::new(v.inner[0], v.inner[1], v.inner[2], 1f32);
                            let p = m * p;
                            let pos = vec4_to_vec3(&p);
                            Vertex {
                                inner: [pos.x, pos.y, pos.z, 0f32, 0f32, 0f32],
                            }
                        })
                        .collect(),
                })
                .collect(),
        };

        let polygons_data = PolygonsData { inner: vec![p2] };
        let mut op =
            PolygonsTessellationOperator::new(&polygons_data, &TessellationOptions::default());
        op.tessellate(&Mat3::identity(), &Vec3::zeros());

        // DEBUG Code: Deactivated by default
        // use std::io::{BufWriter, Write};
        // let mesh = op.into_mesh();
        // // write resulting mesh to disk as OFF
        // let file = std::fs::File::create("rotated_circle.off").unwrap();
        // let mut file = BufWriter::new(file);
        // writeln!(file, "OFF").unwrap();
        // writeln!(
        //     file,
        //     "{} {} 0",
        //     mesh.get_vertices().len(),
        //     mesh.get_primitives().num_primitives()
        // )
        // .unwrap();

        // for v in mesh.get_vertices().get_positions().iter() {
        //     writeln!(file, "{} {} {}", v.0.x, v.0.y, v.0.z).unwrap();
        // }

        // for triangle in mesh
        //     .get_primitives()
        //     .get_raw_index_data()
        //     .get_indices_ref()
        //     .unwrap()
        //     .chunks_exact(3)
        // {
        //     writeln!(file, "3 {} {} {}", triangle[0], triangle[1], triangle[2]).unwrap();
        // }
    }
}
