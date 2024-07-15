use std::{fmt::Display, io::Read, str::FromStr};

use crate::Error;

/// Identifier for the RVM format.
#[derive(Clone, Copy, Debug)]
pub struct Identifier {
    chars: [u8; 4],
}

impl Identifier {
    /// Returns a new empty identifier.
    pub fn new() -> Self {
        Self {
            chars: [0, 0, 0, 0],
        }
    }

    /// Returns true if the identifier is empty
    pub fn is_empty(&self) -> bool {
        self.chars[0] == 0
    }

    /// Returns true if the identifier is valid
    pub fn is_valid(&self) -> bool {
        let keywords = ["HEAD", "END", "MODL", "CNTB", "PRIM", "CNTE", "COLR"];

        for keyword in keywords.iter() {
            if self == keyword {
                return true;
            }
        }

        false
    }

    /// Reads an identifier from the given reader.
    ///
    /// # Errors
    /// * `r` - The reader from which the identifier is read.
    pub fn read<R: Read>(r: &mut R) -> Result<Identifier, Error> {
        let mut buf = [0u8; 12];
        let mut chars = [0u8; 4];

        r.read_exact(&mut buf)?;

        // read first three character
        for (chrs, chunk) in chars
            .iter_mut()
            .zip(buf.iter().as_slice().windows(4).step_by(4))
        {
            // the first three bytes of the current double word have to be zero
            if chunk[0] != 0 || chunk[1] != 0 || chunk[2] != 0 {
                return Ok(Identifier::default());
            }

            // extract character
            *chrs = chunk[3];
        }

        // check if we've reached the end
        if &chars[0..3] == "END".as_bytes() {
            Ok(Identifier::from(chars))
        } else {
            r.read_exact(&mut buf[0..4])?;

            if buf[0] != 0 || buf[1] != 0 || buf[2] != 0 {
                return Ok(Identifier::default());
            }

            chars[3] = buf[3];

            Ok(Identifier::from(chars))
        }
    }
}

impl FromStr for Identifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars: [u8; 4] = [0, 0, 0, 0];

        for (src, dst) in s.as_bytes().iter().zip(chars.iter_mut()) {
            *dst = *src;
        }

        Ok(Self { chars })
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.chars {
            if c == 0 {
                break;
            }

            if let Some(x) = char::from_u32(c as u32) {
                write!(f, "{}", x)?;
            }
        }

        Ok(())
    }
}

impl PartialEq<&str> for Identifier {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        if self.chars[3] == 0 {
            &self.chars[..3] == other.as_bytes()
        } else {
            self.chars == other.as_bytes()
        }
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self::new()
    }
}

impl From<[u8; 4]> for Identifier {
    fn from(chars: [u8; 4]) -> Self {
        Self { chars }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_basics() {
        let i0 = Identifier::from_str("MODL").unwrap();
        assert!(i0.is_valid());
        assert!(!i0.is_empty());

        let i0 = Identifier::from_str("MODX").unwrap();
        assert!(!i0.is_valid());
        assert!(!i0.is_empty());

        let i1: Identifier = Default::default();
        assert!(i1.is_empty());
    }
}
