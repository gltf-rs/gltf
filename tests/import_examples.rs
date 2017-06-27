
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf;

use std::{fs, io, path};

fn try_static_import(importer: &gltf::StaticImporter, path: &path::Path) {
    match importer.import_from_path(path) {
        Ok(_) => println!("Ok: {:?}", path),
        Err(err) => {
            println!("Err: {:?} ({:#?})", path, err);
            panic!();
        },
    }
}

fn try_zero_copy_import(importer: &mut gltf::ZeroCopyImporter, path: &path::Path) {
    match importer.import_from_path(path) {
        Ok(_) => println!("Ok: {:?}", path),
        Err(err) => {
            println!("Err: {:?} ({:#?})", path, err);
            panic!();
        },
    }
}

fn run() -> io::Result<()> {
    let static_importer = gltf::StaticImporter::new();
    let mut zero_copy_importer = gltf::ZeroCopyImporter::new();
    let sample_dir_path = path::Path::new("./glTF-Sample-Models/2.0");
    for entry in fs::read_dir(&sample_dir_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap();

            // Import .gltf
            let mut gltf_path = entry_path.join("glTF").join(file_name);
            gltf_path.set_extension("gltf");
            if gltf_path.exists() {
                try_static_import(&static_importer, &gltf_path);
                try_zero_copy_import(&mut zero_copy_importer, &gltf_path);
            }

            // Import corresponding .glb
            let mut glb_path = entry_path.join("glTF-Binary").join(file_name);
            glb_path.set_extension("glb");
            if glb_path.exists() {
                try_static_import(&static_importer, &glb_path);
                try_zero_copy_import(&mut zero_copy_importer, &gltf_path);
            }
        }
    }
    Ok(())
}

#[test]
fn import() {
    // Import all 'standard' glTF in the glTF-Sample-Models/2.0 directory.
    run().expect("No I/O errors");

    // Temporarily removed until base64 decoding is implemented
    // Minimal example taken from https://github.com/javagl/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_003_MinimalGltfFile.md
    // try_import(path::Path::new("tests/minimal.gltf"));
}

