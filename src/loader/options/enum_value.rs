use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    sync::Arc,
};

use itertools::Itertools;

use crate::Error;

/// The descriptor for an enum option.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumDescriptor {
    options: Vec<String>,
}

impl EnumDescriptor {
    /// Returns a reference onto the available options
    pub fn get_options(&self) -> &[String] {
        &self.options
    }
}

impl AsRef<[String]> for EnumDescriptor {
    fn as_ref(&self) -> &[String] {
        &self.options
    }
}

impl FromIterator<String> for EnumDescriptor {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut existing_options: HashSet<String> = HashSet::new();
        let mut options = Vec::new();

        for option in iter {
            if !existing_options.contains(&option) {
                existing_options.insert(option.clone());
                options.push(option);
            }
        }

        Self { options }
    }
}

impl<'a> FromIterator<&'a str> for EnumDescriptor {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut existing_options: HashSet<String> = HashSet::new();
        let mut options = Vec::new();

        for option in iter {
            if !existing_options.contains(option) {
                existing_options.insert(option.to_owned());
                options.push(option.to_owned());
            }
        }

        Self { options }
    }
}

/// A single enum value.
#[derive(Clone, PartialEq, Eq)]
pub struct EnumValue {
    /// Reference onto the enum value descriptor
    descriptor: Arc<EnumDescriptor>,

    /// The index of the current value
    value: Option<usize>,
}

impl EnumValue {
    /// Returns a new empty enum value.
    ///
    /// # Arguments
    /// * `descriptor` - The descriptor for the num.
    pub fn new(descriptor: Arc<EnumDescriptor>) -> Self {
        Self {
            descriptor,
            value: None,
        }
    }

    /// Returns true if the enum value is empty, i.e., has no option assigned.
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    /// Sets enum to the given option.
    ///
    /// # Arguments
    /// * `option` - The option to which the enum value will be set.
    pub fn set_value(&mut self, option: &str) -> Result<(), Error> {
        let options = self.descriptor.get_options();

        match options.iter().find_position(|s| *s == option) {
            Some((index, _)) => {
                self.value = Some(index);

                Ok(())
            }
            None => Err(Error::InvalidArgument(format!(
                "{} is not a valid option",
                option
            ))),
        }
    }

    /// Returns the option of the enum value.
    pub fn get_value(&self) -> Option<&str> {
        match self.value {
            Some(index) => Some(self.descriptor.options[index].as_str()),
            None => None,
        }
    }
}

impl Debug for EnumValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.get_value() {
            Some(v) => write!(f, "Value={}", v),
            None => write!(f, "NONE"),
        }
    }
}

impl Display for EnumValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.get_value() {
            Some(v) => write!(f, "{}", v),
            None => write!(f, "{{}}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_descriptor_from_iterator() {
        let enum_descriptor = EnumDescriptor::from_iter(["a", "c", "b"]);
        assert_eq!(enum_descriptor.get_options(), ["a", "c", "b"]);

        let enum_descriptor = EnumDescriptor::from_iter(["a", "c", "b", "a"]);
        assert_eq!(enum_descriptor.get_options(), ["a", "c", "b"]);
    }

    #[test]
    fn test_enum_value_set_option() {
        let enum_descriptor = EnumDescriptor::from_iter(["a", "c", "b"]);
        assert_eq!(enum_descriptor.get_options(), ["a", "c", "b"]);
        let enum_descriptor = Arc::new(enum_descriptor);

        let mut enum_value = EnumValue::new(enum_descriptor);
        assert!(enum_value.is_empty());

        enum_value.set_value("a").unwrap();
        assert_eq!(enum_value.get_value(), Some("a"));
        enum_value.set_value("b").unwrap();
        assert_eq!(enum_value.get_value(), Some("b"));
        enum_value.set_value("c").unwrap();
        assert_eq!(enum_value.get_value(), Some("c"));

        assert!(enum_value.set_value("d").is_err());
        assert_eq!(enum_value.get_value(), Some("c"));
    }
}
