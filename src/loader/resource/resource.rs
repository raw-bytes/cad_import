use std::{fmt::Debug, io::{Read, Cursor}};

use log::debug;

use crate::Error;

/// A resource is a descriptor to a specific resource, e.g., a filepath ot a URL.
/// It is possible to create sub-resources from a resource, e.g., '../foobar.txt'.
pub trait Resource: Debug + ToString {
    /// Returns the mimetype of the current resource.
    fn get_mime_type(&self) -> String;

    /// Creates a new resource to the specified sub-resource.
    ///
    /// # Arguments
    /// * `s` - The string which specified the sub-resource.
    ///         E.g., for filepaths this could be '../foobar.txt'.
    /// * `mime_type` - The mime type of the new sub resource.
    fn sub(&self, s: &str, mime_type: &str) -> Result<Box<dyn Resource>, Error>;

    /// Tries to open a reader to the currently specified resource.
    fn open(&self) -> Result<Box<dyn Read>, Error>;

    /// Opens a reader to the specified resource and copies the content to an U8 buffer.
    fn read_to_memory(&self) -> Result<Vec<u8>, Error> {
        let mut buffer: Vec<u8> = Vec::new();

            let mut writer = Cursor::new(&mut buffer);
            let mut reader = self.open()?;
            match std::io::copy(reader.as_mut(), &mut writer) {
                Err(err) => {
                    Err(Error::IO(format!("Failed copying {:?} to memory due to {}", self, err)))
                }
                Ok(l) => {
                    debug!("Copied {} bytes to memory", l);
                    Ok(buffer)
                }
            }
    }
}
