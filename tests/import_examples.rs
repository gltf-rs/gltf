
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate futures;
extern crate futures_cpupool;
extern crate gltf;

use futures::future;
use std::{fs, io, path};

use futures::{BoxFuture, Future};
use futures_cpupool::CpuPool;

type ImportError = gltf::import::Error<gltf::import::FromPath>;

fn import_from_path(path: path::PathBuf) -> BoxFuture<(), ImportError> {
    future::lazy(move || {
        let path = path;
        match gltf::Import::from_path(&path).sync() {
            Ok(_) => {
                println!("{:?}: Ok", path);
                Ok(())
            },
            Err(err) => {
                println!("{:?}: Err({:?})", path, err);
                Err(err)
            },
        }
    }).boxed()
}

fn collect_imports() -> io::Result<Vec<BoxFuture<(), ImportError>>> {
    let sample_dir_path = path::Path::new("./glTF-Sample-Models/2.0");
    let mut imports = vec![];
    for entry in fs::read_dir(&sample_dir_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            let entry_path = entry.path();
            if let Some(file_name) = entry_path.file_name() {
                // Import standard glTF
                let mut gltf_path = entry_path.join("glTF").join(file_name);
                gltf_path.set_extension("gltf");
                imports.push(import_from_path(gltf_path));
                // Import binary glTF
                let mut glb_path = entry_path.join("glTF-Binary").join(file_name);
                glb_path.set_extension("glb");
                if glb_path.exists() {
                    imports.push(import_from_path(glb_path));
                }
            }
        }
    }
    Ok(imports)
}

fn run() -> Result<(), ImportError> {
    let pool = CpuPool::new(8);
    let mut items = vec![];
    for task in collect_imports()? {
        items.push(pool.spawn(task));
    }
    let _ = future::join_all(items).wait()?;
    Ok(())
}

#[test]
fn import() {
    let _ = run().expect("No errors");
}
