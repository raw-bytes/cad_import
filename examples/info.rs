//! Small example to print a list of all existing loader.

use cad_import::loader::Manager;

fn main() {
    let manager = Manager::new();

    for loader in manager.get_loader_list().iter() {
        let extensions: Vec<String> = loader.get_extensions_mime_type_map().keys().map(|s| s.clone()).collect();
        let extensions: String = extensions.join(",");
        let mime_types: String = loader.get_mime_types().join(",");
        println!(
            "Loader {}: Extensions=[{}], Mime-Types=[{}] ",
            loader.get_name(),
            extensions,
            mime_types
        );
    }
}
