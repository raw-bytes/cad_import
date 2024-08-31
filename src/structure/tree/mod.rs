mod node;

pub use node::Node;

/// A unique identifier for a node in a tree.
pub type NodeId = usize;

/// A tree data-structure that holds a collection of nodes.
pub struct Tree {
    /// The pool of nodes in the tree.
    node_pool: Vec<Node>,

    /// The id of the root node.
    root_node_id: Option<NodeId>,
}

impl Tree {
    /// Creates a new empty tree.
    pub fn new() -> Self {
        Self {
            node_pool: Default::default(),
            root_node_id: None,
        }
    }

    /// Sets the root node of the tree.
    /// NOTE: This method will overwrite the current root node.
    ///
    /// # Arguments
    /// * `node_id` - The id of the node to set as the root node.
    pub fn set_root_node_id(&mut self, node_id: NodeId) {
        assert!(node_id < self.node_pool.len());
        self.root_node_id = Some(node_id);
    }

    /// Returns the id of the root node.
    pub fn get_root_node_id(&self) -> Option<NodeId> {
        self.root_node_id
    }

    /// Creates a new node with the given label and adds it to the tree.
    /// NOTE: The created node is not attached to any other node. If it is not the root node, it
    /// needs to be attached to another node.
    ///
    /// # Arguments
    /// * `label` - The label of the node.
    pub fn create_node(&mut self, label: String) -> NodeId {
        let new_node_id = self.node_pool.len();

        let new_node = Node::new(label, new_node_id);

        self.node_pool.push(new_node);

        if self.root_node_id.is_none() {
            self.root_node_id = Some(new_node_id);
        }

        new_node_id
    }

    /// Returns a reference to the root node.
    pub fn get_root_node(&self) -> Option<&Node> {
        let node = self.node_pool.first();
        node
    }

    /// Returns a mutable reference to the root node.
    pub fn get_root_node_mut(&mut self) -> Option<&mut Node> {
        let node = self.node_pool.first_mut();
        node
    }

    /// Returns a reference to the node with the given id.
    pub fn get_node(&self, node_id: NodeId) -> Option<&Node> {
        let node = self.node_pool.get(node_id);
        node
    }

    /// Returns a mutable reference to the node with the given id.
    pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut Node> {
        let node = self.node_pool.get_mut(node_id);
        node
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}
