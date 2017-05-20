
extern crate gltf;

#[test]
fn import_v2() {
    let assets = [
        // minimal example taken from https://github.com/javagl/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_006_SimpleAnimation.md
        "tests/minimal.gltf",
        "glTF-Sample-Models/2.0/2CylinderEngine/glTF/2CylinderEngine.gltf",
        "glTF-Sample-Models/2.0/AnimatedCube/glTF/AnimatedCube.gltf",
        "glTF-Sample-Models/2.0/AnimatedMorphCube/glTF/AnimatedMorphCube.gltf",
        "glTF-Sample-Models/2.0/AnimatedMorphSphere/glTF/AnimatedMorphSphere.gltf",
        "glTF-Sample-Models/2.0/AnimatedTriangle/glTF/AnimatedTriangle.gltf",
        "glTF-Sample-Models/2.0/Avocado/glTF/Avocado.gltf",
        "glTF-Sample-Models/2.0/BarramundiFish/glTF/BarramundiFish.gltf",
        "glTF-Sample-Models/2.0/BoomBox/glTF/BoomBox.gltf",
        "glTF-Sample-Models/2.0/Box/glTF/Box.gltf",
        "glTF-Sample-Models/2.0/BoxAnimated/glTF/BoxAnimated.gltf",
        "glTF-Sample-Models/2.0/BoxTextured/glTF/BoxTextured.gltf",
        "glTF-Sample-Models/2.0/BrainStem/glTF/BrainStem.gltf",
        "glTF-Sample-Models/2.0/Buggy/glTF/Buggy.gltf",
        "glTF-Sample-Models/2.0/Cameras/glTF/Cameras.gltf",
        "glTF-Sample-Models/2.0/CesiumMan/glTF/CesiumMan.gltf",
        "glTF-Sample-Models/2.0/CesiumMilkTruck/glTF/CesiumMilkTruck.gltf",
        "glTF-Sample-Models/2.0/Corset/glTF/Corset.gltf",
        "glTF-Sample-Models/2.0/Cube/glTF/Cube.gltf",
        "glTF-Sample-Models/2.0/Duck/glTF/Duck.gltf",
        "glTF-Sample-Models/2.0/GearboxAssy/glTF/GearboxAssy.gltf",
        "glTF-Sample-Models/2.0/Lantern/glTF/Lantern.gltf",
        // See https://github.com/KhronosGroup/glTF-Sample-Models/issues/70
        // "glTF-Sample-Models/2.0/MetalRoughSpheres/glTF/MetalRoughSpheres.gltf",
        "glTF-Sample-Models/2.0/Monster/glTF/Monster.gltf",
        "glTF-Sample-Models/2.0/NormalTangentTest/glTF/NormalTangentTest.gltf",
        "glTF-Sample-Models/2.0/ReciprocatingSaw/glTF/ReciprocatingSaw.gltf",
        "glTF-Sample-Models/2.0/RiggedFigure/glTF/RiggedFigure.gltf",
        "glTF-Sample-Models/2.0/RiggedSimple/glTF/RiggedSimple.gltf",
        "glTF-Sample-Models/2.0/SciFiHelmet/glTF/SciFiHelmet.gltf",
        "glTF-Sample-Models/2.0/SimpleMeshes/glTF/SimpleMeshes.gltf",
        "glTF-Sample-Models/2.0/SmilingFace/glTF/SmilingFace.gltf",
        "glTF-Sample-Models/2.0/Suzanne/glTF/Suzanne.gltf",
        "glTF-Sample-Models/2.0/Triangle/glTF/Triangle.gltf",
        "glTF-Sample-Models/2.0/TriangleWithoutIndices/glTF/TriangleWithoutIndices.gltf",
        "glTF-Sample-Models/2.0/TwoSidedPlane/glTF/TwoSidedPlane.gltf",
        "glTF-Sample-Models/2.0/VC/glTF/VC.gltf",
        "glTF-Sample-Models/2.0/WalkingLady/glTF/WalkingLady.gltf",
    ];
    for asset in assets.iter() {
        match gltf::v2::import(asset) {
            Ok(_) => {}
            Err(err) => {
                println!("{}: {:?}", asset, err);
                panic!()
            }
        }
    }
}
