use std::io::Write;

use itertools::Itertools;

use log::{debug, warn};
use nalgebra_glm::{Mat4, Vec3};
use quick_xml::{events::attributes::Attribute, writer::Writer, Error as XMLError};

use crate::{
    structure::{CADData, IndexData, Material, Mesh, Node, PrimitiveType, ShapePart, Vertices},
    Error,
};

pub struct X3DExporter<'a> {
    cad_data: &'a CADData,
}

impl<'a> X3DExporter<'a> {
    /// Creates a new X3D exporter for the given cad data.
    ///
    /// # Arguments
    /// * `cad_data` - The CAD data to export.
    pub fn new(cad_data: &'a CADData) -> Self {
        Self { cad_data }
    }

    /// Starts writing the CAD data to the given writer as X3D.
    ///
    /// # Arguments
    /// * `w` - The writer to which the CAD data will be serialized as X3D.
    pub fn write<W: Write>(&self, w: W) -> Result<(), Error> {
        let writer = Writer::new_with_indent(w, b' ', 2);

        debug!("Start writing the XML...");
        match self.write_xml(writer) {
            Ok(()) => {
                debug!("Finished writing the XML");
                Ok(())
            }
            Err(err) => Err(Error::IO(format!("Failed writing XML due to {}", err))),
        }
    }

    /// The central internal entry point for writing the XML data.
    ///
    /// # Arguments
    /// * `writer` - The XML serialize writer.
    fn write_xml<W: Write>(&self, writer: Writer<W>) -> Result<(), XMLError> {
        let mut writer = writer;

        let x3d = writer.create_element("X3D");
        x3d.write_inner_content(|writer| {
            writer
                .create_element("Scene")
                .with_attribute(Attribute::from(("DEF", "scene")))
                .write_inner_content(|writer| {
                    let root_node = self.cad_data.get_root_node();
                    self.write_node(writer, root_node)?;

                    Ok(())
                })?;

            Ok(())
        })?;

        // });
        // x3d.with_attribute(Attribute::from(("profile", "Immersive"))).write_empty().unwrap();

        // <X3D profile='Immersive' version='3.0' xmlns:xsd='http://www.w3.org/2001/XMLSchema-instance' xsd:noNamespaceSchemaLocation='http://www.web3d.org/specifications/x3d-3.0.xsd'>
        // writer.create_element("X3D").with_attribute(attr);

        Ok(())
    }

    fn write_node<W: Write>(&self, writer: &mut Writer<W>, node: &Node) -> Result<(), XMLError> {
        // create the serialized string for the transformation matrix
        let m = node.get_transform().unwrap_or(Mat4::identity());
        let matrix_string: String = Itertools::intersperse(
            m.column_iter()
                .map(|c| format!("{} {} {} {}", c[0], c[1], c[2], c[3])),
            " ".to_owned(),
        )
        .collect();

        let group = writer
            .create_element("MatrixTransform")
            .with_attribute(Attribute::from(("matrix", matrix_string.as_str())));

        group.write_inner_content(|writer| {
            Self::write_label(writer, node.get_label())?;

            // add shape information to the current node if available
            for shape in node.get_shapes() {
                for part in shape.get_parts() {
                    Self::write_part(writer, part)?;
                }
            }

            // process children of current node
            for child in node.get_children() {
                self.write_node(writer, child)?;
            }

            Ok(())
        })?;

        Ok(())
    }

    /// Writes a meta data set to the given writer which contains the given label.
    ///
    /// # Arguments
    /// * `writer` - The writer to which the metadata set will be added
    /// * `label` - The node label which is added to the metadata set.
    fn write_label<W: Write>(writer: &mut Writer<W>, label: &str) -> Result<(), XMLError> {
        let metadata_set = writer.create_element("MetadataSet");
        metadata_set
            .with_attribute(Attribute::from(("containerField", "metadata")))
            .write_inner_content(|writer| {
                writer
                    .create_element("MetadataString")
                    .with_attribute(Attribute::from(("containerField", "value")))
                    .with_attribute(Attribute::from(("name", "Name")))
                    .with_attribute(Attribute::from(("value", label)))
                    .write_empty()?;

                Ok(())
            })?;

        Ok(())
    }

