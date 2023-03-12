use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    rc::Rc,
};

use gltf::{
    buffer::Source, iter::Buffers, material::AlphaMode, mesh::Mode, Document, Gltf,
    Material as GLTFMaterial, Mesh as GLTFMesh,
};
use log::{debug, warn};
use nalgebra_glm::Vec3;

use crate::{
    loader::{Loader, Resource},
    structure::{CADData, Material, PhongMaterialData, PrimitiveType, Shape},
    Color, Error, RGB,
};

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

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, io::Cursor, path::PathBuf, str::FromStr};

    use crate::loader::FileResource;

    use super::*;

    pub struct FakeResource {
        data: &'static [u8],
        mime_type: String,
    }

    impl FakeResource {
        pub fn new(data: &'static [u8], mime_type: String) -> Self {
            Self { data, mime_type }
        }
    }

    impl ToString for FakeResource {
        fn to_string(&self) -> String {
            "".to_owned()
        }
    }

    impl Debug for FakeResource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }

    impl Resource for FakeResource {
        fn open(&self) -> Result<Box<dyn std::io::Read>, Error> {
            Ok(Box::new(Cursor::new(self.data)))
        }

        fn sub(&self, _s: &str, mime_type: &str) -> Result<Box<dyn Resource>, Error> {
            let s = Self {
                data: self.data,
                mime_type: mime_type.to_owned(),
            };
            Ok(Box::new(s))
        }

        fn get_mime_type(&self) -> String {
            self.mime_type.clone()
        }
    }

    #[test]
    fn test_gltf() {
        println!(env!("CARGO_MANIFEST_DIR"));

        // let s = include_str!("../test_data/gltf/Box.gltf");
        let r = FileResource::new(
            PathBuf::from_str("src/loader/test_data/gltf/Box.gltf").unwrap(),
            "model/gltf+json",
        );
        // let r = FakeResource::new(s.as_bytes(), "model/gltf+json".to_owned());

        let loader = LoaderGLTF::new();

        let cad_data = loader.read(&r).unwrap();
    }

    #[test]
    fn test_glb() {
        println!(env!("CARGO_MANIFEST_DIR"));

        // let s = include_str!("../test_data/gltf/Box.gltf");
        let r = FileResource::new(
            PathBuf::from_str("src/loader/test_data/gltf/Box.glb").unwrap(),
            "model/gltf-binary",
        );

        let loader = LoaderGLTF::new();
        let cad_data = loader.read(&r).unwrap();
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
    pub fn new() -> Self {
        Self {
            shape_map: HashMap::new(),
            material_map: HashMap::new(),
        }
    }

    pub fn create(self, gltf_data: &GLTFData) -> Result<CADData, Error> {
        let mut creator = self;

        creator.create_materials(gltf_data)?;
        creator.create_shapes(gltf_data)?;

        todo!()
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
        let base_color = material.pbr_metallic_roughness().base_color_factor();
        let diffuse_color = RGB(Vec3::from_row_slice(&base_color));

        let alpha_value = base_color[3];
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

    /// Creates the shapes from the GLTF meshes.
    fn create_shapes(&mut self, gltf_data: &GLTFData) -> Result<(), Error> {
        let meshes = gltf_data.document.meshes();

        for mesh in meshes {
            let mesh_index = mesh.index();
            let shape = Rc::new(self.create_shape(mesh, gltf_data)?);

            //     self.shape_map.insert(mesh_index, shape);
        }

        Ok(())
    }

    fn create_shape(&mut self, mesh: GLTFMesh, gltf_data: &GLTFData) -> Result<Shape, Error> {
        let primitives = mesh.primitives();

        for primitive in primitives {
            let material = self.get_material(primitive.material());
            let primitive_type = Self::translate_primitive_mode(primitive.mode());

            todo!()
        }

        todo!()
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
}
