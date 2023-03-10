use std::{fmt::Debug, io::Read};

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
}
