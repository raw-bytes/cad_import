use std::{
    fmt::Debug,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::Error;

use super::Resource;

/// A resource described by a file path.
pub struct FileResource {
    p: PathBuf,
    mime_type: String,
}

impl FileResource {
    /// Returns a new file resource with the given file path.
    ///
    /// # Arguments
    /// * `p` - The path to the file resource.
    pub fn new(p: PathBuf, mime_type: &str) -> Self {
        Self { p, mime_type: mime_type.to_owned() }
    }

    /// Returns the internally stored path as reference.
    pub fn get_path(&self) -> &Path {
        self.p.as_path()
    }
}

impl ToString for FileResource {
    fn to_string(&self) -> String {
        self.p.to_string_lossy().into_owned()
    }
}

impl Debug for FileResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.p)
    }
}

impl Resource for FileResource {
    fn get_mime_type(&self) -> String {
        self.mime_type.clone()
    }

    fn open(&self) -> Result<Box<dyn Read>, Error> {
        match File::open(&self.p) {
            Ok(f) => Ok(Box::new(f)),
            Err(err) => Err(Error::IO(format!(
                "Failed to open {} due to {}",
                self.p.to_string_lossy(),
                err
            ))),
        }
    }

    fn sub(&self, s: &str, mime_type: &str) -> Result<Box<dyn Resource>, Error> {
        let mut p = self.p.clone();
        p.pop();
        p.push(s);

        let mut p2 = PathBuf::new();
        for c in p.components() {
            p2.push(c);
        }

        Ok(Box::new(FileResource { p: p2, mime_type: mime_type.to_owned() }))
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use path_clean::clean;

    use super::*;

    #[test]
    fn test_sub() {
        let f = FileResource::new(PathBuf::from_str("/path/to/file.txt").unwrap(), "text/plain");
        assert_eq!(f.get_mime_type(), "text/plain");
        assert_eq!(f.to_string(), "/path/to/file.txt");

        let f2 = f.sub("./foobar.txt", "text/plain").unwrap();
        assert_eq!(f2.get_mime_type(), "text/plain");
        assert_eq!(f2.to_string(), "/path/to/foobar.txt");

        let f2 = f.sub("foobar.txt", "text/plain").unwrap();
        assert_eq!(f2.get_mime_type(), "text/plain");
        assert_eq!(f2.to_string(), "/path/to/foobar.txt");

        let f3 = f.sub("../fluff.txt", "text/plain").unwrap();
        assert_eq!(f.get_mime_type(), "text/plain");
        assert_eq!(clean(f3.to_string()).to_str().unwrap(), "/path/fluff.txt");
    }
}
