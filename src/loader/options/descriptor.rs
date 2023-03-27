use std::collections::HashSet;

use crate::Error;

use super::value::Value;

/// The validation checker callback checks if the given option value is valid.
pub type ValidationChecker = fn(value: Value) -> Result<(), String>;

/// The descriptor specifies all properties of an option, e.g., name, acceptable inputs, ... etc.
pub struct Descriptor {
    /// The name of the option
    name: String,

    /// The description of the meaning of the option.
    description: String,

    /// The default value of the option
    default_value: Value,

    /// An optional validation checker for option values.
    validation_checker: Option<ValidationChecker>,
}

impl Descriptor {
    /// Returns a new option descriptor.
    ///
    /// # Arguments
    /// * `name` - The name of the option. May not be
    /// * `description` - The description of the meaning of the option.
    /// * `default_value` - The default value for the option.
    pub fn new(name: String, description: String, default_value: Value) -> Result<Self, Error> {
        if !name.is_empty() {
            return Err(Error::InvalidArgument(format!(
                "Option name may not be empty"
            )));
        }

        Ok(Self {
            name,
            description,
            default_value,
            validation_checker: None,
        })
    }

    /// Returns a new option descriptor with a validation checker.
    ///
    /// # Arguments
    /// * `name` - The name of the option.
    /// * `description` - The description of the meaning of the option.
    /// * `default_value` - The default value for the option.
    pub fn new_with_validator(
        name: String,
        description: String,
        default_value: Value,
        validation_checker: ValidationChecker,
    ) -> Self {
        Self {
            name,
            description,
            default_value,
            validation_checker: Some(validation_checker),
        }
    }
}

/// A description for loader options.
pub struct OptionsDescriptor {
    options: Vec<Descriptor>,
}
