
extern crate gltf;

#[test]
fn import_v1() {
    // find glTF-Sample-Models/1.0 -name *.gltf -printf "\"%p\",\n" | grep glTF/
    let assets = [
        "glTF-Sample-Models/1.0/2CylinderEngine/glTF/2CylinderEngine.gltf",
        "glTF-Sample-Models/1.0/VC/glTF/VC.gltf",
        "glTF-Sample-Models/1.0/BrainStem/glTF/BrainStem.gltf",
        "glTF-Sample-Models/1.0/BoxAnimated/glTF/BoxAnimated.gltf",
        "glTF-Sample-Models/1.0/BoxWithoutIndices/glTF/BoxWithoutIndices.gltf",
        "glTF-Sample-Models/1.0/CesiumMilkTruck/glTF/CesiumMilkTruck.gltf",
        "glTF-Sample-Models/1.0/Buggy/glTF/Buggy.gltf",
        "glTF-Sample-Models/1.0/Avocado/glTF/Avocado.gltf",
        "glTF-Sample-Models/1.0/WalkingLady/glTF/WalkingLady.gltf",
        "glTF-Sample-Models/1.0/ReciprocatingSaw/glTF/ReciprocatingSaw.gltf",
        "glTF-Sample-Models/1.0/GearboxAssy/glTF/GearboxAssy.gltf",
        "glTF-Sample-Models/1.0/Monster/glTF/Monster.gltf",
        "glTF-Sample-Models/1.0/Duck/glTF/Duck.gltf",
        "glTF-Sample-Models/1.0/RiggedFigure/glTF/RiggedFigure.gltf",
        "glTF-Sample-Models/1.0/BoxTextured/glTF/BoxTextured.gltf",
        "glTF-Sample-Models/1.0/BoxSemantics/glTF/BoxSemantics.gltf",
        "glTF-Sample-Models/1.0/Box/glTF/Box.gltf",
        "glTF-Sample-Models/1.0/SmilingFace/glTF/SmilingFace.gltf",
        "glTF-Sample-Models/1.0/CesiumMan/glTF/CesiumMan.gltf",
        "glTF-Sample-Models/1.0/BarramundiFish/glTF/BarramundiFish.gltf",
        "glTF-Sample-Models/1.0/RiggedSimple/glTF/RiggedSimple.gltf",
    ];
    for asset in &assets {
        match gltf::import::<_, gltf::extras::None>(asset) {
            Ok(_) => {},
            Err(err) => { println!("{:?}", err); panic!() },
        }
    }
}

