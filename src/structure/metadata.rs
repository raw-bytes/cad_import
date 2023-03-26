use std::{
    collections::BTreeMap,
    fmt::Display,
    sync::{Arc, Weak},
};

/// A single metadata value that is assigned to a metadata key
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum MetaDataValue {
    Integer(i64),
    Float(f64),
    Text(String),
}

impl From<f32> for MetaDataValue {
    fn from(x: f32) -> Self {
        Self::Float(x as f64)
    }
}

impl From<f64> for MetaDataValue {
    fn from(x: f64) -> Self {
        Self::Float(x)
    }
}

impl From<i32> for MetaDataValue {
    fn from(x: i32) -> Self {
        Self::Integer(x as i64)
    }
}

impl From<u32> for MetaDataValue {
    fn from(x: u32) -> Self {
        Self::Integer(x as i64)
    }
}

impl From<i64> for MetaDataValue {
    fn from(x: i64) -> Self {
        Self::Integer(x)
    }
}

impl From<String> for MetaDataValue {
    fn from(x: String) -> Self {
        Self::Text(x)
    }
}

impl From<&str> for MetaDataValue {
    fn from(x: &str) -> Self {
        Self::Text(x.to_owned())
    }
}

impl Display for MetaDataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaDataValue::Integer(x) => write!(f, "{}", x),
            MetaDataValue::Float(x) => write!(f, "{}", x),
            MetaDataValue::Text(x) => write!(f, "{}", x),
        }
    }
}

/// A metadata set consisting of key-value pair
pub type MetaDataSet = BTreeMap<String, MetaDataValue>;

/// A meta data node tha can be attached to a structure node.
///
/// Metadata is additional data that can be assigned to the nodes of the structure tree.
/// Metadata consists primarily of key-value pairs, e.g.,`tolerance = 1.5`, `unit = meter`, etc.
/// As multiple nodes can have common metadata, a hierarchical structure can be defined.
/// For example, two nodes n0 and n1 could both have the common metadata key-value pair
/// `unit = meter`.
/// Therefore, the nodes n0 and n1 could have respective metadata nodes m0 and m1 which have both
/// the common metadata node common as parent, like inheriting metadata.
/// The child metadata node always overrides the values of the parent node if keys are equivalent.
pub struct MetaDataNode {
    parent: Weak<MetaDataNode>,
    data: MetaDataSet,
}

impl MetaDataNode {
    /// Returns a new meta data node containing the given meta data set.
    ///
    /// # Arguments
    /// * `data` - The meta data to store in the node
    pub fn new(data: MetaDataSet) -> Self {
        Self {
            parent: Weak::new(),
            data,
        }
    }

    /// Returns a new meta data node containing the given meta data set with the given parent
    /// metadata node.
    ///
    /// # Arguments
    /// * `data` - The meta data to store in the node
    /// * `parent` - The parent meta data node.
    pub fn new_with_parent(data: MetaDataSet, parent: Arc<MetaDataNode>) -> Self {
        Self {
            parent: Arc::downgrade(&parent),
            data,
        }
    }

    /// Returns a reference onto the meta data stored in this node.
    pub fn get_metadata(&self) -> &MetaDataSet {
        &self.data
    }

    /// Returns the parent meta data node if available.
    pub fn get_parent(&self) -> Option<Arc<MetaDataNode>> {
        self.parent.upgrade()
    }

    /// Returns a list of all meta data including the parent node data.
    pub fn get_all_metadata(&self) -> MetaDataSet {
        let mut result = MetaDataSet::new();

        Self::traverse_metadata_node(&mut result, self.parent.clone());
        Self::add_to_metadata_set(&mut result, self.get_metadata());

        result
    }

    /// Adds the source metadata into the destination metadata.
    ///
    /// # Arguments
    /// * `dst_set` - The destination metadata set into which the metadata will be merged.
    /// * `src_set` - The source metadata to copy.
    fn add_to_metadata_set(dst_set: &mut MetaDataSet, src_set: &MetaDataSet) {
        dst_set.extend(src_set.iter().map(|(k, v)| (k.clone(), v.clone())));
    }

