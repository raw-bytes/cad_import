use log::{debug, trace};
use nalgebra_glm::{Mat4, Vec3};

use crate::{
    structure::{CADData, NodeId, Tree},
    Length,
};

use super::rvm_parser::{RVMHeader, RVMInterpreter, RVMModelHeader};

/// The CAD creator creates the cad data structure based on the provided read events.
pub struct CADDataCreator {
    node_stack: Vec<NodeId>,
    tree: Tree,
}

impl CADDataCreator {
    pub fn new() -> Self {
        Self {
            node_stack: Vec::new(),
            tree: Tree::new(),
        }
    }

    /// Transforms the cad data creator to an cad data object.
    pub fn to_cad_data(self) -> CADData {
        let mut cad_data = CADData::new(self.tree);
        cad_data.change_length_unit(Length::MILLIMETER);

        cad_data
    }
}

impl RVMInterpreter for CADDataCreator {
    fn header(&mut self, header: RVMHeader) {
        debug!("Header: {:?}", header);
    }

    fn model(&mut self, header: RVMModelHeader) {
        debug!("Model: {:?}", header);

        let root_id = self.tree.create_node(header.model_name.clone());
        self.node_stack.push(root_id);
    }

    fn primitive(&mut self, primitive: super::primitive::Primitive, matrix: [f32; 12]) {
        println!("Primitive: {:?}, {:?}", primitive, matrix);
    }

    fn begin_group(&mut self, group_name: String, translation: Vec3, material_id: usize) {
        trace!(
            "Group: {:?}, {:?}, {:?}",
            group_name,
            translation,
            material_id
        );

        let parent_id = *self.node_stack.last().expect("No parent node found");
        let new_id = self.tree.create_node_with_parent(group_name, parent_id);
        let node = self.tree.get_node_mut(new_id).unwrap();

        let translation_mat: Mat4 = nalgebra_glm::translation(&translation);
        node.set_transform(translation_mat);

        self.node_stack.push(new_id);
    }

    fn end_group(&mut self) {
        assert!(self.node_stack.len() > 1);
        self.node_stack.pop();
        trace!("End group");
    }
}
