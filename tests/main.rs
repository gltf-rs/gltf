use std::path::Path;

type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;
type UnitResult = Result<()>;

const SAMPLE_MODELS_DIRECTORY_PATH: &str = "glTF-Sample-Assets/Models";

fn visit(category: &str, extension: &str, visitor: &dyn Fn(&Path) -> UnitResult) -> UnitResult {
    let sample_dir_path = Path::new(SAMPLE_MODELS_DIRECTORY_PATH);
    for entry in std::fs::read_dir(sample_dir_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            let entry_path = entry.path();
            if let Some(dir_name) = entry_path.file_name() {
                let mut file_path = entry_path.join(category).join(dir_name);
                file_path.set_extension(extension);
                if file_path.exists() {
                    print!("{}: ", file_path.display());
                    let _ = visitor(&file_path)?;
                    println!("ok");
                }
            }
        }
    }
    Ok(())
}

#[test]
fn deserialize_standard() -> UnitResult {
    visit("glTF", "gltf", &|path| {
        let file = std::fs::read_to_string(path)?;
        if let Err(error) = serde_json::from_str::<gltf::Root>(&file) {
            panic!("failed to parse {}: {}", path.display(), error);
        } else {
            Ok(())
        }
    })
}

#[test]
fn deserialize_binary() -> UnitResult {
    visit("glTF-Binary", "glb", &|path| {
        let file = std::fs::read(path)?;
        if let Err(error) = gltf::binary::Glb::from_slice(&file) {
            panic!("failed to parse {}: {}", path.display(), error);
        } else {
            Ok(())
        }
    })
}

#[test]
fn deserialize_embedded() -> UnitResult {
    visit("glTF-Embedded", "gltf", &|path| {
        let file = std::fs::read_to_string(path)?;
        if let Err(error) = serde_json::from_str::<gltf::Root>(&file) {
            panic!("failed to parse {}: {}", path.display(), error);
        } else {
            Ok(())
        }
    })
}
