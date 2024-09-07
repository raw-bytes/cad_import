use lazy_static::lazy_static;

use crate::{Angle, Error, Length};

use super::{Descriptor, OptionsDescriptor, OptionsGroup, Value};

lazy_static! {
    /// The options descriptor for the general options
    static ref GENERAL_OPTIONS_DESCRIPTOR: OptionsDescriptor = {
        let options = [Descriptor::new_with_validator(
            "link_depth".to_owned(),
            "Determines the depth of following links to resolve them.".to_owned(),
            super::Value::Integer(0),
            |value| match value {
                Value::Integer(x) => {
                    if *x < 0 {
                        Err(format!(
                            "Invalid value. Value must be a non-negative integer number, but is {}",
                            *x
                        ))
                    } else {
                        Ok(())
                    }
                }
                _ => Err("Invalid value. Value must be a non-negative integer number".to_string()),
            },
        )
        .unwrap()];

        OptionsDescriptor::new(options.iter())
    };
}

/// Options for tessellation
#[derive(Clone, Debug)]
pub struct TessellationOptions {
    /// The maximum deviation of the tessellated surface from the parametrically defined surface.
    pub max_sag: Length,

    /// The maximum length of a single edge in the tessellated surface.
    pub max_length: Option<Length>,

    /// The maximum angle between two adjacent edges in the tessellated surface in radians.
    pub max_angle: Option<Angle>,
}

impl Default for TessellationOptions {
    fn default() -> Self {
        Self {
            max_sag: Length::new(0.001),
            max_length: None,
            max_angle: None,
        }
    }
}

/// General options that apply to all loaders
#[derive(Clone)]
pub struct GeneralOptions {
    /// Determines the depth of following links to resolve them.
    /// 0 means, never resolve any links, and 1 means only resolve the first stage of 0 links,
    /// ... etc.
    ///
    /// Default: 0
    pub resolving_link_depth: u32,

    /// The parameter for tessellating geometry.
    pub tessellation_options: TessellationOptions,
}

impl GeneralOptions {
    /// Returns a new general option object with default values.
    pub fn new() -> Self {
        Self {
            resolving_link_depth: 0,
            tessellation_options: TessellationOptions::default(),
        }
    }

    /// Returns the depth until which links are resolved.
    pub fn get_resolving_link_depth(&self) -> u32 {
        self.resolving_link_depth
    }

    /// Returns a descriptor for the general options.
    pub fn get_descriptor() -> OptionsDescriptor {
        GENERAL_OPTIONS_DESCRIPTOR.clone()
    }

    /// Sets the general options from the given values.
    ///
    /// # Arguments
    /// * `values` - Values used for setting the general options.
    pub fn set_values(&mut self, values: OptionsGroup) -> Result<(), Error> {
        if values.get_descriptor().get_id() != GENERAL_OPTIONS_DESCRIPTOR.get_id() {
            Err(Error::InvalidArgument(
                "Provided options do not match with general options".to_string(),
            ))
        } else {
            self.resolving_link_depth = values
                .get_value("link_depth")
                .unwrap()
                .to_integer()
                .unwrap() as u32;

            Ok(())
        }
    }
}

impl Default for GeneralOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_id() {
        let d0 = GeneralOptions::get_descriptor();
        let d1 = GeneralOptions::get_descriptor();
        let d2 = GeneralOptions::get_descriptor();

        assert_eq!(d0, d1);
        assert_eq!(d0.get_id(), d1.get_id());
        assert_eq!(d0, d2);
        assert_eq!(d0.get_id(), d2.get_id());
    }

    #[test]
    fn test_set_general_options_values() {
        let mut general_options = GeneralOptions::new();
        assert_eq!(general_options.get_resolving_link_depth(), 0);

        let d = GeneralOptions::get_descriptor();
        let mut values = OptionsGroup::new(d);

        values.set_value("link_depth", Value::from(42)).unwrap();
        general_options.set_values(values).unwrap();

        assert_eq!(general_options.get_resolving_link_depth(), 42);
    }
}
