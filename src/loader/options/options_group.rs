use std::collections::HashMap;

use crate::Error;

use super::{OptionsDescriptor, Value};

/// A group of options, i.e., a list of options defined by the options descriptor.
#[derive(Clone)]
pub struct OptionsGroup {
    /// The descriptor for the options
    descriptor: OptionsDescriptor,

    /// The values for the options
    values: HashMap<String, Value>,
}

impl OptionsGroup {
    /// Creates a new options group with default values based on the provided options descriptor.
    ///
    /// # Arguments
    /// * `descriptor` - The descriptor for the options group.
    pub fn new(descriptor: OptionsDescriptor) -> Self {
        let mut values = HashMap::new();
        for d in descriptor.get_options() {
            let name = d.get_name().to_owned();
            let value = d.get_default();

            values.insert(name, value);
        }

        Self { descriptor, values }
    }

    /// Returns a reference onto the internal options descriptor.
    pub fn get_descriptor(&self) -> &OptionsDescriptor {
        &self.descriptor
    }

    /// Sets a new value for the specified option. Returns an error if the option is not defined or
    /// the value is invalid.
    ///
    /// # Arguments
    /// * `name` - The name of the option for which the value will be set.
    /// * `new_value` - The new value to set.
    pub fn set_value(&mut self, name: &str, new_value: Value) -> Result<(), Error> {
        match self.values.get_mut(name) {
            Some(dst_value) => {
                // Try finding the descriptor for the given option.
                // Note: Unwrap must work as the value with the same name exists in values map
                let option = self.descriptor.get_option(name).expect(&format!(
                    "Internal error: The descriptor for option {} must exists",
                    name
                ));

                // check if the given value is valid
                match option.check_value(&new_value) {
                    Ok(()) => {
                        *dst_value = new_value;
                        Ok(())
                    }
                    Err(err) => Err(Error::InvalidArgument(format!(
                        "Option is invalid due to {}",
                        err
                    ))),
                }
            }
            None => Err(Error::InvalidArgument(format!("Unknown option {}", name))),
        }
    }

    /// Returns a reference onto the value for the specified option.
    ///
    /// # Arguments
    /// * `name` - The name of the option for which the value reference will be returned.
    pub fn get_value(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }

    /// Returns a reference onto the internally stored values.
    pub fn get_values(&self) -> &HashMap<String, Value> {
        &self.values
    }
}

#[cfg(test)]
mod tests {
    use crate::loader::Descriptor;

    use super::*;

    #[test]
    fn test_set_value() {
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

        let options_descriptor = [
            Descriptor::new_with_validator("a".to_owned(), "".to_owned(), Value::from(44), checker)
                .unwrap(),
            Descriptor::new("b".to_owned(), "".to_owned(), Value::from(43)).unwrap(),
        ];
        let options_descriptor = OptionsDescriptor::new(options_descriptor.iter());

        let mut options = OptionsGroup::new(options_descriptor.clone());
        assert_eq!(options.get_values().len(), 2);

        // check default values
        assert_eq!(options.get_value("a"), Some(&Value::from(44)));
        assert_eq!(options.get_value("b"), Some(&Value::from(43)));

        // check for non-present variable c
        assert_eq!(options.get_value("c"), None);

        // change a to 23
        assert!(options.set_value("a", Value::from(23)).is_ok());
        assert_eq!(options.get_value("a"), Some(&Value::from(23)));

        // try to change a to invalid value
        assert!(options.set_value("a", Value::from(100)).is_err());
        assert_eq!(options.get_value("a"), Some(&Value::from(23)));

        // try to change unknown variable c
        assert!(options.set_value("c", Value::from(23)).is_err());
        assert_eq!(options.get_value("c"), None);
    }
}
