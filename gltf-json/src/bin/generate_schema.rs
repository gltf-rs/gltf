use schemars::schema_for;

fn main() {
    let schema = schema_for!(gltf_json::extensions::root::KittyCadBoundaryRepresentation);
    let json = serde_json::to_string_pretty(&schema).unwrap();
    println!("{json}");
}
