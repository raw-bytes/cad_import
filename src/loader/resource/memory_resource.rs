use std::{fmt::Debug, io::Cursor};

use crate::Error;

use super::Resource;

/// A simplified resource to a memory blob.
pub struct MemoryResource {
    data: &'static [u8],
    mime_type: String,
}

impl MemoryResource {
    /// Creates a new memory resource from the given memory reference and mime type.
    pub fn new(data: &'static [u8], mime_type: String) -> Self {
        Self { data, mime_type }
    }
}

impl ToString for MemoryResource {
    fn to_string(&self) -> String {
        format!("memory-resource [Mime-Type={}]", self.mime_type)
    }
}

impl Debug for MemoryResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "memory-resource [Mime-Type={}, Size={}]",
            self.mime_type,
            self.data.len()
        )
    }
}

impl Resource for MemoryResource {
    fn open(&self) -> Result<Box<dyn std::io::Read>, Error> {
        Ok(Box::new(Cursor::new(self.data)))
    }

    fn sub(&self, _s: &str, _m: &str) -> Result<Box<dyn Resource>, Error> {
        let s = Self {
            data: self.data,
            mime_type: self.mime_type.clone(),
        };
        Ok(Box::new(s))
    }

    fn get_mime_type(&self) -> String {
        self.mime_type.clone()
    }
}
