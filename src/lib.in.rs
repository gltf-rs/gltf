
/*
 * Copyright (c) 2016 David Harvey-Macaulay <alteous@outlook.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde_json::from_str;

pub use serde_json::value::{Map, Value};

/// Untyped glTF top-level object identifier
pub type Id = String;

/// Helper trait for looking up top-level objects by their identifier
pub trait Find<T> {
    /// Attempts to find the object of type `T` with identifer `id`
    fn find(&self, id: &str) -> Option<&T>;
}

/// Run time error encountered when loading a glTF asset
#[derive(Debug)]
pub enum Error {
    /// Standard input / output error
    Io(std::io::Error),
    /// Failure when parsing a .gltf metadata file
    Parse(serde_json::error::Error),
}

/// [The root object for a glTF asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#gltf)
#[derive(Debug, Deserialize, Serialize)]
pub struct Gltf {
    #[serde(default)]
    pub accessors: Map<Id, Accessor>,
    #[serde(default)]
    pub asset: Asset,
    #[serde(default)]
    pub buffers: Map<Id, Buffer>,
    #[serde(default)]
    #[serde(rename = "bufferViews")]
    pub buffer_views: Map<Id, BufferView>,
    #[serde(default)]
    pub materials: Map<Id, Material>,
    #[serde(default)]
    pub meshes: Map<Id, Mesh>,
    #[serde(default)]
    pub programs: Map<Id, Program>,
    #[serde(default)]
    pub shaders: Map<Id, Shader>,
    #[serde(default)]
    pub techniques: Map<Id, Technique>,
    // Incomplete
}

/// [Defines a method for retrieving data from within a `BufferView`]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#accessors)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Accessor {
    /// The identifier of the `BufferView` this accessor reads from.
    #[serde(rename = "bufferView")]
    pub buffer_view: Id,
    /// Where the data items begin from in the `BufferView`
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,
    /// The size of each data item in the `BufferView`
    #[serde(rename = "byteStride")]
    #[serde(default)]
    pub byte_stride: u32,
    /// Possible values: `GL_BYTE`, `GL_FLOAT`, `GL_SHORT`, `GL_UNSIGNED_BYTE`, or `GL_UNSIGNED_SHORT`
    #[serde(rename = "componentType")]
    pub component_type: u32,
    /// The number of attributes within the `BufferView` (N.B. not number of bytes)
    pub count: u32,
    /// Possible values: `"SCALAR"`, `"VEC2"`, `"VEC3"`, `"VEC4"`, `"MAT2"`, `"MAT3"`, or `"MAT4"`
    #[serde(rename = "type")]
    pub component_width: String,
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// Maximum value of each component in the attribute
    pub max: Option<Vec<f32>>,
    /// Minimum value of each component in the attribtue
    pub min: Option<Vec<f32>>,
}

/// [Contains metadata about the glTF asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#asset)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    /// A copyright message suitable for display to credit the content creator
    pub copyright: Option<String>,
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// Tool that generated this glTF model
    pub generator: Option<String>,
    /// Specifies if shaders were generated with pre-multiplied alpha
    #[serde(default)]
    #[serde(rename = "premultipliedAlpha")]
    pub pre_multiplied_alpha: bool,
    /// Specifies the target rendering API and version
    pub profile: Option<AssetProfile>,
    /// glTF version
    pub version: String,
}

/// [Specifies the target rendering API and version]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#assetprofile-1)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssetProfile {
    /// Specifies the target rendering API
    #[serde(default = "asset_profile_api_default")]
    pub api: String,
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// Specifies the target rendering API version
    #[serde(default = "asset_profile_version_default")]
    pub version: String,
}

fn asset_profile_api_default() -> String {
    "WebGL".to_string()
}

fn asset_profile_version_default() -> String {
    "1.0.3".to_string()
}

/// The identifier of the `BufferView` this accessor reads from.
/// [Describes the location, type, and size of a binary blob included with the asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#buffer)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Buffer {
    /// The length of the buffer in bytes
    #[serde(default)]
    #[serde(rename = "byteLength")]
    pub byte_length: u32,
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// XMLHttpRequest `responseType`
    #[serde(default = "buffer_response_type_default")]
    #[serde(rename = "type")]
    pub response_type: String,
    /// Uniform resource locator for the buffer data
    pub uri: String,
}

fn buffer_response_type_default() -> String {
    "arraybuffer".to_string()
}

/// [Represents a subset of a `Buffer`]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#buffers-and-buffer-views)  
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BufferView {
    /// The identifier of the parent `Buffer`
    pub buffer: Id,
    /// The length of the buffer view in bytes
    #[serde(default)]
    #[serde(rename = "byteLength")]
    pub byte_length: u32,
    /// Offset into the buffer in bytes
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// Optional target the buffer should be bound to (for example
    /// `GL_ARRAY_BUFFER` or `GL_ELEMENT_ARRAY_BUFFER`)
    pub target: Option<u32>,
}
/// [Describes the material appearance of a primitive]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#material)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Material {
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// ID of the shading technique to be used
    pub technique: Option<Id>,
    /// Parameter values
    #[serde(default)]
    pub values: Map<String, Value>,
}

/// [A set of primitives to be rendered]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#mesh)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mesh {
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    #[serde(default)]
    pub primitives: Vec<MeshPrimitive>,
}

/// [Geometry to be rendered with the given material]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#meshprimitive)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MeshPrimitive {
    /// Mapping of attribute names to `Accessor` IDs
    #[serde(default)]
    pub attributes: Map<String, Id>,
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// Optional ID of the `Accessor` containing index data
    pub indices: Option<Id>,
    /// ID of the material to apply to this primitive when rendering
    pub material: Id,
    /// The type of primitives to render (for example `GL_TRIANGLES`)
    #[serde(default = "mesh_primitive_mode_default")]
    pub mode: u32,
}

fn mesh_primitive_mode_default() -> u32 {
    gl::TRIANGLES
}

/// [A single member of the glTF scene hierarchy]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#scenes)
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Node {
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// The IDs of the `Mesh` objects in this node
    pub meshes: Option<Vec<Id>>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    // Incomplete
}

/// [Describes a GLSL shader program]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#programs)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Program {
    /// Vertex attribute bindings (e.g. `"u_ModelView"`) that will be passed to the shader
    #[serde(default)]
    pub attributes: Vec<String>,
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// ID of the fragment shader component
    #[serde(rename = "fragmentShader")]
    pub fragment_shader: String,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// ID of the vertex shader component
    #[serde(rename = "vertexShader")]
    pub vertex_shader: String,
}

/// [Describes a GLSL shader component]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#shaders)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Shader {
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    /// The shader stage (for example `GL_VERTEX_SHADER` or `GL_FRAGMENT_SHADER`)
    #[serde(rename = "type")]
    pub type_id: u32,
    /// Uniform resource identifier of the GLSL source code
    pub uri: String,
}

/// [Describes the shading technqiue used for a material]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#technique)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Technique {
    /// Maps GLSL attribute names to technique parameter IDs
    #[serde(default)]
    pub attributes: Map<String, String>,
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// Optional user-defined name for this object
    pub name: Option<String>,
    #[serde(default)]
    pub parameters: Map<String, TechniqueParameter>,
    /// ID of the GLSL shader program to render with
    pub program: Id,
    /// Fixed-function rendering states
    #[serde(default)]
    pub states: TechniqueStates,
    /// Maps uniform names to technqiue parameter IDs
    #[serde(default)]
    pub uniforms: Map<String, String>,
}

/// [Describes an attribute or uniform input to a `Technique`]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#techniqueparameters-1)
/// If `semantic` is not `None` then this parameter describes a [built-in uniform value]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#semantics)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TechniqueParameter {
    /// Defines the number of elements if the parameter is an array
    pub count: Option<u32>,
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// ID of the `Node` whose transform is used as the parameter's value
    pub node: Option<Id>,
    /// `"MODELVIEW"`, `"PROJECTION"`, etc.
    pub semantic: Option<String>,
    /// The data type (for example `GL_FLOAT`, or `GL_FLOAT_VEC4`)
    #[serde(rename = "type")]
    pub type_id: u32,
    /// The value of the parameter
    pub value: Option<Value>,
}

/// [Optional arguments to OpenGL state functions]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#render-states)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TechniqueStateFunctions {
    /// Arguments `[red, green, blue, alpha]` for `glBlendColor()`
    #[serde(default)]
    #[serde(rename = "blendColor")]
    pub blend_color: [f64; 4],
    /// Arguments `[mode_rgb, mode_alpha]` for `glBlendEquationSeparate()`
    #[serde(default = "technique_state_functions_blend_equation_default")]
    #[serde(rename = "blendEquationSeparate")]
    pub blend_equation: [u32; 2],
    /// Arguments `[src_rgb, dst_rgb, src_alpha, dst_alpha]` for `glBlendFuncSeparate()`
    #[serde(default = "technique_state_functions_blend_function_default")]
    #[serde(rename = "blendFuncSeparate")]
    pub blend_function: [u32; 4],
    /// Arguments `[red, green, blue, alpha]` for `glColorMask()`
    #[serde(default = "technique_state_functions_color_mask_default")]
    #[serde(rename = "colorMask")]
    pub color_mask: [bool; 4],
    /// Argument `[mode]` for `glCullFace()`
    #[serde(default = "technique_state_functions_cull_face_default")]
    #[serde(rename = "cullFace")]
    pub cull_face: [u32; 1],
    /// Argument `[func]` for `glDepthFunc()`
    #[serde(default = "technique_state_functions_depth_func_default")]
    #[serde(rename = "depthFunc")]
    pub depth_function: [u32; 1],
    /// Argument `[flag]` for `glDepthMask()`
    #[serde(default = "technique_state_functions_depth_mask_default")]
    #[serde(rename = "depthMask")]
    pub depth_mask: [bool; 1],
    /// Arguments `[z_near, z_far]` for `glDepthRange()`
    #[serde(default = "technique_state_functions_depth_range_default")]
    #[serde(rename = "depthRange")]
    pub depth_range: [f64; 2],
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// Argument `[mode]` for `glFrontFace()`
    #[serde(default = "technique_state_functions_front_face_default")]
    #[serde(rename = "frontFace")]
    pub front_face: [u32; 1],
    /// Argument `[width]` for `glLineWidth()`
    #[serde(default = "technique_state_functions_line_width_default")]
    #[serde(rename = "lineWidth")]
    pub line_width: [f32; 1],
    /// Arguments `[factor, units]` for `glPolygonOffset()`
    #[serde(default)]
    #[serde(rename = "polygonOffset")]
    pub polygon_offset: [f32; 2],
    /// Arguments `[x, y, width, height]` for `glScissor()`
    #[serde(default)]
    pub scissor: [i32; 4],
}

fn technique_state_functions_blend_equation_default() -> [u32; 2] {
    [gl::FUNC_ADD, gl::FUNC_ADD]
}

fn technique_state_functions_blend_function_default() -> [u32; 4] {
    [gl::ONE, gl::ZERO, gl::ONE, gl::ZERO]
}

fn technique_state_functions_color_mask_default() -> [bool; 4] {
    [true, true, true, true]
}

fn technique_state_functions_cull_face_default() -> [u32; 1] {
    [gl::BACK]
}

fn technique_state_functions_depth_func_default() -> [u32; 1] {
    [gl::LESS]
}

fn technique_state_functions_depth_mask_default() -> [bool; 1] {
    [true]
}

fn technique_state_functions_depth_range_default() -> [f64; 2] {
    [0.0, 1.0]
}

fn technique_state_functions_front_face_default() -> [u32; 1] {
    [gl::CCW]
}

fn technique_state_functions_line_width_default() -> [f32; 1] {
    [1.0]
}

/// [Required OpenGL render states to be enabled]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#render-states)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TechniqueStates {
    /// OpenGL states to be enabled
    #[serde(default)]
    pub enable: Vec<u32>,
    /// Optional data targeting official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional application specific data
    pub extras: Option<Map<String, Value>>,
    /// Arguments for fixed-function rendering state functions
    pub functions: Option<TechniqueStateFunctions>, 
}

impl Gltf {
    /// Loads a glTF asset
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let gltf = gltf::Gltf::new("./examples/box/Box.gltf")
    ///     .expect("Error loading glTF asset");
    /// ```
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = try!(File::open(path));
        let mut json = String::new();
        try!(file.read_to_string(&mut json));
        from_str(&json)
            .map_err(|cause| Error::Parse(cause))
    }

    /// Looks up a top-level object by its identifier
    ///
    /// # Examples
    ///
    /// Finding a buffer view:
    ///
    /// ```
    /// let gltf = gltf::Gltf::new("./examples/box/Box.gltf").unwrap();
    /// let buffer_view = gltf
    ///     .find::<gltf::BufferView>("bufferView_29")
    ///     .expect("Buffer view not found");
    /// ```
    pub fn find<T>(&self, id: &str) -> Option<&T>
        where Self: Find<T>
    {
        (self as &Find<T>).find(id)
    }
}

macro_rules! impl_find {
    ($ident:ident, $ty:ty) => (
        impl Find<$ty> for Gltf {
            fn find(&self, id: &str) -> Option<&$ty> {
                self.$ident
                    .iter()
                    .find(|&(entry_id, _)| entry_id == id)
                    .map(|(_, entry)| entry)
            }
        }
    )
}

impl_find!(accessors, Accessor);
impl_find!(buffers, Buffer);
impl_find!(buffer_views, BufferView);
impl_find!(materials, Material);
impl_find!(meshes, Mesh);
impl_find!(programs, Program);
impl_find!(shaders, Shader);
impl_find!(techniques, Technique);

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::Parse(err)
    }
}

