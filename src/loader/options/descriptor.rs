use std::collections::HashSet;

use crate::Error;

use super::value::Value;

/// The validation checker callback checks if the given option value is valid.
pub type ValidationChecker = fn(value: &Value) -> Result<(), String>;

#[derive(Clone)]
/// The descriptor specifies all properties of an option, e.g., name, acceptable inputs, ... etc.
pub struct Descriptor {
    /// The name of the option
    name: String,

    /// The description of the meaning of the option.
    description: String,

    /// The default value for the option
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
        if name.is_empty() {
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
    ) -> Result<Self, Error> {
        if name.is_empty() {
            return Err(Error::InvalidArgument(format!(
                "Option name may not be empty"
            )));
        }

        Ok(Self {
            name,
            description,
            default_value,
            validation_checker: Some(validation_checker),
        })
    }

    /// Returns a reference onto the name of the variable.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns a reference onto the description of the variable.
    pub fn get_description(&self) -> &str {
        &self.description
    }

    /// Returns the default value.
    pub fn get_default(&self) -> Value {
        self.default_value.clone()
    }

    /// Checks if the given value is valid w.r.t the internal validation checker.
    /// Returns an error string if the check fails.
    ///
    /// # Arguments
    /// * `value` - The value to check.
    pub fn check_value(&self, value: &Value) -> Result<(), String> {
        match self.validation_checker {
            Some(checker) => checker(value),
            None => Ok(()),
        }
    }
}

/// A description for a set of options.
pub struct OptionsDescriptor {
    options: Vec<Descriptor>,
}

impl OptionsDescriptor {
    pub fn new<'a, I>(options: I) -> Self
    where
        I: Iterator<Item = &'a Descriptor>,
    {
        let mut options_set: HashSet<String> = HashSet::new();
        let mut dst_options: Vec<Descriptor> = Vec::new();

        for option in options {
            if !options_set.contains(option.get_name()) {
                options_set.insert(option.get_name().to_owned());

                dst_options.push(option.clone());
            }
        }

        dst_options.reverse();

        Self {
            options: dst_options,
        }
    }

    /// Returns a reference onto the descriptors.
    pub fn get_options(&self) -> &[Descriptor] {
        &self.options
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_options() {
        let options_descriptions = [
            Descriptor::new("a".to_owned(), "".to_owned(), Value::from(44)).unwrap(),
            Descriptor::new("a".to_owned(), "".to_owned(), Value::from(43)).unwrap(),
        ];

        let d = OptionsDescriptor::new(options_descriptions.iter());
        assert_eq!(d.get_options().len(), 1);

        let option = d.get_options()[0].clone();
        assert_eq!(option.get_default(), Value::from(44));
    }

    #[test]
    fn test_validator() {
        let checker = |value: &Value| match value {
            Value::Integer(x) => {
                if *x < 100 {
                    Ok(())
                } else {
                    Err(format!("Value must be below 100"))
                }
            }
            Value::Float(x) => {
                if *x < 100.0 {
                    Ok(())
                } else {
                    Err(format!("Value must be below 100"))
                }
            }
            _ => Err(format!("Unsupported type")),
        };

        let options_descriptions = [
            Descriptor::new_with_validator("a".to_owned(), "".to_owned(), Value::from(44), checker)
                .unwrap(),
            Descriptor::new("a".to_owned(), "".to_owned(), Value::from(43)).unwrap(),
        ];

        let d = OptionsDescriptor::new(options_descriptions.iter());
        assert_eq!(d.get_options().len(), 1);

        let option = d.get_options()[0].clone();
        assert_eq!(option.get_default(), Value::from(44));

        assert_eq!(option.check_value(&Value::from(32)), Ok(()));
        assert_eq!(
            option.check_value(&Value::from(100)),
            Err(format!("Value must be below 100"))
        );

        assert_eq!(option.check_value(&Value::from(32.5)), Ok(()));
        assert_eq!(
            option.check_value(&Value::from(100.12)),
            Err(format!("Value must be below 100"))
        );
    }
}
