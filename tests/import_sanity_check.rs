use std::error::Error as StdError;
use std::{fs, path};

const SAMPLE_MODELS_DIRECTORY_PATH: &str = "glTF-Sample-Assets/Models";

fn check_import_result(
    result: gltf::Result<(
        gltf::Document,
        Vec<gltf::buffer::Data>,
        Vec<gltf::image::Data>,
    )>,
) {
    use gltf::json::validation::Error;
    match result {
        Err(gltf::Error::Validation(errors)) => {
            assert!(errors
                .iter()
                .all(|(_path, error)| *error == Error::Unsupported));
            println!("skipped");
        }
        Err(otherwise) => {
            panic!("{otherwise:#?}");
        }
        Ok((document, buffer_data, image_data)) => {
            // Check buffers.
            assert_eq!(document.buffers().len(), buffer_data.len());

            for (buf, data) in document.buffers().zip(buffer_data.iter()) {
                assert!((buf.length() + 3) & !3 <= data.0.len())
            }

            // Check images.
            assert_eq!(document.images().len(), image_data.len());

            println!("ok");
        }
    }
}

fn run() -> Result<(), Box<dyn StdError>> {
    let sample_dir_path = path::Path::new(SAMPLE_MODELS_DIRECTORY_PATH);
    for entry in fs::read_dir(sample_dir_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            let entry_path = entry.path();
            if let Some(file_name) = entry_path.file_name() {
                // Import standard glTF.
                let mut gltf_path = entry_path.join("glTF").join(file_name);
                gltf_path.set_extension("gltf");
                if gltf_path.exists() {
                    print!("{}: ", gltf_path.display());
                    check_import_result(gltf::import(&gltf_path));
                }

                // Import standard glTF with embedded buffer and image data.
                let mut gle_path = entry_path.join("glTF-Embedded").join(file_name);
                gle_path.set_extension("gltf");
                if gle_path.exists() {
                    print!("{}: ", gle_path.display());
                    check_import_result(gltf::import(&gle_path));
                }

                // Import binary glTF.
                let mut glb_path = entry_path.join("glTF-Binary").join(file_name);
                glb_path.set_extension("glb");
                if glb_path.exists() {
                    print!("{}: ", glb_path.display());
                    check_import_result(gltf::import(&glb_path));
                }
            }
        }
    }

    sparse_accessor_without_buffer_view_test()
}

/// Test a file with a sparse accessor with no buffer view
fn sparse_accessor_without_buffer_view_test() -> Result<(), Box<dyn StdError>> {
    let glb_path = path::Path::new("tests/box_sparse.glb");
    print!("{}: ", glb_path.display());
    check_import_result(gltf::import(glb_path));

    let gltf_path = path::Path::new("tests/box_sparse.gltf");
    print!("{}: ", gltf_path.display());
    check_import_result(gltf::import(gltf_path));
    Ok(())
}

#[test]
fn import_sample_models() {
    if let Err(error) = run() {
        panic!("import failed: {:?}", error);
    }
}
