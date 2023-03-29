use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::Display,
    rc::Rc,
};

use gltf::{
    accessor::{DataType as GLTFDataType, Dimensions},
    buffer::{Source, View},
    iter::Buffers,
    material::AlphaMode,
    mesh::{iter::Attributes, Mode},
    scene::Transform,
    Accessor, Document, Gltf, Material as GLTFMaterial, Mesh as GLTFMesh, Node as GLTFNode,
    Primitive as GLTFPrimitive, Semantic,
};
use log::{debug, warn};
use nalgebra_glm::{Mat4, Vec3};

use crate::{
    loader::{Loader, OptionsDescriptor, Resource},
    structure::{
        CADData, IndexData, Material, Mesh, Node, Normals, PhongMaterialData, Positions,
        PrimitiveType, Primitives, Shape, ShapePart, Vertices,
    },
    Color, Error, RGB,
};

use super::{accessor_iterator::AccessorIterator, component::ComponentTrait, utils::transmute_vec};

/// A loader for GLTF 2.0
/// Specification: See `<https://www.khronos.org/gltf/>`
pub struct LoaderGLTF {}

impl LoaderGLTF {
    pub fn new() -> Self {
        Self {}
    }

    /// Resolves the buffer references for the specified GLTF.
    ///
    /// # Arguments
    /// * `resource` - The resource specification to the GLTF.
    /// * `buffers` - The GLTF buffer specification.
    /// * `embedded_buffer` - An optional embedded buffer inside the GLB.
    fn resolve_buffers(
        resource: &dyn Resource,
        buffers: Buffers,
        embedded_buffer: Option<Vec<u8>>,
    ) -> Result<Vec<Vec<u8>>, Error> {
        let mut buffers = buffers;
        let mut blobs = Vec::new();

        // check if there is an embedded buffer
        match embedded_buffer {
            Some(buffer) => {
                blobs.push(buffer);
                buffers.next();
            }
            _ => {}
        }

        // check other buffers
        for buffer in buffers {
            match buffer.source() {
                Source::Uri(uri) => {
                    let bin_resource = resource.sub(uri, "application/octet-stream")?;
                    let blob = bin_resource.read_to_memory()?;

                    if buffer.length() < blob.len() {
                        return Err(Error::InvalidFormat(format!(
                            "Specified buffer has length {}, but loaded buffer has only length {}",
                            buffer.length(),
                            blob.len()
                        )));
                    }

                    blobs.push(blob);
                }
                Source::Bin => {
                    return Err(Error::InvalidFormat(format!(
                        "Only the first chunk can be binary"
                    )));
                }
            }
        }

        Ok(blobs)
    }

    /// Creates CAD data based on the provided document and blobs.
    ///
    /// # Arguments
    /// * `document` - The GLTF document
    /// * `blobs` - The buffers associated with the GLTF.
    fn create_cad_data(document: Document, blobs: Vec<Vec<u8>>) -> Result<CADData, Error> {
        let creator = CADDataCreator::new();

        let gltf_data = GLTFData { document, blobs };
        let cad_data = creator.create(&gltf_data)?;

        Ok(cad_data)
    }
}

impl Loader for LoaderGLTF {
    fn get_extensions_mime_type_map(&self) -> crate::loader::ExtensionMap {
        let mut ext_map = BTreeMap::new();

        ext_map.insert(
            "gltf".to_owned(),
            BTreeSet::from(["model/gltf+json".to_owned()]),
        );
        ext_map.insert(
            "glb".to_owned(),
            BTreeSet::from(["model/gltf-binary".to_owned()]),
        );

        ext_map
    }

    fn get_mime_types(&self) -> Vec<String> {
        vec!["model/gltf-binary".to_owned(), "model/gltf+json".to_owned()]
    }

    fn get_name(&self) -> &str {
        "glTF RUNTIME 3D ASSET DELIVERY"
    }

    fn get_priority(&self) -> u32 {
        1000
    }

    fn get_loader_options(&self) -> Option<OptionsDescriptor> {
        None
    }

