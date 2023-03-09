use std::collections::{BTreeMap, BTreeSet};

use gltf::{buffer::Source, iter::Buffers, Document, Gltf};

use crate::{
    loader::{Loader, Resource},
    structure::CADData,
    Error,
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
        todo!()
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
}
