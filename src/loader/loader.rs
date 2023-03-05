use crate::{error::Error, structure::cad_data::CADData};

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
    fn read_file(&self, reader: &mut dyn std::io::Read) -> Result<CADData, Error>;
}
