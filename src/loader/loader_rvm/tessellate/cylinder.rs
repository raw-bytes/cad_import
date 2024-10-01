use nalgebra_glm::{Mat3, Vec2, Vec3};

use crate::{
    loader::{loader_rvm::primitive::CylinderData, TessellationOptions},
    structure::{IndexData, Mesh, Normal, Normals, Point3D, Positions, Primitives, Vertices},
    Length,
};

/// The cylinder tessellation operator is used to tessellate a cylinder based on the specified
/// cylinder data and tessellation options.
pub struct CylinderTessellationOperator {
    height_mm: f32,
    radius_mm: f32,

    tessellation_parameter: CylinderTessellationParameter,
    positions: Positions,
    normals: Normals,
    indices: Vec<u32>,

    unit_circle: Vec<Vec2>,
}

impl CylinderTessellationOperator {
    /// Creates a new cylinder tessellation operator. That is, an operator that tessellates a
    /// cylinder based on the specified cylinder data and tessellation options.
    ///
    /// # Arguments
    /// * `cylinder_data` - The cylinder data to use for the tessellation.
    pub fn new(cylinder_data: &CylinderData, tessellation_options: &TessellationOptions) -> Self {
        let height_mm = cylinder_data.height();
        let radius_mm = cylinder_data.radius();

        let t = Self::determine_cylinder_tessellation_parameter(
            Length::new(radius_mm as f64 * 1e-3f64),
            Length::new(height_mm as f64 * 1e-3f64),
            tessellation_options,
        );

        // determine the overall number of vertices
        let num_vertices_cap = (t.num_radial_circles - 1) * t.num_segments_per_circle + 1;
        let num_vertices_side = t.num_height_segments * t.num_segments_per_circle;
        let num_vertices = 2 * num_vertices_cap + num_vertices_side;

        // determine the number of indices
        let num_indices_cap = (t.num_radial_circles - 1) * t.num_segments_per_circle * 6
            + t.num_segments_per_circle * 3;
        let num_indices_side = (t.num_height_segments - 1) * t.num_segments_per_circle * 6;
        let num_indices = 2 * num_indices_cap + num_indices_side;

        let unit_circle = Self::tessellate_unit_sphere_2d(t.num_segments_per_circle);

        Self {
            height_mm,
            radius_mm,
            tessellation_parameter: t,
            positions: Vec::with_capacity(num_vertices),
            normals: Vec::with_capacity(num_vertices),
            indices: Vec::with_capacity(num_indices),
            unit_circle,
        }
    }

    /// Tessellates the cylinder based on the specified transformation and translation.
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

        self.tessellate_cylinder_cap(CapLocation::Top);
        self.tessellate_cylinder_side();
        self.tessellate_cylinder_cap(CapLocation::Bottom);

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

    /// Converts the tessellated cylinder into a mesh object.
    pub fn into_mesh(self) -> Mesh {
        let index_data = IndexData::Indices(self.indices);
        let mut vertices = Vertices::from_positions(self.positions);
        vertices.set_normals(self.normals).unwrap();
        let primitives =
            Primitives::new(index_data, crate::structure::PrimitiveType::Triangles).unwrap();
        Mesh::new(vertices, primitives).expect("Failed to create mesh")
    }

    /// Tessellates one of the caps of the cylinder, i.e. the top or the bottom cap.
    ///
    /// # Arguments
    /// * `cap_location` - The location of the cap to tessellate.
    fn tessellate_cylinder_cap(&mut self, cap_location: CapLocation) {
        let positions = &mut self.positions;
        let normals = &mut self.normals;
        let indices = &mut self.indices;

        let t = &self.tessellation_parameter;
        let height_mm = self.height_mm;
        let radius_mm = self.radius_mm;
        let unit_circle = &self.unit_circle;

        let num_segments = t.num_segments_per_circle as u32;

        // Determine the direction of the cap based on the location.
        let (dir, d) = match cap_location {
            CapLocation::Top => (1f32, 0),
            CapLocation::Bottom => (-1f32, 1),
        };

        let z = height_mm / 2f32 * dir;
        let vertex_offset = positions.len() as u32;

        // add the center vertex of the cap
        positions.push(Point3D::new(0f32, 0f32, z));

        // create the different circles of the cap
        for circle_index in 1..t.num_radial_circles {
            // determine the radius of the current circle
            let cur_radius = radius_mm * (circle_index + 1) as f32 / t.num_radial_circles as f32;

            // determine the offset of the current circle in the positions array
            let circle_vertex_offset = positions.len() as u32;

            // Add the unit circle vertices to the positions with the current radius, z-coordinate
            // and orientation. Depending on the direction, the orientation is either clockwise or
            // counter-clockwise.
            positions.extend(
                unit_circle
                    .iter()
                    .map(|p| Point3D::new(p.x * cur_radius, p.y * cur_radius, z)),
            );

            // Check if the current circle is the inner circle, consisting only of the center
            // vertex and a circle or if it is an segment being consisting of two circles.
            if circle_index == 1 {
                for i in 0..num_segments {
                    let i0 = vertex_offset; // center vertex
                    let i1 = vertex_offset + 1 + (i + d) % num_segments;
                    let i2 = vertex_offset + 1 + (i + (1 + d) % 2) % num_segments;

                    indices.extend([i0, i1, i2]);
                }
            } else {
                for i in 0..(t.num_segments_per_circle as u32) {
                    let i2 = circle_vertex_offset + (i + d) % num_segments;
                    let i3 = circle_vertex_offset + (i + (1 + d) % 2) % num_segments;

                    let i0 = i2 - num_segments;
                    let i1 = i3 - num_segments;

                    indices.extend([i1, i0, i2]);
                    indices.extend([i1, i2, i3]);
                }
            }
        }

        // Fill up the normals with the normal of the cap that is pointing either in negative or
        // positive z-direction.
        normals.extend(
            std::iter::repeat(Normal::new(0f32, 0f32, dir)).take(positions.len() - normals.len()),
        );
    }

