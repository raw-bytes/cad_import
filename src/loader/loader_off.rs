use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    io::{BufRead, BufReader, Error as IOError},
    iter::Peekable,
    rc::Rc,
    str::{FromStr, SplitAsciiWhitespace},
};

use crate::{
    basic_types::RGBA,
    error::Error,
    structure::{
        CADData, Colors, IndexData, Mesh, Node, Point3D, Positions, PrimitiveType, Primitives,
        Shape, ShapePart, Vertices,
    },
};

use super::{
    loader::{ExtensionMap, Loader},
    OptionsDescriptor, Resource,
};

use log::{debug, trace};

/// A single read line
type LineWithNumber = (usize, Result<String, IOError>);

/// A loader for OFF (Object File Format)
/// Specification: See `<https://segeval.cs.princeton.edu/public/off_format.html>`
pub struct LoaderOff {}

impl LoaderOff {
    pub fn new() -> Self {
        Self {}
    }

    /// Simple wrapper for reading a line from the given lines. Fails if there is no line left
    /// or we hit the end.
    /// The method returns the line number and the corresponding string if successful.
    fn read_line(line: Option<&LineWithNumber>) -> Result<(usize, String), Error> {
        match line {
            Some((line_index, line)) => match line {
                Ok(line) => Ok((line_index + 1, line.clone())),
                Err(err) => Err(Error::IO(format!(
                    "Failed reading line {} due to {}",
                    line_index + 1,
                    err
                ))),
            },
            None => Err(Error::IO("Expected next line to read".to_owned())),
        }
    }

    /// Reads a single number from the split string. Fails if the number cannot be parsed or if
    /// there is no further number.
    fn read_number<'a, N, E>(
        chunks: &mut SplitAsciiWhitespace<'a>,
        line_number: usize,
    ) -> Result<N, Error>
    where
        E: Display,
        N: FromStr<Err = E>,
    {
        match chunks.next() {
            Some(chunk) => match chunk.trim().parse() {
                Ok(n) => Ok(n),
                Err(err) => Err(Error::InvalidFormat(format!(
                    "Invalid number in line {}. {}",
                    line_number, err
                ))),
            },
            None => Err(Error::IO(format!(
                "Expected next number to read in line {}",
                line_number
            ))),
        }
    }

    /// Reads and checks the header which is the first line of lines.
    fn read_header(line: Option<&LineWithNumber>) -> Result<(), Error> {
        trace!("Read header...");

        let (line_number, header) = Self::read_line(line)?;

        if header.trim() == "OFF" {
            Ok(())
        } else {
            Err(Error::InvalidFormat(format!(
                "File has invalid header. Expected OFF in line {}, but found '{}'",
                line_number, header
            )))
        }
    }

    /// Reads the number of vertices and faces of the OFF file
    fn read_num_vertices_and_faces(line: Option<&LineWithNumber>) -> Result<(usize, usize), Error> {
        trace!("Read number of vertices and faces...");

        let (line_number, line) = Self::read_line(line)?;
        let mut chunks = line.split_ascii_whitespace();

        let num_vertices: usize = Self::read_number(&mut chunks, line_number)?;
        let num_faces: usize = Self::read_number(&mut chunks, line_number)?;

        debug!("#Vertices={}, #Faces={}", num_vertices, num_faces);

        Ok((num_vertices, num_faces))
    }

    /// Reads the vertices which consist of position and optionally also have colors.
    fn read_vertices<I>(lines: &mut Peekable<I>, num_vertices: usize) -> Result<Vertices, Error>
    where
        I: Iterator<Item = LineWithNumber>,
    {
        // handle special case of zero vertices
        if num_vertices == 0 {
            return Ok(Vertices::new());
        }

        let mut positions = Positions::with_capacity(num_vertices);

        // determine if the we have colors
        let do_we_have_colors = (Self::read_line(lines.peek())?)
            .1
            .split_ascii_whitespace()
            .count()
            >= 7;

        let mut colors = if do_we_have_colors {
            Colors::with_capacity(num_vertices)
        } else {
            Colors::new()
        };

        // parse vertices
        for _ in 0..num_vertices {
            let (line_number, line) = Self::read_line(lines.next().as_ref())?;

            let mut chunks = line.split_ascii_whitespace();

            let x = Self::read_number(&mut chunks, line_number)?;
            let y = Self::read_number(&mut chunks, line_number)?;
            let z = Self::read_number(&mut chunks, line_number)?;
            let position = Point3D::new(x, y, z);
            positions.push(position);

            if do_we_have_colors {
                let r = Self::read_number(&mut chunks, line_number)?;
                let g = Self::read_number(&mut chunks, line_number)?;
                let b = Self::read_number(&mut chunks, line_number)?;
                let a = Self::read_number(&mut chunks, line_number)?;

                let color = RGBA::new(r, g, b, a);
                colors.push(color);
            }
        }

        let mut vertices = Vertices::from_positions(positions);
        if !colors.is_empty() {
            match vertices.set_colors(colors) {
                Err(err) => {
                    return Err(Error::Internal(format!(
                        "An internal error occurred while setting the colors attribute. {}",
                        err
                    )));
                }
                Ok(()) => {}
            }
        }

        Ok(vertices)
    }

    /// Reads the primitives and converts them to triangles.
    fn read_primitives<I>(
        lines: &mut Peekable<I>,
        num_faces: usize,
        num_vertices: usize,
    ) -> Result<Primitives, Error>
    where
        I: Iterator<Item = LineWithNumber>,
    {
        let mut indices: Vec<u32> = Vec::with_capacity(num_faces * 3);

        // iterate over faces and create triangle indices
        for _ in 0..num_faces {
            let (line_number, line) = Self::read_line(lines.next().as_ref())?;

            let mut chunks = line.split_ascii_whitespace();

            // start reading the number of indices for the current face.
            let n: u32 = Self::read_number(&mut chunks, line_number)?;

            // read first two indices
            let v0: u32 = Self::read_number(&mut chunks, line_number)?;
            let mut v1: u32 = Self::read_number(&mut chunks, line_number)?;

            // read remaining indices
            for _ in 0..(n - 2) {
                let v2 = Self::read_number(&mut chunks, line_number)?;

                // check if one of the indices is outside of the range
                if v0.max(v1).max(v2) as usize >= num_vertices {
                    return Err(Error::InvalidFormat(format!(
                        "Got index which is out of range. Got {} vertices, but have index {}",
                        num_vertices,
                        v0.max(v1).max(v2)
                    )));
                }

                indices.push(v0);
                indices.push(v1);
                indices.push(v2);

                v1 = v2;
            }
        }

        // create the primitives
        let primitives = Primitives::new(IndexData::Indices(indices), PrimitiveType::Triangles)?;

        Ok(primitives)
    }

    /// Creates CAD data from the given vertices and primitives.
    fn create_cad_data(vertices: Vertices, primitives: Primitives) -> Result<CADData, Error> {
        trace!("Create CAD data...");

        // create mesh and shape from the given vertices and primitives
        let mesh = Mesh::new(vertices, primitives)?;
        let part = ShapePart::new(Rc::new(mesh), Default::default());
        let mut shape = Shape::new();
        shape.add_part(part);

        // create root node and attach shape to it
        let mut root_node = Node::new("root".to_owned());
        root_node.attach_shape(Rc::new(shape));

        // finally, create the cad data
        let cad_data = CADData::new(root_node);

        Ok(cad_data)
    }
}

