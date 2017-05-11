
extern crate gltf;

#[test]
fn import_v2() {
    let assets = [
        // These are currently the only available 2.0 compliant sample assets
        "glTF-Sample-Models/2.0/Corset/glTF/Corset.gltf",
        "glTF-Sample-Models/2.0/BoomBox/glTF/BoomBox.gltf",
        "glTF-Sample-Models/2.0/Lantern/glTF/Lantern.gltf",
    ];
    for asset in &assets {
        match gltf::v2::import(asset) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
                panic!()
            }
        }
    }
}
