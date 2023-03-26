use super::value::Value;

/// The descriptor specifies all properties of an option, e.g., name, acceptable inputs, ... etc.
pub struct Descriptor<ValidationChecker> {
    /// The name of the option
    name: String,

    /// The description of the meaning of the option.
    description: String,

    /// The default value of the option
    default_value: Value,

    /// An optional validation checker for option values.
    validation_checker: Option<ValidationChecker>,
}

impl<ValidationChecker> Descriptor<ValidationChecker>
where
    ValidationChecker: Fn(Value) -> Result<(), String>,
{
    /// Returns a new option descriptor.
    ///
    /// # Arguments
    /// * `name` - The name of the option.
    /// * `description` - The description of the meaning of the option.
    /// * `default_value` - The default value for the option.
    pub fn new(name: String, description: String, default_value: Value) -> Self {
        Self {
            name,
            description,
            default_value,
            validation_checker: None,
        }
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

