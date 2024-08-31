use std::{
    fmt::{Debug, Display},
    rc::Rc,
    sync::Arc,
};

use nalgebra_glm::Mat4;

use crate::{
    structure::{MetaDataNode, Shape},
    ID,
};

use super::NodeId;

/// A single node in the assembly structure of the CAD data.
pub struct Node {
    id: NodeId,
    label: String,
    metadata: Option<Arc<MetaDataNode>>,
    transform: Option<Mat4>,
    shapes: Vec<Rc<Shape>>,
    children: Vec<NodeId>,
}

impl Node {
    /// Creates a new node with the given label.
    ///
    /// # Arguments
    /// * `label` - The label of the node.
    pub fn new(label: String, id: NodeId) -> Self {
        Self {
            id,
            label,
            metadata: None,
            transform: None,
            shapes: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Returns the id of the node
    pub fn get_id(&self) -> NodeId {
        self.id
    }

    /// Returns true if the node is a leaf node.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns a reference onto the label of the node.
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Returns the metadata node attached to this node.
    pub fn get_metadata(&self) -> Option<Arc<MetaDataNode>> {
        self.metadata.clone()
    }

    /// Sets the metadata of this node.
    ///
    /// # Arguments
    /// * `metadata` - The metadata to set for this node.
    pub fn set_metadata(&mut self, metadata: Arc<MetaDataNode>) {
        self.metadata = Some(metadata);
    }

    /// Adds the given node as child.
    ///
    /// # Arguments
    /// * `child` - The node id to add as child.
    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    /// Returns a reference onto the children of this node
    pub fn get_children_node_ids(&self) -> &[NodeId] {
        &self.children
    }

    /// Attaches a shape to the current node.
    ///
    /// # Arguments
    /// * `shape` - The shape to attach.
    pub fn attach_shape(&mut self, shape: Rc<Shape>) {
        self.shapes.push(shape);
    }

    /// Returns a reference onto the internal stored shapes.
    pub fn get_shapes(&self) -> &[Rc<Shape>] {
        &self.shapes
    }

    /// Sets the given transformation for the node.
    ///
    /// # Arguments
    /// * `transform` - The transformation to set.
    pub fn set_transform(&mut self, transform: Mat4) {
        self.transform = Some(transform)
    }

    /// Returns the local transformation of the node.
    pub fn get_transform(&self) -> Option<Mat4> {
        self.transform
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node({})[label={}, #Children={}, #Shapes={}]",
            self.id,
            self.label,
            self.children.len(),
            self.shapes.len()
        )
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let shape_ids: Vec<ID> = self.shapes.iter().map(|s| s.get_id()).collect();

        write!(
            f,
            "Node({})[label={}, #Children={:?}, #Shapes={:?}]",
            self.id, self.label, self.children, shape_ids
        )
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

#[cfg(test)]
mod tests {
    use crate::structure::tree::Tree;

    #[test]
    fn test_node_basic() {
        let mut tree = Tree::new();

        let node_id0 = tree.create_node("node".to_owned());
        let node_id1: usize = tree.create_node("node".to_owned());

        assert!(tree.get_node(node_id0).unwrap().is_leaf());
        assert!(tree.get_node(node_id1).unwrap().is_leaf());

        assert_ne!(node_id0, node_id1);

        tree.get_node_mut(node_id0).unwrap().add_child(node_id1);

        let node0 = tree.get_node(node_id0).unwrap();
        assert!(!node0.is_leaf());
        assert_eq!(node0.get_children_node_ids().len(), 1);
        let node1 = tree.get_node(node0.get_children_node_ids()[0]).unwrap();
        assert_eq!(node1.get_id(), node_id1);
    }
}
