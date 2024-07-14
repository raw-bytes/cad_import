use std::io::Read;

use crate::Error;

use byteorder::{BigEndian, ReadBytesExt};
use log::{debug, trace};
use nalgebra_glm::Vec3;

use super::{identifier::Identifier, identifier_reader::IdentifierReader};

/// The RVM interpreter gets all the callbacks to process
pub trait RVMInterpreter {
    /// Called when the RVM header has been read.
    ///
    /// # Arguments
    /// * `header` - The header of the RVM file.
    fn header(&mut self, header: RVMHeader);

    /// Called when the RVM model header has been read.
    ///
    /// # Arguments
    /// * `header` - The model header of the RVM file.
    fn model(&mut self, header: RVMModelHeader);
}

/// The RVM header contains the information from the RVM file.
#[derive(Clone, Debug)]
pub struct RVMHeader {
    pub version: u32,
    pub banner: String,
    pub file_note: String,
    pub date: String,
    pub user: String,
    pub encoding: String,
}

/// The RVM model header
#[derive(Clone, Debug)]
pub struct RVMModelHeader {
    pub version: u32,
    pub project_name: String,
    pub model_name: String,
}

/// Options for the RVM loader
struct RVMLoaderOptions {
    /// Determines if the associated attribute file should be loaded as well
    pub load_attributes: bool,
}

/// The RVM parser parses the rvm data and
pub struct RVMParser<'a, R: Read, Interpreter: RVMInterpreter> {
    /// The reader from which the parsers reads the input
    reader: R,

    /// The interpreter for sending back read events
    interpreter: &'a mut Interpreter,
}

impl<'a, R: Read, Interpreter: RVMInterpreter> RVMParser<'a, R, Interpreter> {
    /// Returns a new parser for the rvm format for the given reader. All read events are delegated
    /// to the provided interpreter.
    pub fn new(reader: R, interpreter: &'a mut Interpreter) -> Self {
        Self {
            reader,
            interpreter,
        }
    }

    /// Parses the content from the internal reader.
    pub fn parse(&mut self) -> Result<(), Error> {
        self.read_head()?;
        self.read_model()?;
        self.read_data()?;

        Ok(())
    }

    /// Reads the header of the RVM file.
    fn read_head(&mut self) -> Result<(), Error> {
        let identifier = self.read_until_valid_identifier()?;
        if identifier.is_empty() {
            return Err(Error::InvalidFormat(
                "Incorrect file format while reading identifier.".to_string(),
            ));
        }

        if identifier != "HEAD" {
            return Err(Error::InvalidFormat("File header not found.".to_string()));
        }

        self.skip_bytes(2)?; // garbage?

        let version = self.read_u32()?;
        debug!("RVM Version: {}", version);

        let banner = self.read_string()?;
        let file_note = self.read_string()?;
        let date = self.read_string()?;
        let user = self.read_string()?;

        debug!("Banner: {}", banner);
        debug!("File Note: {}", file_note);
        debug!("Date: {}", date);
        debug!("User: {}", user);

        let encoding = if version >= 2 {
            let e = self.read_string()?;
            if e == "Unicode UTF-8" {
                "UTF-8".to_string()
            } else {
                e
            }
        } else {
            "UTF-8".to_string()
        };

        debug!("Encoding: {}", encoding);

        let header = RVMHeader {
            version,
            banner,
            file_note,
            date,
            user,
            encoding,
        };
        self.interpreter.header(header);

        Ok(())
    }

    /// Reads the header of the RVM file.
    fn read_model(&mut self) -> Result<(), Error> {
        let id = self.read_until_valid_identifier()?;

        if id.is_empty() {
            return Err(Error::InvalidFormat(
                "Incorrect file format while reading identifier.".to_string(),
            ));
        }

        if id != "MODL" {
            return Err(Error::InvalidFormat("Model not found.".to_string()));
        }

        self.skip_bytes(2)?; // garbage?
        let version = self.read_u32()?;
        debug!("Model Format Version: {}", version);

        let project_name = self.read_string()?;
        let model_name = self.read_string()?;

        debug!("Project Name: {}", project_name);
        debug!("Model Name: {}", model_name);

        let model_header = RVMModelHeader {
            version,
            project_name,
            model_name,
        };

        self.interpreter.model(model_header);

        Ok(())
    }

    /// Reads the data from the RVM file.
    fn read_data(&mut self) -> Result<(), Error> {
        loop {
            // try to read the next identifier and  stop if no valid identifier has been found or
            // the end of the stream has been reached
            let id = self.read_until_valid_identifier()?;
            if id.is_empty() || id == "END" {
                break;
            }

            trace!("Identifier: {}", id);

            let size = self.read_u32()?;
            debug!("Size: {}", size);

            if id == "CNTB" {
                self.read_group()?;
            }
        }

        Ok(())
    }

    /// Reads a group node.
    fn read_group(&mut self) -> Result<(), Error> {
        self.skip_bytes(2)?; // garbage?

        let group_name = self.read_string()?;
        trace!("Group Name: {}", group_name);

        let translation: Vec3 = Vec3::from_row_slice(&self.read_f32_array::<3>()?);
        trace!("Translation: {:?}", translation);

        let material_id = self.read_f32()? as usize;
        trace!("Material ID: {}", material_id);

        unimplemented!("Read group");
    }

    /// Reads an array of 32-bit floating point numbers.
    #[inline]
    fn read_f32_array<const N: usize>(&mut self) -> Result<[f32; N], Error> {
        let mut array = [0.0; N];

        for x in array.iter_mut() {
            *x = self.read_f32()?;
        }

        Ok(array)
    }

    /// Reads a new 32-bit floating point number.
    #[inline]
    fn read_f32(&mut self) -> Result<f32, Error> {
        let x = self.reader.read_f32::<BigEndian>()?;
        Ok(x)
    }

    /// Reads a new 32-bit unsigned integer.
    #[inline]
    fn read_u32(&mut self) -> Result<u32, Error> {
        let x = self.reader.read_u32::<BigEndian>()?;
        Ok(x)
    }

    /// Reads a new string from the input stream.
    fn read_string(&mut self) -> Result<String, Error> {
        let size = (self.read_u32()? * 4) as usize;
        if size == 0 {
            Ok(String::new())
        } else {
            let mut chars = vec![0u8; size];
            self.reader.read_exact(&mut chars)?;

            Ok(String::from_utf8_lossy(&chars).to_string())
        }
    }

    /// Skips the number of specified double words.
    ///
    /// # Arguments
    /// * `num_dwords` - The number of dwords to skip, i.e.,
    ///                  num_dwords * 4 is the number of bytes to skip.
    fn skip_bytes(&mut self, num_dwords: u64) -> Result<(), Error> {
        let bytes_to_skip = num_dwords * 4;
        std::io::copy(
            &mut self.reader.by_ref().take(bytes_to_skip),
            &mut std::io::sink(),
        )?;

        Ok(())
    }

    /// Reads until either the end of the stream has been reached or a valid keyword has been found.
    /// The idea is to be able to jump over unknown blocks.
    ////
    /// # Arguments
    /// * `in` - The input stream
    /// * `outIdentifier` - Reference for returning the keyword if one has been found.
    fn read_until_valid_identifier(&mut self) -> Result<Identifier, Error> {
        let mut identifier_reader = IdentifierReader::new(&mut self.reader);
        identifier_reader.read()
    }
}

impl Default for RVMLoaderOptions {
    fn default() -> Self {
        Self {
            load_attributes: true,
        }
    }
}
