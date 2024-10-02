use std::collections::HashMap;

use nalgebra_glm::{Mat3, Vec3};

use crate::{
    loader::{loader_rvm::primitive::SphereData, TessellationOptions},
    structure::{IndexData, Mesh, Normal, Normals, Point3D, Positions, Primitives, Vertices},
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

/// The sphere tessellation operator is used to tessellate a sphere by subdividing an icosahedron
/// based on the specified sphere data and tessellation options.
pub struct SphereTessellationOperator {
    radius_mm: f32,

    /// In order to guarantee all tessellation options are met, it is sufficient to only define the
    /// maximum edge length.
    max_edge_length: f32,

    /// The middle vertex of an edge is stored in a hashmap to avoid duplicate vertices.
    map_edge_middle_vertex: HashMap<(u32, u32), u32>,

    positions: Positions,
    normals: Normals,
    indices: Vec<u32>,
}

impl SphereTessellationOperator {
    /// Creates a new sphere tessellation operator. That is, an operator that tessellates a
    /// sphere based on the specified sphere data and tessellation options.
    ///
    /// # Arguments
    /// * `sphere_data` - The sphere data to use for the tessellation.
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
            map_edge_middle_vertex: HashMap::new(),
            positions: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Tessellates the sphere based on the specified transformation and translation.
    /// Function may only be called once.
    ///
    /// # Arguments
    /// * `transform` - The transformation matrix to apply to the sphere.
    /// * `translation` - The translation vector to apply to the sphere.
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

        // start by first registering the vertices of the icosahedron
        self.register_icosahedron_vertices();

        // create the indices of the tessellated sphere
        self.create_indices();

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

    /// Registers the vertices of the icosahedron to initialize the tessellation.
    fn register_icosahedron_vertices(&mut self) {
        self.positions.extend(
            ICOSAHEDRON_VERTICES
                .iter()
                .map(|v| Point3D(*v * self.radius_mm)),
        );

        self.normals
            .extend(ICOSAHEDRON_VERTICES.iter().map(|v| Normal { 0: *v }));
    }

    /// Creates the indices of the tessellated sphere by subdividing the icosahedron until the
    /// edge length is below the maximum edge length.
    fn create_indices(&mut self) {
        let mut triangle_stack: Vec<[u32; 3]> = ICOSAHEDRON_INDICES
            .chunks(3)
            .map(|chunk| [chunk[0], chunk[1], chunk[2]])
            .collect();

        while let Some(t) = triangle_stack.pop() {
            let v0 = self.positions[t[0] as usize];
            let v1 = self.positions[t[1] as usize];
            let v2 = self.positions[t[2] as usize];

            let edge_length = Self::determine_edge_length_of_triangle(v0, v1, v2);

            if edge_length > self.max_edge_length {
                let v01 = self.register_middle_vertex(t[0], t[1]);
                let v12 = self.register_middle_vertex(t[1], t[2]);
                let v20 = self.register_middle_vertex(t[2], t[0]);

                triangle_stack.push([t[0], v01, v20]);
                triangle_stack.push([t[1], v12, v01]);
                triangle_stack.push([t[2], v20, v12]);
                triangle_stack.push([v01, v12, v20]);
            } else {
                self.indices.extend_from_slice(&t);
            }
        }
    }

    /// Registers the middle vertex of the edge defined by the two vertices.
    /// If the middle vertex has already been registered, the index of the middle vertex is
    /// returned. Otherwise, the middle vertex is registered and the index of the middle vertex is
    /// returned.
    ///
    /// # Arguments
    /// * `v0` - The first vertex of the edge.
    /// * `v1` - The second vertex of the edge.
    fn register_middle_vertex(&mut self, v0: u32, v1: u32) -> u32 {
        let edge = (v0.min(v1), v0.max(v1));

        if let Some(index) = self.map_edge_middle_vertex.get(&edge) {
            *index
        } else {
            let v0 = self.positions[v0 as usize];
            let v1 = self.positions[v1 as usize];

            let normal = (v0.0 + v1.0).normalize();

            // create the middle point by reprojecting the middle point onto the sphere
            let middle = Point3D(normal * self.radius_mm);

            let normal = Point3D(normal);

            let index = self.positions.len() as u32;
            self.positions.push(middle);
            self.normals.push(normal);

            self.map_edge_middle_vertex.insert(edge, index);

            index
        }
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

    /// Determines the edge length of the triangle defined by the three vertices.
    ///
    /// # Arguments
    /// * `v0` - The first vertex of the triangle.
    /// * `v1` - The second vertex of the triangle.
    /// * `v2` - The third vertex of the triangle.
    fn determine_edge_length_of_triangle(v0: Point3D, v1: Point3D, v2: Point3D) -> f32 {
        let edge0 = (v0.0 - v1.0).norm();
        let edge1 = (v1.0 - v2.0).norm();
        let edge2 = (v2.0 - v0.0).norm();

        edge0.max(edge1).max(edge2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sphere_tessellation() {
        let t0 = std::time::Instant::now();
        let sphere_data = SphereData { diameter: 1.0 };
        let tessellation_options = TessellationOptions {
            max_length: Some(Length::new(0.0001)),
            ..Default::default()
        };

        let mut operator = SphereTessellationOperator::new(&sphere_data, &tessellation_options);
        operator.tessellate(&Mat3::identity(), &Vec3::zeros());

        let mesh = operator.into_mesh();

        println!("Time: {:.2} ms", (t0.elapsed().as_secs_f64() * 1e3f64));

        println!(
            "Number of vertices: {}",
            mesh.get_vertices().get_positions().len()
        );
        println!(
            "Number of triangles: {}",
            mesh.get_primitives()
                .get_raw_index_data()
                .get_indices_ref()
                .unwrap()
                .len()
                / 3
        );

        // check that all triangles are oriented correctly
        let indices = mesh
            .get_primitives()
            .get_raw_index_data()
            .get_indices_ref()
            .unwrap();
        let positions = mesh.get_vertices().get_positions();
        for triangle in indices.chunks(3) {
            let v0 = positions[triangle[0] as usize];
            let v1 = positions[triangle[1] as usize];
            let v2 = positions[triangle[2] as usize];

            let a = v1.0 - v0.0;
            let b = v2.0 - v0.0;
            let normal = a.cross(&b).normalize();

            let center = (v0.0 + v1.0 + v2.0) / 3.0;

            assert!(normal.dot(&center) > 0.0);
        }
    }
}
