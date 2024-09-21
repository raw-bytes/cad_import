use nalgebra_glm::{Mat3, Vec3};

use crate::{
    loader::TessellationOptions,
    structure::{IndexData, Mesh, Point3D, Primitives, Vertices},
    Error, Length,
};

use super::primitive::{BoxData, CylinderData};

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
        let mut positions = vec![
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

        // Apply the transformation and translation to the positions.
        positions.iter_mut().for_each(|p| {
            p.0 = transform * p.0 + translation;
        });

        let mut normals = vec![
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

        // Transform the normals using the normal transformation matrix.
        let normal_mat = transform.transpose().try_inverse().unwrap();
        normals.iter_mut().for_each(|n| {
            n.0 = (normal_mat * n.0).normalize();
        });

        let index_data = IndexData::Indices(indices);
        let mut vertices = Vertices::from_positions(positions);
        vertices.set_normals(normals).unwrap();
        let primitives =
            Primitives::new(index_data, crate::structure::PrimitiveType::Triangles).unwrap();
        let mesh = Mesh::new(vertices, primitives).unwrap();

        Ok(mesh)
    }
}

impl Tessellate for CylinderData {
    fn tessellate(
        &self,
        tessellation_options: &TessellationOptions,
        transform: &Mat3,
        translation: &Vec3,
    ) -> Result<Mesh, Error> {
        unimplemented!()
    }
}

/// Determines the required number of segments for the specified circle based on the tessellation
/// options.
///
/// # Arguments
/// * `r` - The radius of the circle.
/// * `tessellation_options` - The tessellation options to use.
fn determine_num_segments_for_circle(
    r: Length,
    tessellation_options: &TessellationOptions,
) -> usize {
    let radius_mm = r.get_unit_in_meters() * 1e3f64;

    assert!(radius_mm > 0.0, "The radius must be positive.");
    let mut num_segments = 4;

    // determine the minimal required number of segments to satisfy the sag error condition
    let sag_mm = tessellation_options.max_sag.get_unit_in_meters() * 1e3f64;
    // If the sag is greater or equal to the radius, it cannot have any impact. That is, the
    // circle will always satisfy the sag error condition.
    // If the sag is less or equal to zero, no tessellated circle can satisfy the constraint.
    if sag_mm > 0.0 && sag_mm < radius_mm {
        // For a given radius r and number of segments n, the sag is given by:
        // sag = r * (1 - cos(pi / n))
        // To determine the number of segments n for a given sag, we can solve the above equation for n:
        // n = pi / acos(1 - sag / r)

        let n = (std::f64::consts::PI / (1.0 - (sag_mm / radius_mm)).acos()).ceil() as usize;
        num_segments = num_segments.max(n);
    }

    // If the maximum length is defined, we need to determine the number of segments based on the
    // length.
    if let Some(max_length) = tessellation_options.max_length {
        // For a given radius r and number of segments n, the chord length of a segment is given by:
        // length = sin(pi / n) * 2 * r
        // To determine the number of segments n for a given length, we can solve the above equation for n:
        // n = pi / asin(length / (2 * r))

        let max_length_mm = max_length.get_unit_in_meters() * 1e3f64;

        if max_length_mm > 0.0 {
            let n = (std::f64::consts::PI / (max_length_mm / (2f64 * radius_mm)).asin()).ceil()
                as usize;
            num_segments = num_segments.max(n);
        }
    }

    // If the maximum angle is defined, we need to determine the number of segments based on the
    // angle.
    if let Some(max_angle) = tessellation_options.max_angle {
        let max_angle_rad = max_angle.get_unit_in_radians();

        if max_angle_rad > 0.0 {
            // The maximum angle between two adjacent segments is given by:
            // angle = 2 * pi / n
            // To determine the number of segments n for a given angle, we can solve the above equation for n:
            // n = 2 * pi / angle

            let n = (2f64 * std::f64::consts::PI / max_angle_rad).ceil() as usize;
            num_segments = num_segments.max(n);
        }
    }

    num_segments
}

