
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf;

use std::{fs, io, path};

fn try_import(path: &path::Path) {
    let mut importer = gltf::Importer::new();
    let _ = importer.import_from_path(path).map_err(|err| {
        println!("{:?}: {:#?}", path, err);
        panic!();
    });
}

fn run() -> io::Result<()> {
    let sample_dir_path = path::Path::new("./glTF-Sample-Models/2.0");
    for entry in fs::read_dir(&sample_dir_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            let entry_path = entry.path();
            if let Some(file_name) = entry_path.file_name() {
                let mut gltf_path = entry_path.join("glTF").join(file_name);
                gltf_path.set_extension("gltf");
                try_import(&gltf_path);
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

