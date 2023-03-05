//! The loader module contains the loader manager, the loader trait and all implemented loaders.
pub mod loader_gltf;
pub mod loader_off;

mod loader;
mod manager;
mod resource;

pub use loader::{Loader, ExtensionMap};
pub use manager::Manager;
pub use resource::*;
