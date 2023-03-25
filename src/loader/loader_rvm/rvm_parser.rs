use std::io::Read;

use crate::Error;

/// The RVM interpreter gets all the callbacks to process
pub trait RVMInterpreter {}

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
        todo!()
    }
}
