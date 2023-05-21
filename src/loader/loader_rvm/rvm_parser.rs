use std::io::Read;

use crate::Error;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use log::debug;

use super::{identifier::Identifier, identifier_reader::IdentifierReader};

/// The RVM interpreter gets all the callbacks to process
pub trait RVMInterpreter {}

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

        todo!()
    }

    fn read_head(&mut self) -> Result<(), Error> {
        let identifier = self.read_until_valid_identifier()?;
        if identifier.is_empty() {
            return Err(Error::InvalidFormat(format!(
                "Incorrect file format while reading identifier."
            )));
        }

        if identifier != "HEAD" {
            return Err(Error::InvalidFormat(format!("File header not found.")));
        }

        self.skip_bytes(2)?; // garbage?

        let version = self.read_u32()?;
        debug!("RVM Version: {}", version);

        Ok(())
    }

    /// Reads a new 32-bit unsigned integer.
    #[inline]
    fn read_u32(&mut self) -> Result<u32, Error> {
        let x = self.reader.read_u32::<BigEndian>()?;
        Ok(x)
    }

    fn read_string(&mut self) -> Result<String, Error> {
        let size = self.read_u32()? * 4;
        if size == 0 {
            Ok(String::new())
        } else {
            let mut chars = vec![0u8; size];
        }
    }

    /// Skips the number of specified double words.
    ///
    /// # Arguments
    /// * `num_dwords` - The number of dwords to skip, i.e.,
    ///                  num_dwords * 4 is the number of bytes to skip.
    fn skip_bytes(&mut self, mut num_dwords: u64) -> Result<(), Error> {
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