    fn read(&self, resource: &dyn Resource) -> Result<CADData, Error> {
        let buffer = resource.read_to_memory()?;

        let gltf_data = match Gltf::from_slice(&buffer) {
            Ok(g) => g,
            Err(err) => {
                return Err(Error::InvalidFormat(format!(
                    "Failed reading GLTF due to {}",
                    err
                )));
            }
        };

        let d = gltf_data.document;

        let buffers = Self::resolve_buffers(resource, d.buffers(), gltf_data.blob)?;
        debug!("Got {} buffers", buffers.len());

        Self::create_cad_data(d, buffers)
    }
}

struct GLTFData {
    pub document: Document,
    pub blobs: Vec<Vec<u8>>,
}

struct CADDataCreator {
    shape_map: HashMap<usize, Rc<Shape>>,
    material_map: HashMap<usize, Rc<Material>>,
}

impl CADDataCreator {
    /// Returns a new empty CAD data creator object.
    pub fn new() -> Self {
        Self {
            shape_map: HashMap::new(),
            material_map: HashMap::new(),
        }
    }

    /// Creates the CAD-Data from the given GLTF data.
    ///
    /// # Arguments
    /// * `gltf_data` - The GLTF data used for creating the overall CAD data.
    pub fn create(self, gltf_data: &GLTFData) -> Result<CADData, Error> {
        let mut creator = self;

        creator.create_materials(gltf_data)?;
        creator.create_shapes(gltf_data)?;
        let root_node = creator.create_nodes(gltf_data)?;

        Ok(CADData::new(root_node))
    }

    /// Creates a tree from all GLTF scenes and data.
    ///
    /// # Arguments
    /// * `gltf_data` - The GLTF data which is used for parsing and creating the tree.
    fn create_nodes(&self, gltf_data: &GLTFData) -> Result<Node, Error> {
        // iterate over the list of GLTF scenes and create a node for each scene
        let scenes = gltf_data.document.scenes();
        let mut root_nodes: Vec<Node> = Vec::with_capacity(scenes.len());
        for scene in scenes {
            let label = match scene.name() {
                Some(s) => s.to_owned(),
                None => "".to_owned(),
            };

            let mut scene_root_node = Node::new(label);

            for node in scene.nodes() {
                scene_root_node.add_child(self.process_node(gltf_data, node)?);
            }

            root_nodes.push(scene_root_node);
        }

        // check if we have 1 or more scenes or none at all which is an error
        match root_nodes.len() {
            0 => Err(Error::InvalidFormat(format!("No scenes at all"))),
            1 => Ok(root_nodes.pop().unwrap()),
            _ => {
                let mut root_node = Node::new("root".to_owned());
                for n in root_nodes {
                    root_node.add_child(n);
                }

                Ok(root_node)
            }
        }
    }

    /// Create a tree from the given node.
    ///
    /// # Arguments
    /// * `gltf_data` - The GLTF data which is used for parsing and creating the tree.
    /// * `in_node` - The gltf node which defines the subtree.
    fn process_node(&self, gltf_data: &GLTFData, in_node: GLTFNode) -> Result<Node, Error> {
        let label = match in_node.name() {
            Some(s) => s.to_owned(),
            None => "".to_owned(),
        };

        let mut out_node = Node::new(label);

        // set the matrix for the node
        let m = Self::transform_to_matrix(in_node.transform());
        out_node.set_transform(m);

        // attach shapes to the node
        match in_node.mesh() {
            Some(mesh) => {
                let mesh_index = mesh.index();
                match self.shape_map.get(&mesh_index) {
                    Some(shape) => {
                        out_node.attach_shape(shape.clone());
                    }
                    None => {
                        return Err(Error::InvalidFormat(format!(
                            "Could not find mesh with index {}",
                            mesh_index
                        )));
                    }
                }
            }
            None => {}
        }

        // iterate over the children
        for in_child in in_node.children() {
            let out_child = self.process_node(gltf_data, in_child)?;
            out_node.add_child(out_child);
        }

        Ok(out_node)
    }

