//! `cad_import` is a library for loading various 3D and CAD file formats.
//!
//! The library consists of a list of loaders registered in a loader manager and an unified
//! in-memory structure to store the loaded CAD/3D-data.
//!
//! # Example
//!
//! The following code prints a list of all registered loader to the console.
//! ```rust
//! use cad_import::loader::Manager;
//!
//! fn main() {
//!     let manager = Manager::new();
//!
//!     for loader in manager.get_loader_list().iter() {
//!         let extensions: String = loader.get_extensions().join(",");
//!         let mime_types: String = loader.get_mime_types().join(",");
//!         println!(
//!             "Loader {}: Extensions=[{}], Mime-Types=[{}] ",
//!             loader.get_name(),
//!             extensions,
//!             mime_types
//!         );
//!     }
//! }
//! ```
//!
//! In order to load a specific file see the following code
//! ```rust
//! use cad_import::loader::Manager;
//! use std::{fs::File, path::Path};
//! use std::env;
//!
//! fn main() {
//!     let manager = Manager::new();
//!     let mime_type = "model/vnd.off";
//!
//!     let loader = manager.get_loader_by_mime_type(mime_type).unwrap();
//!
//!     let args: Vec<String> = env::args().collect();
//!     let args = &args[1..];
//!
//!     if args.len() != 2 {
//!       println!("USAGE: <FILE-PATH>");
//!     } else {
//!         let file_path = Path::new(&args[1]);
//!
//!         let cad_data = loader.read_file(&file_path);
//!     }
//! }
//! ```
mod basic_types;
mod error;
pub mod loader;
pub mod structure;

pub use basic_types::*;
pub use error::Error;