#[cfg(test)]
mod test {
    use nalgebra_glm::DVec2 as Vec2;

    use crate::{structure::PrimitiveType, Angle, Length};

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

            assert_eq!(normal, n0);
            assert_eq!(normal, n1);
            assert_eq!(normal, n2);
        }

        assert_eq!(total_area, 24f32);
    }

    #[test]
    fn test_determine_num_segments_for_circle() {
        let radius = [
            Length::new(1.0),
            Length::new(2.0),
            Length::new(3.0),
            Length::new(4.0),
            Length::new(5.0),
        ];

        for r in radius {
            let max_angle_error = [
                None,
                Some(Angle::new(0.7)),
                Some(Angle::new(0.2)),
                Some(Angle::new(0.1)),
            ];

            let max_length_error = [
                None,
                Some(Length::new(0.1)),
                Some(Length::new(0.05)),
                Some(Length::new(0.01)),
                Some(Length::new(0.001)),
            ];

            let max_sag_error = [
                Length::new(0.1),
                Length::new(0.05),
                Length::new(0.01),
                Length::new(0.001),
            ];

            for max_angle in &max_angle_error {
                for max_length in &max_length_error {
                    for max_sag in &max_sag_error {
                        let options = TessellationOptions {
                            max_sag: *max_sag,
                            max_length: *max_length,
                            max_angle: *max_angle,
                        };

                        let num_segments = determine_num_segments_for_circle(r, &options);
                        assert!(num_segments > 0);

                        // compute three consecutive points on the circle
                        let a0 = 0f64;
                        let a1 = 2f64 * std::f64::consts::PI / num_segments as f64;
                        let a2 = 2f64 * a1;
                        let points: [Vec2; 3] = [
                            Vec2::new(
                                r.get_unit_in_meters() * a0.cos(),
                                r.get_unit_in_meters() * a0.sin(),
                            ),
                            Vec2::new(
                                r.get_unit_in_meters() * a1.cos(),
                                r.get_unit_in_meters() * a1.sin(),
                            ),
                            Vec2::new(
                                r.get_unit_in_meters() * a2.cos(),
                                r.get_unit_in_meters() * a2.sin(),
                            ),
                        ];

                        // compute the sag error
                        let mut sag_error_m = 0f64;
                        for f in [
                            0f64, 0.1f64, 0.2f64, 0.3f64, 0.4f64, 0.5f64, 0.6f64, 0.7f64, 0.8f64,
                            0.9f64, 1.0f64,
                        ]
                        .iter()
                        {
                            let p = points[0] * (1f64 - *f) + points[1] * *f;
                            let err = (p.norm() - r.get_unit_in_meters()).abs();

                            sag_error_m = sag_error_m.max(err);
                        }

                        assert!(
                            sag_error_m <= max_sag.get_unit_in_meters(),
                            "Sag error is too large. Expected: {}, Actual: {}",
                            max_sag.get_unit_in_meters(),
                            sag_error_m
                        );

                        // compute length error
                        if let Some(max_length) = max_length {
                            let length_error_m = (points[1] - points[0]).norm();
                            assert!(
                                length_error_m <= max_length.get_unit_in_meters(),
                                "Length error is too large. Expected: {}, Actual: {}",
                                max_length.get_unit_in_meters(),
                                length_error_m
                            );
                        }

                        // compute angle error
                        if let Some(max_angle) = max_angle {
                            // compute normal of each segment respectively
                            let a = points[1] - points[0];
                            let b = points[2] - points[1];
                            let n0 = Vec2::new(-a.y, a.x).normalize();
                            let n1 = Vec2::new(-b.y, b.x).normalize();

                            let angle_error_rad = n0.dot(&n1).acos();
                            assert!(
                                angle_error_rad <= max_angle.get_unit_in_radians(),
                                "Angle error is too large. Expected: {}, Actual: {}",
                                max_angle.get_unit_in_radians(),
                                angle_error_rad
                            );
                        }
                    }
                }
            }
        }
    }
}