    /// Returns a matrix 4 from the given GLTF transformation.
    ///
    /// # Arguments
    /// * `t` - The input transformation.
    fn transform_to_matrix(t: Transform) -> Mat4 {
        let values = t.matrix();

        let mut m = Mat4::zeros();
        for (mut dst_col, src_col) in m.column_iter_mut().zip(values.iter()) {
            dst_col.copy_from_slice(src_col);
        }

        m
    }

    /// Creates the materials from the GLTF materials.
    fn create_materials(&mut self, gltf_data: &GLTFData) -> Result<(), Error> {
        for (material_index, material) in gltf_data.document.materials().enumerate() {
            let material = Rc::new(self.create_material(material)?);
            self.material_map.insert(material_index, material);
        }

        Ok(())
    }

    /// Creates a phong material from the given PBR material.
    ///
    /// # Arguments
    /// * `material` - The GLTF material used for creating the phong material
    fn create_material(&self, material: GLTFMaterial) -> Result<Material, Error> {
        let [r, g, b, alpha_value] = material.pbr_metallic_roughness().base_color_factor();
        let diffuse_color = RGB(Vec3::new(r, g, b));

        let alpha_value = match material.alpha_mode() {
            AlphaMode::Opaque => 1f32,
            AlphaMode::Mask => alpha_value,
            AlphaMode::Blend => match material.alpha_cutoff() {
                None => alpha_value,
                Some(alpha_cut_off) => {
                    if alpha_value <= alpha_cut_off {
                        0f32
                    } else {
                        1f32
                    }
                }
            },
        };

        let mut phong_data = PhongMaterialData::default();
        phong_data.diffuse_color = diffuse_color;
        phong_data.transparency = 1f32 - alpha_value;

        Ok(Material::PhongMaterial(phong_data))
    }

    /// Returns the default material. If it doesn't exists, it will be created.
    fn get_default_material(&mut self) -> Rc<Material> {
        let default_material_index = usize::MAX;

        match self.material_map.get(&default_material_index) {
            Some(m) => m.clone(),
            None => {
                let mut phong_data: PhongMaterialData = Default::default();
                phong_data.diffuse_color = RGB::black();
                let default_material = Rc::new(Material::PhongMaterial(phong_data));

                self.material_map
                    .insert(default_material_index, default_material.clone());

                default_material
            }
        }
    }

    /// Returns a material for the given GLTF material. If the material cannot be found, a warning
    /// is emitted and the default material is returned instead.
    ///
    /// # Arguments
    ///
    /// * `material` - The GLTF material to translate to material.
    fn get_material(&mut self, material: GLTFMaterial) -> Rc<Material> {
        // check if the given GLTF material has an index defined
        let index = match material.index() {
            Some(index) => index,
            None => return self.get_default_material(),
        };

        // use index to lookup the material
        match self.material_map.get(&index) {
            Some(m) => {
                return m.clone();
            }
            None => {
                warn!(
                    "Cannot find material with index {}. Take default material",
                    index
                );
                return self.get_default_material();
            }
        }
    }

    /// Creates an internal map from GLTF mesh index to shape.
    ///
    /// # Arguments
    /// * `gltf_data` - The overall loaded GLTF data.
    fn create_shapes(&mut self, gltf_data: &GLTFData) -> Result<(), Error> {
        let meshes = gltf_data.document.meshes();

        for mesh in meshes {
            let mesh_index = mesh.index();
            let shape = Rc::new(self.create_shape(mesh, gltf_data)?);

            self.shape_map.insert(mesh_index, shape);
        }

        Ok(())
    }

