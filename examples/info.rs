//! Small example to print a list of all existing loader.

use cad_import::loader::Manager;

fn main() {
    let manager = Manager::new();

    for loader in manager.get_loader_list().iter() {
        let extensions: String = loader.get_extensions().join(",");
        let mime_types: String = loader.get_mime_types().join(",");
        println!(
            "Loader {}: Extensions=[{}], Mime-Types=[{}] ",
            loader.get_name(),
            extensions,
            mime_types
        );
    }
}
