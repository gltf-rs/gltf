
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
mod macros;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct AccessorIndex(u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct BufferIndex(u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct BufferViewIndex(u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct CameraIndex(u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct ImageIndex(u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct MaterialIndex(u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct MeshIndex(u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct NodeIndex(u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct SamplerIndex(u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct SceneIndex(u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct SkinIndex(u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct TextureIndex(u32);

pub type UntypedObject = std::collections::HashMap<String, serde_json::Value>;
pub type Extensions = Option<UntypedObject>;
pub type Extras = Option<UntypedObject>;

/// Run time error encountered when loading a glTF asset
#[derive(Debug)]
pub enum LoadError {
    /// Standard input / output error
    Io(std::io::Error),
    /// Failure when deserializing the .gltf metadata file
    De(serde_json::error::Error),
}

/// [The root object for a glTF asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#gltf)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Gltf {
    accessors: Vec<Accessor>,
    animations: Vec<Animation>,
    asset: Asset,
    buffers: Vec<Buffer>,
    #[serde(rename = "bufferViews")]
    buffer_views: Vec<BufferView>,
    #[serde(rename = "extensionsUsed")]
    extensions_used: Vec<String>,
    #[serde(rename = "extensionsRequired")]
    extensions_required: Vec<String>,
    cameras: Vec<Camera>,
    images: Vec<Image>,
    materials: Vec<Material>,
    meshes: Vec<Mesh>,
    nodes: Vec<Node>,
    samplers: Vec<Sampler>,
    scene: SceneIndex,
    scenes: Vec<Scene>,
    skins: Vec<Skin>,
    textures: Vec<Texture>,
}

/// [Defines a method for retrieving data from within a `BufferView`]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#accessors)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Accessor {
    /// The identifier of the `BufferView` this accessor reads from.
    #[serde(rename = "bufferView")]
    pub buffer_view: BufferViewIndex,
    /// Where the data items begin from in the `BufferView`
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,
    /// The size of each data item in the `BufferView`
    #[serde(rename = "byteStride")]
    #[serde(default)]
    pub byte_stride: u32,
    /// The data type of each element
    #[serde(rename = "componentType")]
    pub component_type: AccessorComponentType,
    /// The number of elements within the `BufferView` (N.B. not number of bytes)
    pub count: u32,
    /// The multiplicity of each element
    #[serde(rename = "type")]
    pub component_width: AccessorComponentWidth,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
}

impl_enum! {
    AccessorComponentType {
        I8 = 5120,
        U8 = 5121,
        I16 = 5122,
        U16 = 5123,
        U32 = 5125,
        F32 = 5126,
    }
}

// TODO: Replace with statically typed constants
pub type AccessorComponentWidth = String;

/// [A keyframe animation]
/// (https://github.com/KhronosGroup/glTF/blob/d63b796e6b7f6b084c710b97b048d59d749cb04a/specification/2.0/schema/animation.schema.json)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Animation {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Defines the channels of the animation
    pub channels: Vec<AnimationChannel>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Defines samplers that combine input and output accessors
    pub samplers: Vec<AnimationSampler>,
}

/// Targets an animation's sampler at a node's property
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnimationChannel {
    /// The index of the sampler used to compute the value for the target
    pub sampler: SamplerIndex,
    /// The index of the node and TRS property to target
    pub target: AnimationChannelTarget,
}

/// The index of the node and TRS property that an animation channel targets
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnimationChannelTarget {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The index of the node to target
    pub node: NodeIndex,
    /// The name of the node's TRS property to modify e.g. `"translation"`
    pub path: String,
}

/// Defines a keyframe graph but not its target
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnimationSampler {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The index of the accessor containing keyframe input values (e.g. time)
    pub input: AccessorIndex,
    /// The interpolation algorithm (e.g. `"LINEAR"` or `"STEP"`)
    // TODO: Replace with statically typed constant
    pub interpolation: String,
    /// The index of an accessor containing keyframe output values
    pub output: AccessorIndex,
}

/// [Contains metadata about the glTF asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#asset)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    /// A copyright message suitable for display to credit the content creator
    pub copyright: Option<String>,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Tool that generated this glTF model
    pub generator: Option<String>,
    /// glTF version
    #[serde(default = "asset_version_default")]
    pub version: String,
}

fn asset_version_default() -> String {
    "2.0".to_string()
}

/// [The identifier of the `BufferView` this accessor reads from.
/// Describes the location, type, and size of a binary blob included with the asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#buffer)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Buffer {
    /// The length of the buffer in bytes
    #[serde(default)]
    #[serde(rename = "byteLength")]
    pub byte_length: u32,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Uniform resource locator for the buffer data relative to the .gltf file
    // N.B. the spec says this is not required but I think that is incorrect
    pub uri: String,
}

/// [Represents a subset of a `Buffer`]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#buffers-and-buffer-views)  
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BufferView {
    /// The id of the parent `Buffer`
    pub buffer: BufferIndex,
    /// The length of the buffer view data in bytes
    #[serde(rename = "byteLength")]
    pub byte_length: u32,
    /// Offset into the parent buffer in bytes
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,
    /// The stride in bytes between vertex attributes in this buffer view
    #[serde(default)]
    pub byte_stride: u32,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Optional target the buffer should be bound to
    pub target: Option<BufferTarget>,
}

impl_enum! {
    BufferTarget {
        ArrayBuffer = 34962,
        ElementArrayBuffer = 34963,
    }
}

// TODO: This implementation is rubbish. Replace with enum instead
// and derive (De)Serialize manually. It would be trivial to do so
// if it were not for the `name`, `extension`, and `extra` fields.
/// A camera's projection
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Camera {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Orthographic camera values
    #[serde(rename = "orthographic")]
    pub orthographic: Option<CameraOrthographic>,
    /// Perspective camera values
    #[serde(rename = "perspective")]
    pub perspective: Option<CameraPerspective>,
    /// `"perspective"` or `"orthographic"`
    #[serde(rename = "type")]
    pub ty: String, 
}

/// Values for an orthographic camera
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CameraOrthographic {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The horizontal magnification of the view
    #[serde(default, rename = "xmag")]
    pub x_mag: f32,
    /// The vertical magnification of the view
    #[serde(default, rename = "ymag")]
    pub y_mag: f32,
    /// The distance to the far clipping plane
    #[serde(default, rename = "zfar")]
    pub z_far: f32,
    /// The distance to the near clipping plane
    #[serde(default, rename = "znear")]
    pub z_near: f32,
}

/// Values for a perspective camera
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CameraPerspective {
    /// Aspect ratio of the field of view
    #[serde(default, rename = "aspectRatio")]
    pub aspect_ratio: f32,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The vertical field of view in radians
    #[serde(default, rename = "yfov")]
    pub y_fov: f32,
    /// The distance to the far clipping plane
    #[serde(default, rename = "zfar")]
    pub z_far: f32,
    /// The distance to the near clipping plane
    #[serde(default, rename = "znear")]
    pub z_near: f32,
}

/// Image data used to create a texture
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Image {
    /// The index of the `BufferView` that contains the image
    #[serde(rename = "bufferView")]
    pub buffer_view: Option<BufferViewIndex>,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The image's MIME type
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The uniform resource identifier of the image relative to the .gltf file
    pub uri: Option<String>,
}

/// [Describes the material appearance of a primitive]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#material)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Material {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Defines the metallic-roughness material model from Physically-Based Rendering (PBR) methodology
    #[serde(rename = "pbrMetallicRoughness")]
    pub pbr: MaterialPbr,
    #[serde(rename = "normalTexture")]
    pub normal_texture: MaterialNormalTexture,
    #[serde(rename = "occlusionTexture")]
    pub occlusion_texture: MaterialOcclusionTexture,
    #[serde(rename = "emissiveTexture")]
    pub emissive_texture: TextureInfo,
    #[serde(rename = "emissiveFactor")]
    #[serde(default)]
    pub emissive_factor: [f32; 3],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialPbr {
    /// The base color factor
    #[serde(default = "material_pbr_base_color_factor_default")]
    #[serde(rename = "baseColorFactor")]
    pub base_color_factor: [f32; 4],
    /// The base color texture
    #[serde(rename = "baseColorTexture")]
    pub base_color_texture: TextureInfo,
    /// The metalness of the material
    #[serde(default = "material_pbr_metallic_factor_default")]
    #[serde(rename = "metallicFactor")]
    pub metallic_factor: f32,
    /// The roughness of the material
    #[serde(default = "material_pbr_roughness_factor_default")]
    #[serde(rename = "roughnessFactor")]
    pub roughness_factor: f32,
    /// The metallic-roughness texture
    #[serde(rename = "metallicRoughnessTexture")]
    pub metallic_roughness_texture: TextureInfo,
}

fn material_pbr_base_color_factor_default() -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

fn material_pbr_metallic_factor_default() -> f32 {
    1.0
}

fn material_pbr_roughness_factor_default() -> f32 {
    1.0
}

/// Defines the normal texture of a material
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialNormalTexture {
    /// The index of the texture
    pub index: TextureIndex,
    /// The scalar multiplier applied to each normal vector of the normal texture
    #[serde(default = "material_normal_texture_scale_default")]
    pub scale: f32,
    /// The set index of the texture's `TEXCOORD` attribute
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,
}

fn material_normal_texture_scale_default() -> f32 {
    1.0
}

/// Defines the occlusion texture of a material
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialOcclusionTexture {
    /// The index of the texture
    pub index: TextureIndex,
    /// The scalar multiplier controlling the amount of occlusion applied
    #[serde(default = "material_occlusion_texture_strength_default")]
    pub strength: f32,
    /// The set index of the texture's `TEXCOORD` attribute
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,
}

fn material_occlusion_texture_strength_default() -> f32 {
    1.0
}

/// [A set of primitives to be rendered]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#mesh)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mesh {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Defines the geometry of this mesh to be renderered with a material
    pub primitives: Vec<MeshPrimitive>,
    /// Defines the weights to be applied to the morph targets
    #[serde(default)]
    pub weights: Vec<f32>,
}

/// [Geometry to be rendered with the given material]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#meshprimitive)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MeshPrimitive {
    /// Maps attribute semantic names to the `Accessor`s containing their data
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, AccessorIndex>,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Index of the `Accessor` containing mesh indices
    pub indices: Option<AccessorIndex>,
    /// The index of the material to apply to this primitive when rendering
    pub material: MaterialIndex,
    /// The type of primitives to render
    #[serde(default)]
    pub mode: MeshPrimitiveMode,
    #[serde(default)]
    /// Morph targets
    pub targets: Vec<MeshPrimitiveTarget>,
}

impl_enum! {
    MeshPrimitiveMode {
        Points = 0,
        Lines = 1,
        LineLoop = 2,
        LineStrip = 3,
        Triangles = 4,
        TriangleStrip = 5,
        TriangleFan = 6,
    }
}

/// *Unimplemented*
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MeshPrimitiveTarget;

/// [A single member of the glTF scene hierarchy]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md#scenes)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Node {
    /// The index of the camera referenced by this node
    pub camera: CameraIndex,
    /// The indices of this node's children
    #[serde(default)]
    pub children: Vec<NodeIndex>,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// 4x4 column-major transformation matrix
    #[serde(default = "node_matrix_default")]
    pub matrix: [[f32; 4]; 4],
    /// The indices of the `Mesh` objects in this node
    #[serde(default)]
    pub meshes: Vec<MeshIndex>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The node's unit quaternion rotation `[x, y, z, w]`
    #[serde(default = "node_rotation_default")]
    pub rotation: [f32; 4],
    #[serde(default = "node_scale_default")]
    /// The node's non-uniform scale
    pub scale: [f32; 3],
    #[serde(default)]
    /// The node's translation
    pub translation: [f32; 3],
    /// The index of the skin referenced by this node
    pub skin: SkinIndex,
    /// The weights of the morph target
    pub weights: Vec<f32>,
}

fn node_matrix_default() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn node_rotation_default() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

fn node_scale_default() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

/// [Defines texture sampler properties for filtering and wrapping modes]
/// (https://github.com/KhronosGroup/glTF/blob/d63b796e6b7f6b084c710b97b048d59d749cb04a/specification/2.0/schema/sampler.schema.json)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Sampler {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Magnification filter
    #[serde(default, rename = "magFilter")]
    pub mag_filter: SamplerMagFilter,
    /// Minification filter
    #[serde(default, rename = "minFilter")]
    pub min_filter: SamplerMinFilter,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// s wrapping mode
    #[serde(default, rename = "wrapS")]
    pub wrap_s: SamplerWrappingMode,
    /// t wrapping mode
    #[serde(default, rename = "wrapT")]
    pub wrap_t: SamplerWrappingMode,
}

impl_enum! {
    SamplerMagFilter {
        Nearest = 9728,
        Linear = 9729,
    }
}

impl_enum! {
    SamplerMinFilter {
        Nearest = 9728,
        Linear = 9729,
        NearestMipmapNearest = 9984,
        LinearMipmapNearest = 9985,
        NearestMipmapLinear = 9986,
        LinearMipmapLinear = 9987,
    }
}

impl_enum! {
    SamplerWrappingMode {
        ClampToEdge = 33071,
        MirroredRepeat = 33648,
        Repeat = 10497,
    }
}

/// [A set of visual objects to render](https://github.com/KhronosGroup/glTF/tree/2.0/specification/2.0#scenes)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The indices of each root `Node` in this scene
    #[serde(default)]
    pub nodes: Vec<NodeIndex>,
}

/// [Joints and matrices defining a skin](https://github.com/KhronosGroup/glTF/blob/d63b796e6b7f6b084c710b97b048d59d749cb04a/specification/2.0/schema/skin.schema.json)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Skin {
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// The index of the accessor containing the 4x4 inverse-bind matrices
    #[serde(rename = "inverseBindMatrices")]
    pub inverse_bind_matrices: Option<AccessorIndex>,
    /// Indices of skeleton nodes used as joints in this skin
    pub joints: Vec<NodeIndex>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The index of the node used as a skeleton root
    pub skeleton: Option<NodeIndex>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Texture {
    /// Texel data type
    #[serde(default, rename = "type")]
    pub data_type: TextureDataType,
    /// Optional data targeting official extensions
    pub extensions: Extensions,
    /// Optional application specific data
    pub extras: Extras,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The texture format
    #[serde(default)]
    pub format: TextureFormat,
    /// The texture internal format
    #[serde(default, rename = "internalFormat")]
    pub internal_format: TextureFormat,
    /// The index of the sampler used by this texture
    pub sampler: SamplerIndex,
    /// The index of the image used by this texture
    pub source: ImageIndex,
    /// The target the texture should be bound to
    #[serde(default)]
    pub target: TextureTarget,
}

impl_enum! {
    TextureDataType {
        U8 = 5121,
        U16_5_6_5 = 33635,
        U16_4_4_4_4 = 32819,
        U16_5_5_5_1 = 32820,
    }
}

impl_enum! {
    TextureFormat {
        Alpha = 6406,
        Rgb = 6407,
        Rgba = 6408,
        Luminance = 6409,
        LuminanceAlpha = 6410,
    }
}

impl_enum! {
    TextureTarget {
        Texture2d = 3553,
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
/// Reference to a `Texture`
pub struct TextureInfo {
    /// The index of the texture
    pub index: TextureIndex,
    /// The set index of the texture's `TEXCOORD` attribute
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,
}

impl Default for MeshPrimitiveMode {
    fn default() -> Self {
        MeshPrimitiveMode::Triangles
    }
}

impl Default for SamplerMagFilter {
    fn default() -> Self {
        SamplerMagFilter::Linear
    }
}

impl Default for SamplerMinFilter {
    fn default() -> Self {
        SamplerMinFilter::NearestMipmapLinear
    }
}

impl Default for SamplerWrappingMode {
    fn default() -> Self {
        SamplerWrappingMode::Repeat
    }
}

impl Default for TextureDataType {
    fn default() -> Self {
        TextureDataType::U8
    }
}

impl Default for TextureFormat {
    fn default() -> Self {
        TextureFormat::Rgba
    }
}

impl Default for TextureTarget {
    fn default() -> Self {
        TextureTarget::Texture2d
    }
}

impl Gltf {
    pub fn from_file<P>(path: P) -> Result<Self, LoadError>
        where P: AsRef<std::path::Path>
    {
        use std::io::Read;
        let mut file = std::fs::File::open(path).map_err(LoadError::Io)?;
        let mut json = String::new();
        let _ = file.read_to_string(&mut json).map_err(LoadError::Io)?;
        serde_json::from_str(&json).map_err(|err| LoadError::De(err))
    }

    pub fn accessor(&self, index: AccessorIndex) -> Option<&Accessor> {
        self.accessors.get(index.0 as usize)
    }
    
    pub fn asset(&self) -> &Asset {
        &self.asset
    }
    
    pub fn buffer(&self, index: BufferIndex) -> Option<&Buffer> {
        self.buffers.get(index.0 as usize)
    }
    
    pub fn buffer_view(&self, index: BufferViewIndex) -> Option<&BufferView> {
        self.buffer_views.get(index.0 as usize)
    }

    pub fn extensions_used(&self) -> &[String] {
        &self.extensions_used[..]
    }

    pub fn extensions_required(&self) -> &[String] {
        &self.extensions_required[..]
    }
    
    pub fn camera(&self, index: CameraIndex) -> Option<&Camera> {
        self.cameras.get(index.0 as usize)
    }

    pub fn image(&self, index: ImageIndex) -> Option<&Image> {
        self.images.get(index.0 as usize)
    }

    pub fn material(&self, index: MaterialIndex) -> Option<&Material> {
        self.materials.get(index.0 as usize)
    }
    
    pub fn mesh(&self, index: MeshIndex) -> Option<&Mesh> {
        self.meshes.get(index.0 as usize)
    }
    
    pub fn node(&self, index: NodeIndex) -> Option<&Node> {
        self.nodes.get(index.0 as usize)
    }

    pub fn sampler(&self, index: SamplerIndex) -> Option<&Sampler> {
        self.samplers.get(index.0 as usize)
    }
    
    pub fn scene(&self, index: SceneIndex) -> Option<&Scene> {
        self.scenes.get(index.0 as usize)
    }

    pub fn skin(&self, index: SkinIndex) -> Option<&Skin> {
        self.skins.get(index.0 as usize)
    }

    pub fn texture(&self, index: TextureIndex) -> Option<&Texture> {
        self.textures.get(index.0 as usize)
    }
}