    /// Tessellates the side of the cylinder.
    fn tessellate_cylinder_side(&mut self) {
        let positions = &mut self.positions;
        let normals = &mut self.normals;
        let indices = &mut self.indices;

        let t = &self.tessellation_parameter;
        let height_mm = self.height_mm;
        let half_height_mm = height_mm / 2f32;
        let radius_mm = self.radius_mm;
        let unit_circle = &self.unit_circle;

        let num_segments = t.num_segments_per_circle as u32;
        let num_height_segments = t.num_height_segments as u32;

        for height_segment_index in 0..(num_height_segments + 1) {
            // determine the height of the current segment
            let z = height_mm * height_segment_index as f32 / num_height_segments as f32
                - half_height_mm;

            let vertex_offset = positions.len() as u32;

            // Add the unit circle vertices to the positions with the current radius, z-coordinate
            // and orientation. Depending on the direction, the orientation is either clockwise or
            // counter-clockwise.
            positions.extend(
                unit_circle
                    .iter()
                    .map(|p| Point3D::new(p.x * radius_mm, p.y * radius_mm, z)),
            );

            // Add the unit circle vertices as normals.
            normals.extend(
                unit_circle
                    .iter()
                    .map(|p| Normal::new(p.x, p.y, 0f32))
                    .collect::<Vec<Normal>>(),
            );

            // Add the indices for the triangles of the current segment if it is not the last
            // segment.
            if height_segment_index < num_height_segments {
                for i in 0..num_segments {
                    let i1 = vertex_offset + i;
                    let i0 = vertex_offset + (i + 1) % num_segments;
                    let i2 = i0 + num_segments;
                    let i3 = i1 + num_segments;

                    indices.extend([i1, i0, i2]);
                    indices.extend([i1, i2, i3]);
                }
            }
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

    /// Determines the tessellation parameter for the cylinder based on the tessellation options and
    /// the dimensions of the cylinder.
    ///
    /// # Arguments
    /// * `r` - The radius of the cylinder.
    /// * `h` - The height of the cylinder.
    /// * `tessellation_options` - The tessellation options to use.
    fn determine_cylinder_tessellation_parameter(
        r: Length,
        h: Length,
        tessellation_options: &TessellationOptions,
    ) -> CylinderTessellationParameter {
        let max_length_mm = tessellation_options
            .max_length
            .map(|l| l.get_unit_in_meters() * 1e3f64);

        let num_segments_per_circle =
            Self::determine_num_segments_for_circle(r, tessellation_options);

        // Determine the number of height segments based on the maximum length.
        let num_height_segments = if let Some(max_length_mm) = max_length_mm {
            if max_length_mm > 0f64 {
                let height_mm = h.get_unit_in_meters() * 1e3f64;

                2.max((height_mm / max_length_mm).ceil() as usize)
            } else {
                2
            }
        } else {
            2
        };

        // Determine the number of radial segments based on the maximum length.
        let num_radial_circles = if let Some(max_length_mm) = max_length_mm {
            if max_length_mm > 0f64 {
                let radius_mm = r.get_unit_in_meters() * 1e3f64;

                2.max((radius_mm / max_length_mm).ceil() as usize)
            } else {
                2
            }
        } else {
            2
        };

        CylinderTessellationParameter {
            num_radial_circles,
            num_height_segments,
            num_segments_per_circle,
        }
    }

    /// Tessellates a unit circle in 2D in the x-y plane in counter-clockwise order with the specified
    /// number of segments.
    ///
    /// # Arguments
    /// * `num_segments` - The number of segments to use.
    fn tessellate_unit_sphere_2d(num_segments: usize) -> Vec<Vec2> {
        (0..num_segments)
            .map(|i| {
                let angle = 2f32 * std::f32::consts::PI * i as f32 / num_segments as f32;
                Vec2::new(angle.cos(), angle.sin())
            })
            .collect()
    }
}

/// The tessellation parameter for the cylinder.
#[derive(Clone, Debug)]
struct CylinderTessellationParameter {
    /// The number of radial segments, i.e., the number of circle at the bottom and top of the
    /// cylinder around the center.
    /// 2 is the minimum number of radial segments and means that the cylinder has a center and
    /// one outer circle.
    pub num_radial_circles: usize,

    /// The number of height segments, i.e., the number of segments along the height of the cylinder.
    /// 2 is the minimum number of height segments and means that the cylinder has a top and a bottom.
    pub num_height_segments: usize,

    /// The number of segments per circle.
    pub num_segments_per_circle: usize,
}

/// The location of the cap of the cylinder.
#[derive(Clone, Copy, Debug)]
enum CapLocation {
    Top,
    Bottom,
}

#[cfg(test)]
mod test {
    use nalgebra_glm::DVec2 as Vec2;

    use crate::Angle;

    use super::*;

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

                        let num_segments =
                            CylinderTessellationOperator::determine_num_segments_for_circle(
                                r, &options,
                            );
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

    #[test]
    fn test_determine_cylinder_tessellation_parameter() {
        // test number of height segments
        let r = CylinderTessellationOperator::determine_cylinder_tessellation_parameter(
            Length::new(1.0),
            Length::new(2.0),
            &TessellationOptions {
                max_length: Some(Length::new(0.5)),
                ..TessellationOptions::default()
            },
        );
        assert_eq!(r.num_height_segments, 4);

        let r = CylinderTessellationOperator::determine_cylinder_tessellation_parameter(
            Length::new(1.0),
            Length::new(3.0),
            &TessellationOptions {
                max_length: Some(Length::new(0.1)),
                ..TessellationOptions::default()
            },
        );
        assert_eq!(r.num_height_segments, 30);

        let r = CylinderTessellationOperator::determine_cylinder_tessellation_parameter(
            Length::new(1.0),
            Length::new(3.0),
            &TessellationOptions {
                max_length: Some(Length::new(0.0)),
                ..TessellationOptions::default()
            },
        );
        assert_eq!(r.num_height_segments, 2);

        // test number of radial segments
        let r = CylinderTessellationOperator::determine_cylinder_tessellation_parameter(
            Length::new(1.0),
            Length::new(2.0),
            &TessellationOptions {
                max_length: Some(Length::new(0.5)),
                ..TessellationOptions::default()
            },
        );
        assert_eq!(r.num_radial_circles, 2);

        let r = CylinderTessellationOperator::determine_cylinder_tessellation_parameter(
            Length::new(1.0),
            Length::new(3.0),
            &TessellationOptions {
                max_length: Some(Length::new(0.1)),
                ..TessellationOptions::default()
            },
        );
        assert_eq!(r.num_radial_circles, 10);
    }

    #[test]
    fn test_cylinder_tessellation() {
        let mut op = CylinderTessellationOperator::new(
            &CylinderData {
                inner: [4000.0, 7000.0],
            },
            &TessellationOptions {
                max_sag: Length::new(4e-3f64),
                max_length: Some(Length::new(1.0)),
                ..TessellationOptions::default()
            },
        );

        op.tessellate(&Mat3::identity(), &Vec3::new(0f32, 0f32, 0f32));
        let mesh = op.into_mesh();

        // check the orientation of the triangles
        let positions = mesh.get_vertices().get_positions();
        let normals = mesh.get_vertices().get_normals().unwrap();
        let indices = mesh
            .get_primitives()
            .get_raw_index_data()
            .get_indices_ref()
            .unwrap();
        indices.chunks(3).for_each(|triangle| {
            let v0 = positions[triangle[0] as usize].0;
            let v1 = positions[triangle[1] as usize].0;
            let v2 = positions[triangle[2] as usize].0;

            let n0 = normals[triangle[0] as usize].0;
            let n1 = normals[triangle[1] as usize].0;
            let n2 = normals[triangle[2] as usize].0;

            let face_normal = (n0 + n1 + n2).normalize();

            let a = v1 - v0;
            let b = v2 - v0;

            let n = a.cross(&b).normalize();

            assert!(
                n.dot(&v0) > 0f32,
                "Normal has wrong orientation. Indices={:?}, Triangle=({:?},{:?},{:?}), Face Normal: {:?}, Calculated Normal: {:?}",
                triangle,
                v0,
                v1,
                v2,
                face_normal,
                n
            );
        });
    }
}
