extern crate gltf;

use gltf::json;
use std::{fs, io};

use std::boxed::Box;
use std::error::Error as StdError;

fn run(path: &str) -> Result<(), Box<StdError>> {
    use io::Read;
    let file = fs::File::open(&path)?;
    let mut data = Vec::with_capacity(file.metadata()?.len() as usize);
    let mut reader = io::BufReader::new(file);
    let _ = reader.read_to_end(&mut data)?;
    let json: json::Root = if gltf::is_binary(&data) {
        let glb = gltf::Glb::from_slice(&data)?;
        json::from_slice(glb.json)
    } else {
        json::from_slice(&data)
    }?;
    println!("{:#?}", json);
    Ok(())
}

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        run(&path).expect("runtime error");
    } else {
        println!("usage: gltf-display <FILE>");
    }
}
