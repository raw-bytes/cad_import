use super::{tree::node::Node};

/// A single loaded cad data asset
pub struct CADData {
    /// The root node of the assembly structure of the cad data.
    root_node: Node,
}

impl CADData {
    /// Creates and returns a new CAD data object.
    /// 
    /// # Arguments
    /// * `root_node` - The root node of the assembly structure.
    pub fn new(root_node: Node) -> Self {
        Self{
            root_node,
        }
    }

    /// Returns a reference onto the root node of the assembly structure.
    pub fn get_root_node(&self) -> &Node {
        &self.root_node
    }
}