    /// Creates a shape from of the given GLTF mesh.
    ///
    /// # Arguments
    /// * `gltf_data` - The overall loaded GLTF data.
    /// * `mesh` - The GLTF mesh that is parsed to create the shape.
    fn create_shape(&mut self, mesh: GLTFMesh, gltf_data: &GLTFData) -> Result<Shape, Error> {
        let mut shape = Shape::new();

        let primitives = mesh.primitives();
        for primitive in primitives {
            let material = self.get_material(primitive.material());

            // create the mesh primitive data
            let primitive_type = Self::translate_primitive_mode(primitive.mode());
            let index_data = Self::create_index_data(gltf_data, primitive.clone())?;
            let mesh_primitives = Primitives::new(index_data, primitive_type)?;

            // create positions
            let positions: Positions = match Self::find_accessor_by_semantic(
                primitive.attributes(),
                Semantic::Positions,
            ) {
                Some(accessor) => transmute_vec(Self::create_vec3_data(gltf_data, accessor)?),
                None => {
                    return Err(Error::InvalidFormat(format!(
                        "Missing position attribute for the primitive data"
                    )));
                }
            };

            let num_vertices = positions.len();
            let mut vertices = Vertices::from_positions(positions);

            match Self::find_accessor_by_semantic(primitive.attributes(), Semantic::Normals) {
                Some(accessor) => {
                    let normals: Normals =
                        transmute_vec(Self::create_vec3_data(gltf_data, accessor)?);
                    if normals.len() != num_vertices {
                        return Err(Error::InvalidFormat(format!(
                            "Number of positions {} do not match number of normals {}",
                            num_vertices,
                            normals.len()
                        )));
                    }

                    vertices.set_normals(normals)?;
                }
                None => {}
            }

            let mesh = Mesh::new(vertices, mesh_primitives)?;
            shape.add_part(ShapePart::new(Rc::new(mesh), material));
        }

        Ok(shape)
    }

    /// Tries to find an accessor with the specified semantic.
    ///
    /// # Arguments
    /// * `attributes` - The attributes to search within.
    /// * `semantic` - The semantic to search for.
    fn find_accessor_by_semantic(attributes: Attributes, semantic: Semantic) -> Option<Accessor> {
        let mut attributes = attributes;
        match attributes.find(|(s, _a)| *s == semantic) {
            Some((_, a)) => Some(a),
            None => None,
        }
    }

    /// Creates the index data for the given GLTF mesh.
    ///
    /// # Arguments
    /// * `gltf_data` - The overall loaded GLTF data.
    /// * `primitive` - The mesh for which the index data will be created.
    fn create_index_data(
        gltf_data: &GLTFData,
        primitive: GLTFPrimitive,
    ) -> Result<IndexData, Error> {
        match primitive.indices() {
            Some(accessor) => {
                if accessor.dimensions() != Dimensions::Scalar {
                    return Err(Error::InvalidFormat(format!(
                        "Dimension for indices must be scalar, but is {:?}",
                        accessor.dimensions()
                    )));
                }

                let data_type = accessor.data_type();
                if !Self::is_data_type_integer(data_type) {
                    return Err(Error::InvalidFormat(format!(
                        "Data Type for indices must be an integer, but is {:?}",
                        data_type
                    )));
                }

                match accessor.view() {
                    None => {
                        return Err(Error::InvalidFormat(format!(
                            "Indices are missing corresponding buffer view"
                        )));
                    }
                    Some(view) => {
                        let indices = match accessor.data_type() {
                            GLTFDataType::U8 => {
                                Self::extract_indices::<u8>(gltf_data, accessor, view)
                            }
                            GLTFDataType::U16 => {
                                Self::extract_indices::<u16>(gltf_data, accessor, view)
                            }
                            GLTFDataType::U32 => {
                                Self::extract_indices::<u32>(gltf_data, accessor, view)
                            }
                            GLTFDataType::I8 => {
                                Self::extract_indices::<i8>(gltf_data, accessor, view)
                            }
                            GLTFDataType::I16 => {
                                Self::extract_indices::<i16>(gltf_data, accessor, view)
                            }
                            _ => {
                                return Err(Error::InvalidFormat(format!(
                                    "Invalid data type for indices {:?}",
                                    accessor.data_type()
                                )));
                            }
                        }?;

                        let index_data = IndexData::Indices(indices);

                        Ok(index_data)
                    }
                }
            }
            None => {
                let num_vertices = Self::determine_num_vertices(primitive.attributes())?;
                let index_data = IndexData::NonIndexed(num_vertices);

                Ok(index_data)
            }
        }
    }

