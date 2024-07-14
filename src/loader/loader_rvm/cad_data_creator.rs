use nalgebra_glm::Vec3;

use crate::structure::CADData;

use super::rvm_parser::{RVMHeader, RVMInterpreter, RVMModelHeader};

/// The CAD creator creates the cad data structure based on the provided read events.
pub struct CADDataCreator {}

impl CADDataCreator {
    pub fn new() -> Self {
        Self {}
    }

    /// Transforms the cad data creator to an cad data object.
    pub fn to_cad_data(self) -> CADData {
        todo!()
    }
}

impl RVMInterpreter for CADDataCreator {
    fn header(&mut self, header: RVMHeader) {
        println!("Header: {:?}", header);
    }

    fn model(&mut self, header: RVMModelHeader) {
        println!("Model: {:?}", header);
    }

    fn begin_group(&mut self, group_name: String, translation: Vec3, material_id: usize) {
        println!(
            "Group: {:?}, {:?}, {:?}",
            group_name, translation, material_id
        );
    }

    fn end_group(&mut self) {
        println!("End group");
    }
}
