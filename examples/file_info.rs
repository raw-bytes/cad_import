use std::{collections::HashSet, env, fs::File, path::Path, rc::Rc};

use cad_import::{
    loader::{Loader, Manager},
    structure::{CADData, Node},
    ID,
};

///! This example loads the given file and dumps all infos about it to console

fn usage() {
    println!("usage: file_info <file-path> [<mime-type>]\n");
    println!("file-path: The path to the cad file to parse.");
    println!(
        "mime-type: Optional parameter to specify the mimetype. If not provided, the extension
           of the file is used"
    );
}

fn find_loader(
    manager: &Manager,
    input_file: &Path,
    mime_type: Option<&str>,
) -> Option<Rc<dyn Loader>> {
    match mime_type {
        Some(mime_type) => manager.get_loader_by_mime_type(mime_type),
        None => match input_file.extension() {
            Some(ext) => match ext.to_str() {
                Some(ext) => manager.get_loader_by_extension(ext),
                None => {
                    eprintln!("Input file has invalid extension");
                    None
                }
            },
            None => {
                eprintln!("Input file has no extension");
                None
            }
        },
    }
}

struct VisitorContext {
    pub num_nodes: usize,
    pub num_vertices: usize,
    pub num_primitives: usize,
    pub shapes: HashSet<ID>,
}

fn visit_node(node: &Node, ctx: &mut VisitorContext) {
    ctx.num_nodes += 1;

    for shape in node.get_shapes().iter() {
        let id = shape.get_id();
        if ctx.shapes.contains(&id) {
            continue;
        }

        ctx.shapes.insert(id);

        for part in shape.get_parts() {
            let mesh = part.get_mesh();
            ctx.num_vertices += mesh.get_vertices().len();
            ctx.num_primitives += mesh.get_primitives().num_primitives();
        }
    }

    for child in node.get_children().iter() {
        visit_node(child, ctx);
    }
}

fn dump_info(cad_data: &CADData) {
    let mut ctx = VisitorContext {
        num_nodes: 0,
        num_vertices: 0,
        num_primitives: 0,
        shapes: HashSet::new(),
    };
    visit_node(cad_data.get_root_node(), &mut ctx);

    println!("Statistics:");
    println!("Num Vertices: {}", ctx.num_vertices);
    println!("Num Primitives: {}", ctx.num_primitives);
    println!("Num Nodes: {}", ctx.num_nodes);
    println!("Num Shapes: {}", ctx.shapes.len());
}

fn run_program(input_file: &Path, mime_type: Option<&str>) -> bool {
    let manager = Manager::new();

    let loader = find_loader(&manager, input_file, mime_type);
    let loader = match loader {
        None => {
            eprintln!("Cannot find loader");
            return false;
        }
        Some(loader) => loader,
    };

    println!("Reading file {:?}...", input_file);
    let mut reader = match File::open(input_file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Reading file {:?}...FAILED", input_file);
            eprintln!("Error: {}", err);
            return false;
        }
    };

    let cad_data = match loader.read_file(&mut reader) {
        Ok(cad_data) => cad_data,
        Err(err) => {
            eprintln!("Reading file {:?}...FAILED", input_file);
            eprintln!("Error: {}", err);
            return false;
        }
    };

    println!("Reading file {:?}...DONE", input_file);

    dump_info(&cad_data);

    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    // check if the number of arguments is invalid
    if args.len() == 0 {
        usage();
        std::process::exit(-1);
    } else if args.len() > 2 {
        eprintln!("Too many arguments!!!\n\n");
        usage();
        std::process::exit(-1);
    }

    // parse arguments
    let input_file = Path::new(&args[0]);
    let mime_type = if args.len() == 2 {
        Some(args[1].as_str())
    } else {
        None
    };

    if run_program(input_file, mime_type) {
        println!("FINISHED");
    } else {
        eprintln!("FAILED!!!");
        std::process::exit(-1);
    }
}
