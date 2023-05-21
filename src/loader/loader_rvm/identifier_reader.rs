use std::io::Read;

use crate::Error;

use super::identifier::Identifier;

/// The identifier reader reads until a known identifier has been found.
pub struct IdentifierReader<'a, R: Read> {
    buffer: [u8; 16],
    chars: [u8; 4],
    num_bytes: usize,
    reader: &'a mut R,
}

impl<'a, R: Read> IdentifierReader<'a, R> {
    /// Returns an empty identifier reader
    pub fn new(reader: &'a mut R) -> Self {
        Self {
            buffer: [0u8; 16],
            chars: [0u8; 4],
            num_bytes: 0,
            reader,
        }
    }

    /// Reads until an identifier has been found.
    pub fn read(&mut self) -> Result<Identifier, Error> {
        loop {
            self.read_bytes_until(12)?;

            // try to load the first three characters and stop if this fails
            if !self.read_first_three_chars() {
                self.remove_first_byte();
                continue;
            }

            // check if we got the end identifier
            if &self.chars[..3] == "END".as_bytes() {
                self.chars[3] = 0;
                return Ok(Identifier::from(self.chars));
            }

            // check if we can read the fourth character
            if !self.read_last_char()? {
                self.remove_first_byte();
                continue;
            }

            // create identifier and check if it is valid
            let out_identifier = Identifier::from(self.chars);
            if out_identifier.is_valid() {
                return Ok(out_identifier);
            }

            // didn't work, so we throw away the first byte and continue
            self.remove_first_byte();
        }
    }

    /// Reads bytes until the specified number of bytes is in the buffer.
    ///
    /// # Arguments
    /// * `num_bytes` - The number of bytes to fill up the buffer
    fn read_bytes_until(&mut self, num_bytes: usize) -> Result<(), Error> {
        debug_assert!(num_bytes <= 16);

        if num_bytes > self.num_bytes {
            self.reader
                .read_exact(&mut self.buffer[self.num_bytes..num_bytes])?;
            self.num_bytes = num_bytes;
        }

        Ok(())
    }

    // Remove first byte from the buffer and shift all read bytes to left
    fn remove_first_byte(&mut self) {
        debug_assert!(self.num_bytes > 0);

        self.buffer.rotate_left(1);
        self.num_bytes -= 1;
    }

    /// Tries to read the first identifier character and returns false if the first three
    /// characters where invalid.
    fn read_first_three_chars(&mut self) -> bool {
        for (dst, chunk) in self
            .chars
            .iter_mut()
            .zip(self.buffer.iter().as_slice().windows(4).step_by(4))
        {
            // the first three bytes of the current double word have to be zero
            if chunk[0] != 0 || chunk[1] != 0 || chunk[2] != 0 {
                return false;
            }

            *dst = chunk[3];
        }

        true
    }

    /// Reads the last character from the reader and returns true if the character is valid.
    fn read_last_char(&mut self) -> Result<bool, Error> {
        // Check that the first 3 bytes are zero.
        // Here, we are a little bit more careful to read as few bytes as needed
        for i in 0..3usize {
            // check that the buffer is large enough
            self.read_bytes_until(13 + i)?;

            // stop reading if an invalid character is encountered
            if self.buffer[12 + i] != 0 {
                return Ok(false);
            }
        }

        // finally, read last byte
        self.read_bytes_until(16)?;
        self.chars[3] = self.buffer[15];

        Ok(true)
    }
}
