use super::{Length, Node};

/// The central in-memory data-structure for loaded CAD data.
pub struct CADData {
    /// The root node of the assembly structure of the cad data.
    root_node: Node,

    /// The length unit in which all spacial coordinates are defined
    length_unit: Length,
}

impl CADData {
    /// Creates and returns a new CAD data object.
    ///
    /// # Arguments
    /// * `root_node` - The root node of the assembly structure.
    pub fn new(root_node: Node) -> Self {
        Self {
            root_node,
            length_unit: Length::METER,
        }
    }

    /// Returns a reference onto the root node of the assembly structure.
    pub fn get_root_node(&self) -> &Node {
        &self.root_node
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
    use super::*;

    #[test]
    fn test_cad_data_unit() {
        let root_node = Node::new("Root".to_owned());
        let mut cad_data = CADData::new(root_node);

        assert_eq!(cad_data.get_length_unit(), Length::METER);

        cad_data.change_length_unit(Length::INCH);
        assert_eq!(cad_data.get_length_unit(), Length::INCH);
    }
}