    /// Writes a single shape part as shape to the X3D.
    ///
    /// # Arguments
    /// * `writer` - The XML writer to which the shape node will be added.
    /// * `part` - The shape part to be written out as shape.
    fn write_part<W: Write>(writer: &mut Writer<W>, part: &ShapePart) -> Result<(), XMLError> {
        let shape = writer.create_element("Shape");

        shape.write_inner_content(|writer| {
            // write material
            match part.get_material().as_ref() {
                Material::PhongMaterial(phong_data) => {
                    let diffuse_color = phong_data.diffuse_color.0;
                    let specular_color = phong_data.specular_color.0;

                    writer
                        .create_element("Appearance")
                        .write_inner_content(|writer| {
                            let xml_mat = writer.create_element("Material");
                            xml_mat
                                .with_attribute(Attribute::from((
                                    "diffuseColor",
                                    format!(
                                        "{} {} {}",
                                        diffuse_color[0], diffuse_color[1], diffuse_color[2]
                                    )
                                    .as_str(),
                                )))
                                .with_attribute(Attribute::from((
                                    "specularColor",
                                    format!(
                                        "{} {} {}",
                                        specular_color[0], specular_color[1], specular_color[2]
                                    )
                                    .as_str(),
                                )))
                                .write_empty()?;

                            Ok(())
                        })?;
                }
                Material::None => {}
            }

            // write mesh
            let mesh = part.get_mesh();
            Self::write_mesh(writer, &mesh)?;

            Ok(())
        })?;

        Ok(())
    }

    /// Writes the given mesh data to the XML writer.
    ///
    /// # Arguments
    /// * `writer` - The XML writer to which the tessellation data will be written.
    /// * `mesh` - The mesh data which is written out as a X3D tessellation geometry node.
    fn write_mesh<W: Write>(writer: &mut Writer<W>, mesh: &Mesh) -> Result<(), XMLError> {
        let vertices = mesh.get_vertices();
        let primitives = mesh.get_primitives();
        let primitive_type = primitives.get_primitive_type();
        let index_data = primitives.get_raw_index_data();

        match (primitive_type, index_data) {
            (PrimitiveType::Triangles, IndexData::NonIndexed(_)) => {
                writer
                    .create_element("TriangleSet")
                    .write_inner_content(|w| Self::write_vertices(w, vertices))?;
            }
            (PrimitiveType::Triangles, IndexData::Indices(indices)) => {
                let index_str: String =
                    Itertools::intersperse(indices.iter().map(|i| i.to_string()), " ".to_owned())
                        .collect();

                writer
                    .create_element("IndexedTriangleSet")
                    .with_attribute(Attribute::from(("index", index_str.as_str())))
                    .write_inner_content(|w| Self::write_vertices(w, vertices))?;
            }
            _ => {
                warn!("Skipping writing geometry");
            }
        }

        Ok(())
    }

    /// Writes the attributes of the given vertices to the XML writer.
    ///
    /// # Arguments
    /// * `writer` - The XML writer to which the X3D attribute nodes will be written.
    /// * `vertices` - The vertices data that is written to the XML writer.
    fn write_vertices<W: Write>(
        writer: &mut Writer<W>,
        vertices: &Vertices,
    ) -> Result<(), XMLError> {
        let positions_str = Self::vec3_to_string(vertices.get_positions().iter().map(|p| p.0));

        writer
            .create_element("Coordinate")
            .with_attribute(Attribute::from(("point", positions_str.as_str())))
            .write_empty()?;

        Ok(())
    }

    /// Returns a concatenated string of all coordinates of all given vectors separated by spaces.
    ///
    /// # Arguments
    /// * `vecs` - An iterator onto the vec3's to concatenate.
    fn vec3_to_string<I>(vecs: I) -> String
    where
        I: Iterator<Item = Vec3>,
    {
        Itertools::intersperse(
            vecs.map(|v| format!("{} {} {}", v[0], v[1], v[2])),
            " ".to_owned(),
        )
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::loader::{loader_off::LoaderOff, Loader, MemoryResource};

    use super::*;

    fn load_example_cad_data() -> CADData {
        let data = include_bytes!("../loader/test_data/cube.off");
        let r = MemoryResource::new(data, "model/vnd.off".to_owned());
        let l = LoaderOff::new();

        l.read(&r).unwrap()
    }

    #[test]
    fn test_x3d_writer() {
        let cad_data = load_example_cad_data();

        let mut data: Vec<u8> = Vec::new();
        {
            let c = Cursor::new(&mut data);
            let x = X3DExporter::new(&cad_data);
            x.write(c).unwrap();
        }

        let s = String::from_utf8(data).unwrap();
        println!("{}", s);
    }
}