    /// Extracts the indices from the given accessor and related buffer view.
    ///
    /// # Arguments
    /// * `gltf_data` - The overall loaded GLTF data.
    /// * `accessor` - The accessor used for extracting the model data.
    /// * `view` - The buffer that defines the view onto the data.
    fn extract_indices<T>(
        gltf_data: &GLTFData,
        accessor: Accessor,
        view: View,
    ) -> Result<Vec<u32>, Error>
    where
        T: Sized + Copy + TryInto<u32> + Display,
    {
        let buffer_index = view.buffer().index();
        if buffer_index >= gltf_data.blobs.len() {
            return Err(Error::InvalidFormat(format!(
                "Invalid buffer index {}",
                buffer_index
            )));
        }

        let buffer = gltf_data.blobs[buffer_index].as_ref();

        let it = AccessorIterator::<T>::new(buffer, view, accessor.clone());
        let mut indices = Vec::with_capacity(accessor.count());
        for index in it {
            let index: u32 = match index.try_into() {
                Ok(index) => index,
                Err(_) => {
                    return Err(Error::InvalidFormat(format!("Invalid index {}", index)));
                }
            };

            indices.push(index);
        }

        Ok(indices)
    }

    /// Creates vector 3 data from the given accessor.
    ///
    /// # Arguments
    /// * `gltf_data` - The overall loaded GLTF data.
    /// * `accessor` - The accessor that is used for the data.
    fn create_vec3_data(gltf_data: &GLTFData, accessor: Accessor) -> Result<Vec<Vec3>, Error> {
        if accessor.dimensions().multiplicity() != 3 {
            return Err(Error::InvalidFormat(format!(
                "Dimension is not 3, but {}",
                accessor.dimensions().multiplicity()
            )));
        }

        let view = match accessor.view() {
            Some(view) => view,
            None => {
                return Err(Error::InvalidFormat(format!(
                    "Missing buffer view reference"
                )));
            }
        };

        let vecs = match accessor.data_type() {
            GLTFDataType::U8 => Self::extract_vecs3::<u8>(gltf_data, accessor, view),
            GLTFDataType::U16 => Self::extract_vecs3::<u16>(gltf_data, accessor, view),
            GLTFDataType::U32 => Self::extract_vecs3::<u32>(gltf_data, accessor, view),
            GLTFDataType::I8 => Self::extract_vecs3::<i8>(gltf_data, accessor, view),
            GLTFDataType::I16 => Self::extract_vecs3::<i16>(gltf_data, accessor, view),
            GLTFDataType::F32 => Self::extract_vecs3::<f32>(gltf_data, accessor, view),
        }?;

        Ok(vecs)
    }

    /// Extracts the vector 3 from the given accessor and related buffer view.
    ///
    /// # Arguments
    /// * `gltf_data` - The overall loaded GLTF data.
    /// * `accessor` - The accessor used for extracting the data.
    /// * `view` - The buffer that defines the view onto the data.
    fn extract_vecs3<T: ComponentTrait>(
        gltf_data: &GLTFData,
        accessor: Accessor,
        view: View,
    ) -> Result<Vec<Vec3>, Error>
    where
        T: Sized + Copy + Display,
    {
        let normalize = accessor.normalized();

        let buffer_index = view.buffer().index();
        if buffer_index >= gltf_data.blobs.len() {
            return Err(Error::InvalidFormat(format!(
                "Invalid buffer index {}",
                buffer_index
            )));
        }

        let buffer = gltf_data.blobs[buffer_index].as_ref();

        let mut vecs: Vec<Vec3> = Vec::with_capacity(accessor.count());
        let it = AccessorIterator::<[T; 3]>::new(buffer, view, accessor.clone());

        for x in it {
            let v = Vec3::new(
                x[0].to_f32(normalize),
                x[1].to_f32(normalize),
                x[2].to_f32(normalize),
            );

            vecs.push(v);
        }

        if vecs.len() != accessor.count() {
            return Err(Error::InvalidFormat(format!(
                "Read {} values, but should have been {}",
                vecs.len(),
                accessor.count() * 3
            )));
        }

        Ok(vecs)
    }

