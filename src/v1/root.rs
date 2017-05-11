
use std::collections::HashMap;
use v1::*;

/// Extension specific data structures for `Root`.
pub mod extensions {
    /// Header format for binary glTF (without the initial magic string).
    #[cfg(feature = "KHR_binary_glTF")]
    #[derive(Clone, Debug)]
    #[repr(C)]
    pub struct KhrBinaryGltfHeader {
        /// The binary glTF version number.
        pub version: u32,
        
        /// The total length of the binary glTF in bytes, including the
        /// header, content and body.
        pub length: u32,
        
        /// The length in bytes of the glTF content.
        pub content_length: u32,
        
        /// Specifies the format of the glTF content.
        ///
        /// Must be 0 (JSON).
        pub content_format: u32,
    }

    #[cfg(feature = "KHR_binary_glTF")]
    #[derive(Clone, Debug)]
    pub struct KhrBinaryGltf {          
        /// The internal binary glTF data.
        pub body: Vec<u8>,
        
        /// The binary glTF equivalent of the .gltf file.
        pub content: Vec<u8>,

        /// The binary glTF header (without the initial magic string).
        pub header: KhrBinaryGltfHeader,
    }
}

/// The root object for a glTF 1.0 asset.
#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    /// A dictionary object of accessor objects.
    ///
    /// The name of each accessor is an ID in the global glTF namespace that is
    /// used to reference the accessor. An accessor is a typed view into a
    /// bufferView.
    #[serde(default)]
    pub accessors: HashMap<String, accessor::Accessor>,

    /// A dictionary object of keyframe animation objects.
    ///
    /// The name of each animation is an ID in the global glTF namespace that is
    /// used to reference the animation.
    #[serde(default)]
    pub animation: HashMap<String, animation::Animation>,

    /// Metadata about the glTF asset.
    pub asset: asset::Asset,

    /// A dictionary object of buffer objects.
    ///
    /// The name of each buffer is an ID in the global glTF namespace that is
    /// used to reference the buffer. A buffer points to binary geometry,
    /// animation, or skins.
    #[serde(default)]
    pub buffers: HashMap<String, buffer::Buffer>,

    /// A dictionary object of bufferView objects.
    ///
    /// The name of each bufferView is an ID in the global glTF namespace that
    /// is used to reference the bufferView. A bufferView is a view into a
    /// buffer generally representing a subset of the buffer.
    #[serde(rename = "bufferViews")]
    #[serde(default)]
    pub buffer_views: HashMap<String, buffer::BufferView>,

    /// A dictionary object of camera objects.
    ///
    /// The name of each camera is an ID in the global glTF namespace that is
    /// used to reference the camera. A camera defines a projection matrix.
    #[serde(default)]
    pub cameras: HashMap<String, camera::Camera>,

    /// Names of glTF extensions used somewhere in this asset.
    #[serde(rename = "extensionsUsed")]
    #[serde(default)]
    pub extensions_used: Vec<String>,

    /// Names of glTF extensions required to properly load this asset.
    #[serde(rename = "extensionsRequired")]
    #[serde(default)]
    pub extensions_required: Vec<String>,

    /// A dictionary object of image objects.
    ///
    /// The name of each image is an ID in the global glTF namespace that is
    /// used to reference the image. An image defines data used to create a
    /// texture.
    #[serde(default)]
    pub images: HashMap<String, image::Image>,

    /// A dictionary object of material objects.
    ///
    /// The name of each material is an ID in the global glTF namespace that is
    /// used to reference the material. A material defines the appearance of a
    /// primitive.
    #[serde(default)]
    pub materials: HashMap<String, material::Material>,

    /// A dictionary object of mesh objects.
    ///
    /// The name of each mesh is an ID in the global glTF namespace that is used
    /// to reference the mesh. A mesh is a set of primitives to be rendered.
    #[serde(default)]
    pub meshes: HashMap<String, mesh::Mesh>,

    /// A dictionary object of node objects in the node hierarchy.
    ///
    /// The name of each node is an ID in the global glTF namespace that is used
    /// to reference the node.
    #[serde(default)]
    pub nodes: HashMap<String, scene::Node>,

    /// A dictionary object of shader program objects.
    ///
    /// The name of each program is an ID in the global glTF namespace that is
    /// used to reference the program.
    #[serde(default)]
    pub programs: HashMap<String, program::Program>,

    /// A dictionary object of sampler objects.
    ///
    /// The name of each sampler is an ID in the global glTF namespace that is
    /// used to reference the sampler. A sampler contains properties for texture
    /// filtering and wrapping modes.
    #[serde(default)]
    pub samplers: HashMap<String, texture::Sampler>,

    /// The ID of the default scene.
    pub scene: Option<String>,

    /// A dictionary object of scene objects.
    ///
    /// The name of each scene is an ID in the global glTF namespace that is
    /// used to reference the scene.
    #[serde(default)]
    pub scenes: HashMap<String, scene::Scene>,

    /// A dictionary object of shader objects.
    ///
    /// The name of each shader is an ID in the global glTF namespace that is
    /// used to reference the shader.
    #[serde(default)]
    pub shaders: HashMap<String, shader::Shader>,

    /// A dictionary object of skin objects.
    ///
    /// The name of each skin is an ID in the global glTF namespace that is used
    /// to reference the skin. A skin is defined by joints and matrices.
    #[serde(default)]
    pub skins: HashMap<String, skin::Skin>,

    /// A dictionary object of technique objects.
    ///
    /// The name of each technique is an ID in the global glTF namespace that is
    /// used to reference the technique. A technique is a template for a
    /// material appearance.
    #[serde(default)]
    pub techniques: HashMap<String, technique::Technique>,

    /// A dictionary object of texture objects.
    ///
    /// The name of each texture is an ID in the global glTF namespace that is
    /// used to reference the texture.
    #[serde(default)]
    pub textures: HashMap<String, texture::Texture>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: RootExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Root`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RootExtensions {
    /// Owns the binary encoded data for binary glTF 1.0 assets.
    #[cfg(feature = "KHR_binary_glTF")]
    #[serde(skip_deserializing, skip_serializing)]
    pub khr_binary_gltf: Option<extensions::KhrBinaryGltf>,

    _allow_extra_fields: (),
}

