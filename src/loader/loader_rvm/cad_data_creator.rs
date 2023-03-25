use crate::structure::CADData;

use super::rvm_parser::RVMInterpreter;

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

impl RVMInterpreter for CADDataCreator {}
