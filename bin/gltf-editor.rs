extern crate gltf;

use std::{env, fs, io, str};

use gltf::Gltf;
use std::boxed::Box;
use std::error::Error;
use std::path::Path;

fn read_to_end(path: &str) -> Result<Vec<u8>, Box<Error>> {
    use io::Read;
    let file = fs::File::open(path)?;
    let len = file.metadata()?.len() as usize;
    let mut reader = io::BufReader::new(file);
    let mut data = Vec::with_capacity(len);
    let _ = reader.read_to_end(&mut data)?;
    Ok(data)
}

fn print_prologue(gltf: &Gltf) {
    macro_rules! print_len {
        ($item:ident) => {
            println!("{}: {}", stringify!($item), gltf.$item().len());
        };
    }

    print_len!(accessors);
    print_len!(animations);
    print_len!(buffers);
    print_len!(cameras);
    if let Some(scene) = gltf.default_scene() {
        println!("default_scene: {}", scene.index());
    } else {
        println!("default_scene: nil");
    }
    // TODO: Requires #104 to be fixed.
    // print_len!(images);
    print_len!(materials);
    print_len!(meshes);
    print_len!(samplers);
    print_len!(skins);
    print_len!(textures);
}

fn print_cmd(_: &Gltf, mut args: str::SplitWhitespace) -> Result<(), Box<Error>> {
    if let Some(path) = args.next() {
        if path.starts_with("cameras") {
            
        }
    }
    Ok(())
}

fn print_prompt() {
    use io::Write;
    let stdout = io::stdout();
    let mut output = stdout.lock();
    let _ = write!(output, "> ").unwrap();
    let _ = output.flush().unwrap();
}


fn run_gui_loop(_: &Gltf) -> Result<(), Box<Error>> {
    unimplemented!()
}

fn run_tui_loop(gltf: &Gltf) -> Result<(), Box<Error>> {
    use io::BufRead;
    let stdin = io::stdin();
    let input = io::BufReader::new(stdin.lock());
    print_prompt();
    for result in input.lines() {
        let line = result?;
        let mut args = line.split_whitespace();
        let cmd = args.next();
        match cmd {
            Some("p") | Some("print") => print_cmd(&gltf, args)?,
            Some("q") | Some("quit") => break,
            _ => println!("?"),
        }
        print_prompt();
    }
    Ok(())
}

fn run(path: &str, mut args: env::Args) -> Result<(), Box<Error>> {
    let contents = read_to_end(path)?;
    let gltf = Gltf::from_slice(&contents)?.validate_minimally()?;
    print_prologue(&gltf);
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "--gui" => run_gui_loop(&gltf)?,
            "--tui" => run_tui_loop(&gltf)?,
            _ => {},
        }
    }
    Ok(())
}

fn main() {
    let mut args = env::args();
    if let Some(path) = args.nth(1) {
        if Path::new(&path).exists() {
            let _ = run(&path, args).unwrap();
        } else {
            println!("error: File \"{}\" not found", path);
        }
    } else {
        println!("usage: gltf-treeview <PATH>");
    }
}
