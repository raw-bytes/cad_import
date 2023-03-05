use std::collections::{BTreeMap, BTreeSet};
use crate::{loader::{Loader, Resource}, structure::CADData, Error};

/// A loader for GLTF 2.0
/// Specification: See `<https://www.khronos.org/gltf/>`
pub struct LoaderGLTF {}

impl LoaderGLTF {
    pub fn new() -> Self {
        Self {}
    }
}

impl Loader for LoaderGLTF {
    fn get_extensions_mime_type_map(&self) -> crate::loader::ExtensionMap {
        let mut ext_map = BTreeMap::new();

        ext_map.insert("gltf".to_owned(), BTreeSet::from(["model/gltf+json".to_owned()]));
        ext_map.insert("glb".to_owned(), BTreeSet::from(["model/gltf-binary".to_owned()]));

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
        todo!()
    }
}

#[cfg(test)]
mod tests {}
