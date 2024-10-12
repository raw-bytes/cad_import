use std::{env, fs::File, path::Path};

use cad_import::{
    exporter::X3DExporter,
    loader::{FileResource, Manager},
};
use log::LevelFilter;

/// This example loads the given file and exports as X3D to the specified path
fn usage() {
    println!("usage: to_x3d <file-path> <x3d-path> [<mime-type>]\n");
    println!("file-path: The path to the cad file to parse.");
    println!("x3d-path: The path to the resulting X3D file.");
    println!(
        "mime-type: Optional parameter to specify the mimetype. If not provided, the extension
           of the file is used"
    );
}

/// Initializes the program logging
///
/// # Arguments
/// * `filter` - The log level filter, i.e., the minimum log level to be logged.
fn initialize_logging(filter: LevelFilter) {
    pretty_env_logger::formatted_builder()
        .filter_level(filter)
        .init();

    // env_logger::builder()
    //     .format(|buf, record| {
    //         writeln!(
    //             buf,
    //             "{}:{} {} [{}] - {}",
    //             record.file().unwrap_or("unknown"),
    //             record.line().unwrap_or(0),
    //             chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
    //             record.level(),
    //             record.args()
    //         )
    //     })
    //     .filter_level(filter)
    //     .init();
}

fn determine_mime_types(
    manager: &Manager,
    input_file: &Path,
    mime_type: Option<&str>,
) -> Vec<String> {
    match mime_type {
        Some(m) => vec![m.to_owned()],
        None => match input_file.extension() {
            Some(ext) => match ext.to_str() {
                Some(ext) => manager.get_mime_types_for_extension(ext),
                None => {
                    eprintln!("Input file has invalid extension");
                    Vec::new()
                }
            },
            None => {
                eprintln!("Input file has no extension");
                Vec::new()
            }
        },
    }
}

fn run_program(input_file: &Path, x3d_file: &Path, mime_type: Option<&str>) -> bool {
    let manager = Manager::new();

    let mime_types = determine_mime_types(&manager, input_file, mime_type);
    if mime_types.is_empty() {
        eprintln!("Could not determine the mime type");
        return false;
    }

    let loader = manager.get_loader_by_mime_type(&mime_types[0]);
    let loader = match loader {
        None => {
            eprintln!("Cannot find loader");
            return false;
        }
        Some(loader) => loader,
    };

    println!("Reading file {:?}...", input_file);
    let file_resource = FileResource::new(input_file.to_owned(), &mime_types[0]);

    let cad_data = match loader.read(&file_resource) {
        Ok(cad_data) => cad_data,
        Err(err) => {
            eprintln!("Reading file {:?}...FAILED", input_file);
            eprintln!("Error: {}", err);
            return false;
        }
    };

    println!("Reading file {:?}...DONE", input_file);

    println!("Writing X3D {:?}...", x3d_file);
    let x3d_exporter = X3DExporter::new(&cad_data);
    let file = match File::create(x3d_file) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Writing X3D {:?}...FAILED", x3d_file);
            eprintln!("Error: {}", err);
            return false;
        }
    };

    match x3d_exporter.write(file) {
        Ok(()) => {
            println!("Writing X3D {:?}...DONE", x3d_file);
        }
        Err(err) => {
            eprintln!("Writing X3D {:?}...FAILED", x3d_file);
            eprintln!("Error: {}", err);
            return false;
        }
    }

    true
}

fn main() {
    initialize_logging(LevelFilter::Debug);

    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    // check if the number of arguments is invalid
    if args.len() < 2 {
        usage();
        std::process::exit(-1);
    } else if args.len() > 3 {
        eprintln!("Too many arguments!!!\n\n");
        usage();
        std::process::exit(-1);
    }

    // parse arguments
    let input_file = Path::new(&args[0]);
    let output_file = Path::new(&args[1]);
    let mime_type = if args.len() == 3 {
        Some(args[2].as_str())
    } else {
        None
    };

    if run_program(input_file, output_file, mime_type) {
        println!("FINISHED");
    } else {
        eprintln!("FAILED!!!");
        std::process::exit(-1);
    }
}
