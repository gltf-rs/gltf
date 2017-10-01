
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf_importer;

use std::{fs, path};

use gltf_importer::{import, Error as ImportError};
use std::boxed::Box;
use std::error::Error as StdError;

fn import_from_path(path: &path::Path) -> Result<(), ImportError> {
    match import(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

#[allow(type_complexity)]
fn run() -> Result<Vec<(path::PathBuf, Result<(), ImportError>)>, Box<StdError>> {
    let sample_dir_path = path::Path::new("../glTF-Sample-Models/2.0");
    let mut results = vec![];
    for entry in fs::read_dir(&sample_dir_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            let entry_path = entry.path();
            if let Some(file_name) = entry_path.file_name() {
                // Import standard glTF.
                let mut gltf_path = entry_path.join("glTF").join(file_name);
                gltf_path.set_extension("gltf");
                let result = import_from_path(&gltf_path);
                results.push((gltf_path, result));

                // Import standard glTF with embedded buffer and image data.
                let mut gle_path = entry_path.join("glTF-Embedded").join(file_name);
                gle_path.set_extension("gltf");
                if gle_path.exists() {
                    let result = import_from_path(&gle_path);
                    results.push((gle_path, result));
                }

                // Import binary glTF.
                let mut glb_path = entry_path.join("glTF-Binary").join(file_name);
                glb_path.set_extension("glb");
                if glb_path.exists() {
                    let result = import_from_path(&glb_path);
                    results.push((glb_path, result));
                }
            }
        }
    }
    Ok(results)
}

#[test]
fn import_examples() {
    let results = run().expect("I/O error");
    let mut nerrs = 0;
    for (path, result) in results {
        match result {
            Ok(()) => println!("{:?}: Ok", path),
            Err(err) => {
                println!("{:?}: Err ({:?})", path, err);
                nerrs += 1;
            },
        }
    }
    if nerrs != 0 {
        panic!("{} import{} failed", nerrs, if nerrs > 1 { "s" } else { "" });
    }
}
