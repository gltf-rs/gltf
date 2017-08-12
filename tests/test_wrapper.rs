extern crate gltf;

use std::{fs, io};

use gltf::mesh::Bounds;
use gltf::json::{self, Path};

use gltf::json::validation::{Validate, Error};

fn import_json(filename: &str) -> gltf::json::Root {
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    gltf::json::from_reader(reader).unwrap()
}

#[test]
fn test_accessor_bounds() {
    // file derived from minimal.gltf with changed min/max values
    let gltf = gltf::Gltf::from_json(import_json("tests/minimal_accessor_min_max.gltf"));
    let mesh = &gltf.meshes().nth(0).unwrap();
    let prim = mesh.primitives().nth(0).unwrap();
    let bounds = prim.bounds();
    assert_eq!(bounds, Some(Bounds { min: [-0.03, -0.04, -0.05], max: [1.0, 1.01, 0.02]}));
}

#[test]
fn test_accessor_bounds_validate_minimally() {
    // file with missing min/max values
    let json = import_json("tests/minimal_accessor_invalid.gltf");

    let mut errs = vec![];
    json.validate_minimally(
        &json,
        || json::Path::new(),
        &mut |path, err| errs.push((path(), err)),
    );
    assert_eq!(errs,
        [(Path("meshes[0].primitives[0].attributes[\"POSITION\"].min".into()), Error::Missing),
        (Path("meshes[0].primitives[0].attributes[\"POSITION\"].max".into()), Error::Invalid)]);
}

// TODO: validate_completely does NOT include validate_minimally -> skip test for now.
#[ignore]
#[test]
fn test_accessor_bounds_validate_completely() {
    // file with missing min/max values
    let json = import_json("tests/minimal_accessor_invalid.gltf");

    let mut errs = vec![];
    json.validate_completely(
        &json,
        || json::Path::new(),
        &mut |path, err| errs.push((path(), err)),
    );
    assert_eq!(errs,
        [(Path("meshes[0].primitives[0].attributes[\"positions\"].min".into()), Error::Missing),
        (Path("meshes[0].primitives[0].attributes[\"positions\"].max".into()), Error::Invalid)]);
}
