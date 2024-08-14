use super::tree::Tree;
use crate::Length;

/// The central in-memory data-structure for loaded CAD data.
pub struct CADData {
    /// The assembly structure of the cad data.
    tree: Tree,

    /// The length unit in which all spacial coordinates are defined
    length_unit: Length,
}

impl CADData {
    /// Creates and returns a new CAD data object.
    ///
    /// # Arguments
    /// * `tree` - The assembly structure of the cad data.
    pub fn new(tree: Tree) -> Self {
        Self {
            tree,
            length_unit: Length::METER,
        }
    }

    /// Returns a reference onto the assembly structure of the cad data.
    pub fn get_assembly(&self) -> &Tree {
        &self.tree
    }

    /// Returns the unit for lengths.
    pub fn get_length_unit(&self) -> Length {
        self.length_unit
    }

    /// Changes the unit for lengths to new one
    ///
    /// # Arguments
    /// * `length_unit` - The new unit for lengths
    pub fn change_length_unit(&mut self, length_unit: Length) {
        self.length_unit = length_unit;
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::tree::Tree;

    use super::*;

    #[test]
    fn test_cad_data_unit() {
        let mut tree = Tree::new();
        tree.create_node("Root".to_owned());

        let mut cad_data = CADData::new(tree);

        assert_eq!(cad_data.get_length_unit(), Length::METER);

        cad_data.change_length_unit(Length::INCH);
        assert_eq!(cad_data.get_length_unit(), Length::INCH);
    }
}
