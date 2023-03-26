//! The structure module contains the definition of the in-memory structure.
mod cad_data;
mod metadata;
mod shape;
mod tree;
mod units;

pub use cad_data::CADData;
pub use metadata::*;
pub use shape::*;
pub use tree::Node;
pub use units::*;
