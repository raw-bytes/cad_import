//! The resource module defines the Resource trait and implements for it.
mod file_resource;
mod memory_resource;
mod resource;

pub use file_resource::FileResource;
pub use memory_resource::MemoryResource;
pub use resource::Resource;
