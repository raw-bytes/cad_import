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
    fn get_extensions(&self) -> Vec<String> {
        vec!["glb".to_owned(), "gltf".to_owned()]
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
