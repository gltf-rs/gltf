
extern crate gltf;

use std::{fs, io, path};

fn try_import(path: &path::Path) {
    let _ = gltf::v1::import(&path).map_err(|err| {
        println!("{:?}: {:?}", path, err);
        panic!();
    });
}

fn run() -> io::Result<()> {
    let sample_dir_path = path::Path::new("./glTF-Sample-Models/1.0");
    for entry in fs::read_dir(&sample_dir_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            let entry_path = entry.path();
            if let Some(file_name) = entry_path.file_name() {
                let mut gltf_path = entry_path.join("glTF-Binary").join(file_name);
                gltf_path.set_extension("glb");
                try_import(&gltf_path);
            }
        }
    }
    Ok(())
}

#[cfg(feature = "KHR_binary_glTF")]
#[test]
fn import_v1() {
    run().expect("No I/O errors");
}

