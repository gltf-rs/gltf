
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

#![feature(proc_macro, custom_attribute)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde_json::from_str;
pub use serde_json::value::{Map, Value};

/// Helper trait for looking up objects by their identifier in a glTF asset.
pub trait Find<T> {
    /// Attempts to find the object with the given type and identifer.
    fn find(&self, id: &str) -> Option<&T>;
}

/// Run time error encountered when loading a glTF asset.
#[derive(Debug)]
pub enum Error {
    /// Standard input / output error
    Io(std::io::Error),
    /// Failure parsing a .gltf metadata file
    Parse(serde_json::error::Error),
}

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

/// [Defines a method for retrieving data from within a `BufferView`]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#accessors)
#[derive(Debug, Deserialize, Serialize)]
pub struct Accessor {
    /// The identifier of the `BufferView` this accessor reads from.
    #[serde(rename = "bufferView")]
    pub buffer_view: String,
    /// Where the data items begin from in the `BufferView`
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,
    /// The size of each data item in the `BufferView`
    #[serde(rename = "byteStride")]
    pub byte_stride: u32,
    /// e.g. "BYTE", "FLOAT", or "UNSIGNED_SHORT"
    #[serde(rename = "componentType")]
    pub component_type: u32,
    /// The number of attributes within the `BufferView` (N.B. not number of bytes)
    pub count: u32,
    /// e.g. "SCALAR", "VEC3", or "MAT4"
    #[serde(rename = "type")]
    pub data_type: String,
    /// Optional data for official extensions
    pub extensions: Option<Map<String, Value>>,
    /// Optional data for custom extensions
    pub extras: Option<Map<String, Value>>,
}

/// [Describes the location, type, and size of a binary blob included with the asset]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#buffers-and-buffer-views)
#[derive(Debug, Deserialize, Serialize)]
pub struct Buffer {
    #[serde(rename = "byteLength")]
    pub byte_length: u32,
    #[serde(rename = "type")]
    pub type_id: String,
    pub uri: String,
}

/// [Describes a subset of a `Buffer`]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#buffers-and-buffer-views)  
#[derive(Debug, Deserialize, Serialize)]
pub struct BufferView {
    /// The identifier of the parent `Buffer`
    pub buffer: String,
    /// The number of bytes in the subset
    #[serde(rename = "byteLength")]
    pub byte_length: u32,
    /// Where the subset starts from, measured in bytes
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,
    /// e.g. `GL_ARRAY_BUFFER` or `GL_ELEMENT_ARRAY_BUFFER`
    pub target: u32,
}

/// [Optional arguments to OpenGL state functions]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#render-states)
#[derive(Debug, Deserialize, Serialize)]
pub struct StateFunctions {
    #[serde(rename = "blendColor")]
    pub blend_color: Option<[f64; 4]>,
    #[serde(rename = "blendEquationSeparate")]
    pub blend_equation: Option<[u32; 2]>,
    #[serde(rename = "blendFuncSeparate")]
    pub blend_function: Option<[bool; 4]>,
    #[serde(rename = "colorMask")]
    pub color_mask: Option<[bool; 4]>,
    #[serde(rename = "cullFace")]
    pub cull_face: Option<u32>,
    #[serde(rename = "depthFunc")]
    pub depth_function: Option<u32>,
    #[serde(rename = "depthRange")]
    pub depth_range: Option<[f32; 2]>,
    #[serde(rename = "frontFace")]
    pub front_face: Option<u32>,
    #[serde(rename = "lineWidth")]
    pub line_width: Option<f32>,
    #[serde(rename = "polygonOffset")]
    pub polygon_offset: Option<[f32; 2]>,
    #[serde(rename = "scissor")]
    pub scissor: Option<[u32; 4]>,
}

/// [Required OpenGL render states to be enabled]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#render-states)
#[derive(Debug, Deserialize, Serialize)]
pub struct States {
    pub enable: Vec<u32>,
    pub functions: StateFunctions, 
}

/// [Describes a shading technique with parameterized values]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#materials-and-shading)
#[derive(Debug, Deserialize, Serialize)]
pub struct Material {
    pub name: String,
    pub technique: String,
    pub values: Map<String, Value>,
}

