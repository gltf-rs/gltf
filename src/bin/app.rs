
extern crate gltf;
#[macro_use]
extern crate serde_derive;

pub mod extras {
    pub mod accessor {
        #[derive(Clone, Debug, Default, Deserialize, Serialize)]
        pub struct NormalizedInteger {
            pub normalized: bool,
        }
        
        #[derive(Clone, Debug, Default, Deserialize, Serialize)]
        pub struct Accessor {
            #[serde(default, rename = "normalizedInteger")]
            pub normalized_integer: NormalizedInteger,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Extras;

impl gltf::traits::Extras for Extras {
    type Root = ();
    type Accessor = extras::accessor::Accessor;
    type Asset = ();
    type Animation = ();
    type AnimationChannel = ();
    type AnimationSampler = ();
    type AnimationTarget = ();
    type Buffer = ();
    type BufferView = ();
    type Camera = ();
    type CameraOrthographic = ();
    type CameraPerspective = ();
    type Image = ();
    type Material = ();
    type Mesh = ();
    type MeshPrimitive = ();
    type Node = ();
    type Program = ();
    type Sampler = ();
    type Scene = ();
    type Shader = ();
    type Skin = ();
    type Technique = ();
    type TechniqueState = ();
    type TechniqueFunction = ();
    type TechniqueParameter = ();
    type Texture = ();
}

fn main() {
    println!("Supported 1.0 extensions: {:?}",
             gltf::v1::extensions::SUPPORTED_EXTENSION_NAMES);
    println!("Supported 2.0 extensions: {:?}",
             gltf::v2::extensions::SUPPORTED_EXTENSION_NAMES);
    
    let gltf = gltf::import::<_, Extras>("tests/Box-1.0.gltf")
        .unwrap()
        .to_v1()
        .unwrap();
    println!("Box (1.0) with custom Extras");
    println!("=========");
    println!("{:#?}", gltf.accessors);
    
    let gltf = gltf::import::<_, gltf::extras::Any>("tests/Box-1.0.gltf")
        .unwrap()
        .to_v1()
        .unwrap();
    println!("Box (1.0) with Any");
    println!("=========");
    println!("{:#?}", gltf.accessors);

    let gltf = gltf::import::<_, gltf::extras::None>("tests/Box-1.0.gltf")
        .unwrap()
        .to_v1()
        .unwrap();
    println!("Box (1.0) with None");
    println!("=========");
    println!("{:#?}", gltf.accessors);

    let gltf = gltf::import::<_, Extras>("tests/Lantern-2.0.gltf")
        .unwrap()
        .to_v2()
        .unwrap();
    println!("Lantern (2.0) with custom Extras");
    println!("=========");
    println!("{:#?}", gltf.accessors());

    let gltf = gltf::import::<_, gltf::extras::Any>("tests/Lantern-2.0.gltf")
        .unwrap()
        .to_v2()
        .unwrap();
    println!("Lantern (2.0) with Any");
    println!("=========");
    println!("{:#?}", gltf.accessors());
    
    let gltf = gltf::import::<_, gltf::extras::None>("tests/Lantern-2.0.gltf")
        .unwrap()
        .to_v2()
        .unwrap();
    println!("Lantern (2.0) with None");
    println!("=========");
    println!("{:#?}", gltf.accessors());
}

