use std::{collections::HashSet, sync::Arc};

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
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumValue {
    /// Reference onto the enum value descriptor
    descriptor: Arc<EnumDescriptor>,

    /// The index of the current value
    value: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_iterator() {
        let enum_value = EnumDescriptor::from_iter(["a", "c", "b"]);
        assert_eq!(enum_value.get_options(), ["a", "c", "b"]);

        let enum_value = EnumDescriptor::from_iter(["a", "c", "b", "a"]);
        assert_eq!(enum_value.get_options(), ["a", "c", "b"]);
    }
}
