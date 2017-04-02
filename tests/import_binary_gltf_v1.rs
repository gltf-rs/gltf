
extern crate gltf;

#[cfg(feature = "KHR_binary_glTF")]
#[test]
fn import_v1() {
    // find glTF-Sample-Models/1.0 -name *.glb -printf "\"%p\",\n" | grep glTF-Binary/
    let assets = [
        "glTF-Sample-Models/1.0/2CylinderEngine/glTF-Binary/2CylinderEngine.glb",
        "glTF-Sample-Models/1.0/VC/glTF-Binary/VC.glb",
        "glTF-Sample-Models/1.0/BrainStem/glTF-Binary/BrainStem.glb",
        "glTF-Sample-Models/1.0/BoxAnimated/glTF-Binary/BoxAnimated.glb",
        "glTF-Sample-Models/1.0/BoxWithoutIndices/glTF-Binary/BoxWithoutIndices.glb",
        "glTF-Sample-Models/1.0/CesiumMilkTruck/glTF-Binary/CesiumMilkTruck.glb",
        "glTF-Sample-Models/1.0/Buggy/glTF-Binary/Buggy.glb",
        "glTF-Sample-Models/1.0/Avocado/glTF-Binary/Avocado.glb",
        "glTF-Sample-Models/1.0/WalkingLady/glTF-Binary/WalkingLady.glb",
        "glTF-Sample-Models/1.0/ReciprocatingSaw/glTF-Binary/ReciprocatingSaw.glb",
        "glTF-Sample-Models/1.0/GearboxAssy/glTF-Binary/GearboxAssy.glb",
        "glTF-Sample-Models/1.0/Monster/glTF-Binary/Monster.glb",
        "glTF-Sample-Models/1.0/Duck/glTF-Binary/Duck.glb",
        "glTF-Sample-Models/1.0/RiggedFigure/glTF-Binary/RiggedFigure.glb",
        "glTF-Sample-Models/1.0/BoxTextured/glTF-Binary/BoxTextured.glb",
        "glTF-Sample-Models/1.0/BoxSemantics/glTF-Binary/BoxSemantics.glb",
        "glTF-Sample-Models/1.0/Box/glTF-Binary/Box.glb",
        "glTF-Sample-Models/1.0/SmilingFace/glTF-Binary/SmilingFace.glb",
        "glTF-Sample-Models/1.0/CesiumMan/glTF-Binary/CesiumMan.glb",
        "glTF-Sample-Models/1.0/BarramundiFish/glTF-Binary/BarramundiFish.glb",
        "glTF-Sample-Models/1.0/RiggedSimple/glTF-Binary/RiggedSimple.glb"
    ];
    for asset in &assets {
        match gltf::v1::import::<_, gltf::v1::extras::None>(&asset) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
                panic!()
            }
        }
    }
}