impl Loader for LoaderOff {
    fn get_extensions_mime_type_map(&self) -> ExtensionMap {
        let mut ext_map = BTreeMap::new();

        ext_map.insert(
            "off".to_owned(),
            BTreeSet::from(["model/vnd.off".to_owned()]),
        );

        ext_map
    }

    fn get_mime_types(&self) -> Vec<String> {
        vec!["model/vnd.off".to_owned()]
    }

    fn get_name(&self) -> &str {
        "Object File Format"
    }

    fn get_priority(&self) -> u32 {
        1000
    }

    fn get_loader_options(&self) -> Option<OptionsDescriptor> {
        None
    }

    fn read_with_options(
        &self,
        resource: &dyn Resource,
        _: Option<super::LoaderOptions>,
    ) -> Result<CADData, Error> {
        let reader = resource.open().unwrap();
        let reader = BufReader::new(reader);
        let mut lines = reader.lines().enumerate();

        Self::read_header(lines.next().as_ref())?;
        let (num_vertices, num_faces) = Self::read_num_vertices_and_faces(lines.next().as_ref())?;

        let mut lines = lines.peekable();
        let vertices = Self::read_vertices(&mut lines, num_vertices)?;

        let primitives = Self::read_primitives(&mut lines, num_faces, num_vertices)?;
        let cad_data = Self::create_cad_data(vertices, primitives)?;

        Ok(cad_data)
    }
}

#[cfg(test)]
mod tests {
    use nalgebra_glm::{cross, Vec3, U3};

    use crate::loader::MemoryResource;

    use super::*;

    /// Computes the bounding volume for the given positions.
    fn compute_bbox(positions: &[Point3D]) -> (Vec3, Vec3) {
        let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);

        for p in positions.iter() {
            let p = p.0;

            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);

            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        }

        (min, max)
    }

    fn compute_area(positions: &[Point3D], indices: &[u32]) -> f32 {
        assert_eq!(indices.len() % 3, 0);

        let mut total_area = 0f32;
        for t in indices.iter().as_slice().windows(3).step_by(3) {
            let v0 = positions[t[0] as usize].0;
            let v1 = positions[t[1] as usize].0;
            let v2 = positions[t[2] as usize].0;

            let a = v1 - v0;
            let b = v2 - v0;

            let n = cross::<_, U3>(&a, &b);

            let area = nalgebra_glm::l2_norm(&n) * 0.5f32;
            total_area += area;
        }

        total_area
    }

    #[test]
    fn test_cube() {
        let s = include_str!("test_data/cube.off");

        let r = MemoryResource::new(s.as_bytes(), "model/vnd.off".to_owned());

        let loader = LoaderOff::new();

        let cad_data = loader.read(&r).unwrap();
        let root_node = cad_data.get_root_node();
        assert!(root_node.is_leaf());

        let shapes = root_node.get_shapes();
        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        let parts = shape.get_parts();
        assert_eq!(parts.len(), 1);

        let part = parts.first().unwrap();
        let mesh = part.get_mesh();

        let vertices = mesh.get_vertices();
        let primitives = mesh.get_primitives();

        assert_eq!(vertices.len(), 8);
        assert_eq!(vertices.get_colors(), None);
        assert_eq!(vertices.get_normals(), None);

        assert_eq!(primitives.num_primitives(), 12);
        assert_eq!(primitives.get_primitive_type(), PrimitiveType::Triangles);

        // compute surface area and bounding volume of the cube
        let (min, max) = compute_bbox(vertices.get_positions());

        assert_eq!(min, Vec3::new(-0.5f32, -0.5f32, -0.5f32));
        assert_eq!(max, Vec3::new(0.5f32, 0.5f32, 0.5f32));

        let area = compute_area(
            vertices.get_positions(),
            primitives.get_raw_index_data().get_indices_ref().unwrap(),
        );
        assert!((area - 6f32).abs() <= 1e-6f32);
    }
}
