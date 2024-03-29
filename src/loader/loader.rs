use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use crate::{error::Error, structure::CADData};

use super::{FileResource, Options, OptionsDescriptor, Resource};

/// A map with file extensions and a respective list of registered associated mime-types
pub type ExtensionMap = BTreeMap<String, BTreeSet<String>>;

/// The trait for a registered loader.
pub trait Loader {
    /// Returns a reference onto the name of the loader.
    fn get_name(&self) -> &str;

    /// Returns the priority of the loader. The higher the priority the more likely that the
    /// loader is chosen if multiple loaders match for a given resource.
    fn get_priority(&self) -> u32;

    /// Returns a map of known file extensions with their respective mime types for the loader.
    fn get_extensions_mime_type_map(&self) -> ExtensionMap;

    /// Returns list of supported mime types for the formats the loader is supporting.
    fn get_mime_types(&self) -> Vec<String>;

    /// Returns a descriptor for the loader options if available.
    fn get_loader_options(&self) -> Option<OptionsDescriptor>;

    /// Reads the CAD data with provided options from the given resource. If something
    /// happens, the loader will return an error message.
    ///
    /// # Arguments
    /// * `resource` - The resource from which the loader will read the cad data.
    /// * `options` - Optionally, provide options loading resources.
    fn read_with_options(
        &self,
        resource: &dyn Resource,
        options: Option<Options>,
    ) -> Result<CADData, Error>;

    /// Reads the CAD data from the given resource. If something happens, the loader will return
    /// a error message.
    ///
    /// # Arguments
    /// * `reader` - The reader from which the loader will read the cad data.
    fn read(&self, resource: &dyn Resource) -> Result<CADData, Error> {
        self.read_with_options(resource, None)
    }

    /// Reads the CAD data from the given path. If something happens, the loader will return
    /// a error message.
    ///
    /// # Arguments
    /// * `p` - The path from which the loader will read the cad data.
    fn read_file(&self, p: &Path, mime_type: &str) -> Result<CADData, Error> {
        let f = FileResource::new(p.to_owned(), mime_type);
        self.read(&f)
    }

    /// Reads the CAD data from the given path with the provided options. If something
    /// happens, the loader will return a error message.
    ///
    /// # Arguments
    /// * `p` - The path from which the loader will read the cad data.
    /// * `options` - Optionally provide options loading resources.
    fn read_file_with_options(
        &self,
        p: &Path,
        mime_type: &str,
        options: Option<Options>,
    ) -> Result<CADData, Error> {
        let f = FileResource::new(p.to_owned(), mime_type);
        self.read_with_options(&f, options)
    }
}
