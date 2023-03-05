//! The loader module contains the loader manager, the loader trait and all implemented loaders.
pub mod loader_off;

mod loader;
mod manager;

pub use loader::Loader;
pub use manager::Manager;
