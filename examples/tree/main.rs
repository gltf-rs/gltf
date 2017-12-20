extern crate gltf;

use std::{fs, io};

use gltf::{Glb, Gltf};
use std::boxed::Box;
use std::error::Error as StdError;

fn print_tree(node: &gltf::Node, depth: i32) {
    for _ in 0..(depth - 1) {
        print!("  ");
    }
    print!(" -");
    let index = node.index();
    let name = node.name().unwrap_or("<Unnamed>");
    println!(" Node {} ({})", index, name);
    for child in node.children() {
        print_tree(&child, depth + 1);
    }
}

fn run(path: &str) -> Result<(), Box<StdError>> {
    use io::Read;
    let file = fs::File::open(&path)?;
    let mut data = Vec::with_capacity(file.metadata()?.len() as usize);
    let mut reader = io::BufReader::new(file);
    let _ = reader.read_to_end(&mut data)?;
    let gltf = if gltf::is_binary(&data) {
        let glb = Glb::from_slice(&data)?;
        Gltf::from_slice(&glb.json)
    } else {
        Gltf::from_slice(&data)
    }?.validate_completely()?;
    for scene in gltf.scenes() {
        let index = scene.index();
        let name = scene.name().unwrap_or("<Unnamed>");
        println!("Scene {} ({})", index, name);
        for node in scene.nodes() {
            print_tree(&node, 1);
        }
    }
    Ok(())
}

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        run(&path).expect("runtime error");
    } else {
        println!("usage: gltf-tree <FILE>");
    }
}
