//! The loader module contains the loader manager, the loader trait and all implemented loaders.
pub mod loader_gltf;
pub mod loader_off;

mod loader;
mod manager;
mod options;
mod resource;

pub use loader::{ExtensionMap, Loader};
pub use manager::Manager;
pub use options::*;
pub use resource::*;
