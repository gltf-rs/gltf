
extern crate gltf;

use gltf::Generic::V2;

#[test]
fn import_v2() {
    let assets = [
        // These are currently the only available 2.0 compliant sample assets
        "glTF-Sample-Models/2.0/Corset/glTF/Corset.gltf",
        "glTF-Sample-Models/2.0/BoomBox/glTF/BoomBox.gltf",
        "glTF-Sample-Models/2.0/Lantern/glTF/Lantern.gltf",
    ];
    for asset in &assets {
        match gltf::import::<_, gltf::extras::None>(asset) {
            Ok(V2(_)) => {},
            Ok(_) => { println!("import() detected wrong version"); panic!() },
            Err(err) => { println!("{:?}", err); panic!() },
        }
    }
}