    /// Traverses and copies the metadata of all meta data nodes into the provided reference.
    /// Children override the meta data of their parents if the keys are equal.
    ///
    /// # Arguments
    /// * `dst_set` - The destination for copying the collected metadata.
    /// * `node` - The node to start traversing.
    fn traverse_metadata_node(dst_set: &mut MetaDataSet, node: Weak<MetaDataNode>) {
        match node.upgrade() {
            Some(node) => {
                // children potentially override the meta data of their parents if the keys are equal
                Self::traverse_metadata_node(dst_set, node.parent.clone());
                Self::add_to_metadata_set(dst_set, node.get_metadata());
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_from() {
        let m = MetaDataValue::from(32);
        assert_eq!(m, MetaDataValue::Integer(32));

        let m = MetaDataValue::from(32f32);
        assert_eq!(m, MetaDataValue::Float(32f64));

        let m = MetaDataValue::from("foobar");
        assert_eq!(m, MetaDataValue::Text("foobar".to_owned()));
    }

    #[test]
    fn test_metadata_all() {
        let mut parent_set = MetaDataSet::new();
        let mut child_set = MetaDataSet::new();

        parent_set.insert("project".to_owned(), MetaDataValue::from("foobar"));
        child_set.insert("node_id".to_owned(), MetaDataValue::from(42));

        let parent = Arc::new(MetaDataNode::new(parent_set));
        let child = Arc::new(MetaDataNode::new_with_parent(child_set, parent.clone()));

        let all_metadata = child.get_all_metadata();

        assert_eq!(all_metadata.len(), 2);
        assert_eq!(
            all_metadata.get("project"),
            Some(&MetaDataValue::from("foobar"))
        );
        assert_eq!(all_metadata.get("node_id"), Some(&MetaDataValue::from(42)));

        let all_metadata = child.get_metadata();
        assert_eq!(all_metadata.len(), 1);
        assert_eq!(all_metadata.get("node_id"), Some(&MetaDataValue::from(42)));

        let all_metadata = parent.get_metadata();
        assert_eq!(all_metadata.len(), 1);
        assert_eq!(
            all_metadata.get("project"),
            Some(&MetaDataValue::from("foobar"))
        );
    }

    #[test]
    fn test_metadata_all_override() {
        let mut set0 = MetaDataSet::new();
        let mut set1 = MetaDataSet::new();
        let mut set2 = MetaDataSet::new();

        set0.insert("project".to_owned(), MetaDataValue::from("foobar"));
        set0.insert("date".to_owned(), MetaDataValue::from("2023-03-26"));

        set1.insert("project".to_owned(), MetaDataValue::from("foobar2"));
        set1.insert("node_id".to_owned(), MetaDataValue::from(43));

        set2.insert("node_id".to_owned(), MetaDataValue::from(42));

        let common = Arc::new(MetaDataNode::new(set0));
        let node0 = Arc::new(MetaDataNode::new_with_parent(set1, common.clone()));
        let node1 = Arc::new(MetaDataNode::new_with_parent(set2, common.clone()));

        let all_metadata = node0.get_all_metadata();
        assert_eq!(all_metadata.len(), 3);
        assert_eq!(
            all_metadata.get("project"),
            Some(&MetaDataValue::from("foobar2"))
        );
        assert_eq!(all_metadata.get("node_id"), Some(&MetaDataValue::from(43)));
        assert_eq!(
            all_metadata.get("date"),
            Some(&MetaDataValue::from("2023-03-26"))
        );

        let all_metadata = node1.get_all_metadata();
        assert_eq!(all_metadata.len(), 3);
        assert_eq!(
            all_metadata.get("project"),
            Some(&MetaDataValue::from("foobar"))
        );
        assert_eq!(all_metadata.get("node_id"), Some(&MetaDataValue::from(42)));
        assert_eq!(
            all_metadata.get("date"),
            Some(&MetaDataValue::from("2023-03-26"))
        );
    }
}