/// [Describes one instance of renderable geometry]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#geometry-and-meshes)
#[derive(Debug, Deserialize, Serialize)]
pub struct Mesh {
    pub name: String,
    pub primitives: Vec<Primitive>,
}

/// [A single member of the glTF scene hierarchy]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#scenes)
#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub meshes: Vec<String>,
    pub name: String,
}

/// [Describes a shader input parameter]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#parameters)
///
/// If `semantic` is not `None` then this parameter describes a [built-in uniform value]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#semantics)
/// (e.g. `"MODELVIEW"`).
#[derive(Debug, Deserialize, Serialize)]
pub struct Parameter {
    /// `"MODELVIEW"`, `"PROJECTION"`, etc.
    pub semantic: Option<String>,
    /// `GL_FLOAT`, `GL_FLOAT_VEC4` etc.
    #[serde(rename = "type")]
    pub type_id: u32,
}

/// [Describes a GLSL shader program]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#programs)
#[derive(Debug, Deserialize, Serialize)]
pub struct Program {
    /// Vertex attribute bindings (e.g. `"u_ModelView"`) that will be passed to the shader
    pub attributes: Vec<String>,
    /// ID of the fragment shader component
    #[serde(rename = "fragmentShader")]
    pub fragment_shader: String,
    /// ID of the vertex shader component
    #[serde(rename = "vertexShader")]
    pub vertex_shader: String,
}

/// [Describes a renderable subset of a mesh]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#meshes)
#[derive(Debug, Deserialize, Serialize)]
pub struct Primitive {
    pub attributes: Map<String, String>,
    pub indices: Option<String>,
    pub material: String,
    pub mode: u32,
}

/// [Describes a GLSL shader component]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#shaders)
#[derive(Debug, Deserialize, Serialize)]
pub struct Shader {
    /// e.g. `GL_VERTEX_SHADER` or `GL_FRAGMENT_SHADER`
    #[serde(rename = "type")]
    pub type_id: u32,
    /// Uniform resource identifier
    pub uri: String,
}

/// [Describes the shading used for a material]
/// (https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#techniques)
#[derive(Debug, Deserialize, Serialize)]
pub struct Technique {
    /// Maps vertex attribute bindings to their definitions
    /// (e.g. `("a_Position", "position")`)
    pub attributes: Map<String, String>,
    /// Maps uniform bindings to their definitions
    /// (e.g. `("u_ProjectionMatrix", "projectionMatrix")`)
    pub uniforms: Map<String, String>,
    pub parameters: Map<String, Parameter>,
    /// ID of the GLSL shader program to render with
    pub program: String,
}

/// 'Raw' glTF data structure that closely matches the structure of a .gltf file
#[derive(Debug, Deserialize, Serialize)]
pub struct Gltf {
    pub accessors: Map<String, Accessor>,
    #[serde(rename = "buffers")]
    pub buffers: Map<String, Buffer>,
    #[serde(rename = "bufferViews")]
    pub buffer_views: Map<String, BufferView>,
    pub materials: Map<String, Material>,
    pub meshes: Map<String, Mesh>,
    pub programs: Map<String, Program>,
    pub shaders: Map<String, Shader>,
    pub techniques: Map<String, Technique>,
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

impl Gltf {
    /// Loads a glTF asset from the file system.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let gltf = Gltf::new("foo.gltf").expect("Error loading glTF asset");
    /// ```
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = try!(File::open(path));
        let mut json = String::new();
        try!(file.read_to_string(&mut json));
        match from_str(&json) {
            Ok(gltf) => Ok(gltf),
            Err(cause) => Err(Error::Parse(cause)),
        }
    }

    /// Looks up a top-level object by its identifier.
    ///
    /// # Examples
    ///
    /// Finding a buffer view:
    ///
    /// ```
    /// use gltf::{BufferView, Gltf};
    /// let gltf = Gltf::new("foo.gltf").unwrap();
    /// let buffer_view = gltf
    ///     .find::<BufferView>("bufferView-123")
    ///     .expect("Buffer view not found");
    /// ```
    pub fn find<T>(&self, id: &str) -> Option<&T>
        where Self: Find<T>
    {
        (self as &Find<T>).find(id)
    }
}