    /// Translates the given GLTF mode into a primitive type.
    fn translate_primitive_mode(mode: Mode) -> PrimitiveType {
        match mode {
            Mode::Points => PrimitiveType::Point,
            Mode::Lines => PrimitiveType::Line,
            Mode::LineLoop => PrimitiveType::LineLoop,
            Mode::LineStrip => PrimitiveType::LineStrip,
            Mode::Triangles => PrimitiveType::Triangles,
            Mode::TriangleFan => PrimitiveType::TriangleFan,
            Mode::TriangleStrip => PrimitiveType::TriangleStrip,
        }
    }

    /// Returns if the given data type is an integer.
    ///
    /// # Arguments
    /// * `data_type` - The datatype to check.
    fn is_data_type_integer(data_type: GLTFDataType) -> bool {
        match data_type {
            GLTFDataType::I8 => true,
            GLTFDataType::U8 => true,
            GLTFDataType::I16 => true,
            GLTFDataType::U16 => true,
            GLTFDataType::U32 => true,
            _ => false,
        }
    }

    /// Tries to determine the number of vertices for the given attributes.
    ///
    /// # Arguments
    /// * `attributes` - The attributes whose total number will be determined.
    fn determine_num_vertices(attributes: Attributes) -> Result<usize, Error> {
        let mut attributes = attributes;
        match attributes.find(|(s, _)| *s == Semantic::Positions) {
            Some((_, a)) => Ok(a.count()),
            None => Err(Error::InvalidFormat(format!(
                "Primitive attributes have no position"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use nalgebra_glm::cross;

    use crate::{loader::FileResource, structure::Point3D};

    use super::*;

    /// Helper function to find any shape by traversing through the nodes.
    /// Will stop as soon as it encounters the first shape.
    ///
    /// # Arguments
    /// * `node` - The node and its children to check.
    fn find_shape(node: &Node) -> Option<Rc<Shape>> {
        if !node.get_shapes().is_empty() {
            return Some(node.get_shapes()[0].clone());
        }

        for child in node.get_children() {
            match find_shape(child) {
                Some(shape) => return Some(shape),
                None => {}
            }
        }

        None
    }

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

            let n = cross(&a, &b);

            let area = nalgebra_glm::l2_norm(&n) * 0.5f32;
            total_area += area;
        }

        total_area
    }

    fn test_if_it_is_a_box(cad_data: &CADData) {
        let shape = find_shape(cad_data.get_root_node()).unwrap();
        assert_eq!(shape.get_parts().len(), 1);
        let part = &shape.get_parts()[0];
        let mesh = part.get_mesh();

        assert_eq!(mesh.get_vertices().len(), 24);

        let (min_bb, max_bb) = compute_bbox(&mesh.get_vertices().get_positions());
        assert_eq!(min_bb, Vec3::new(-0.5, -0.5, -0.5));
        assert_eq!(max_bb, Vec3::new(0.5, 0.5, 0.5));

        let indices = mesh
            .get_primitives()
            .get_raw_index_data()
            .get_indices_ref()
            .unwrap();
        let area = compute_area(&mesh.get_vertices().get_positions(), indices);
        assert_eq!(area, 6.0);
    }

    #[test]
    fn test_gltf() {
        println!(env!("CARGO_MANIFEST_DIR"));

        let r = FileResource::new(
            PathBuf::from_str("src/loader/test_data/gltf/Box.gltf").unwrap(),
            "model/gltf+json",
        );

        let loader = LoaderGLTF::new();

        let cad_data = loader.read(&r).unwrap();
        test_if_it_is_a_box(&cad_data);
    }

    #[test]
    fn test_glb() {
        println!(env!("CARGO_MANIFEST_DIR"));

        let r = FileResource::new(
            PathBuf::from_str("src/loader/test_data/gltf/Box.glb").unwrap(),
            "model/gltf-binary",
        );

        let loader = LoaderGLTF::new();
        let cad_data = loader.read(&r).unwrap();
        test_if_it_is_a_box(&cad_data);
    }
}
