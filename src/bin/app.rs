
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

pub type Extensions = gltf::extensions::None;

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
    let v1 = gltf::import::<_, Extensions, Extras>("tests/Box-1.0.gltf")
        .unwrap()
        .to_v1()
        .unwrap();
    println!("Box (1.0)");
    println!("=========");
    println!("{:#?}", v1.accessors);

    let v2 = gltf::import::<_, Extensions, Extras>("tests/Lantern-2.0.gltf")
        .unwrap()
        .to_v2()
        .unwrap();
    println!("Lantern (2.0)");
    println!("=========");
    println!("{:#?}", v2.accessors());
}

