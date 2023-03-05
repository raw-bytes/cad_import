use std::path::Path;

use crate::{error::Error, structure::CADData};

use super::{FileResource, Resource};

/// The trait for a registered loader.
pub trait Loader {
    /// Returns a reference onto the name of the loader.
    fn get_name(&self) -> &str;

    /// Returns the priority of the loader. The higher the priority the more likely that the
    /// loader is chosen if multiple loaders match for a given resource.
    fn get_priority(&self) -> u32;

    /// Returns list of supported file extensions in lower case.
    fn get_extensions(&self) -> Vec<String>;

    /// Returns list of supported mime types for the format.
    fn get_mime_types(&self) -> Vec<String>;

    /// Reads the CAD data from the given reader. If something happens, the loader will return
    /// a error message.
    ///
    /// # Arguments
    /// * `reader` - The reader from which the loader will read the cad data.
    fn read(&self, resource: &dyn Resource) -> Result<CADData, Error>;

    /// Reads the CAD data from the given path. If something happens, the loader will return
    /// a error message.
    ///
    /// # Arguments
    /// * `p` - The path from which the loader will read the cad data.
    fn read_file(&self, p: &Path) -> Result<CADData, Error> {
        let f = FileResource::new(p.to_owned());
        self.read(&f)
    }
}
