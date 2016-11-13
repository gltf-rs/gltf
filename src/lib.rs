
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

pub trait Find<T> {
    fn find(&self, id: &str) -> Option<&T>;
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Accessor {
    #[serde(rename = "bufferView")]
    pub buffer_view: String,
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,
    #[serde(rename = "byteStride")]
    pub byte_stride: u32,
    #[serde(rename = "componentType")]
    pub component_type: u32,
    pub count: u32,
    #[serde(rename = "type")]
    pub data_type: String,
    pub extensions: Option<Map<String, Value>>,
    pub extras: Option<Map<String, Value>>,
    pub max: Option<[f32; 3]>,
    pub min: Option<[f32; 3]>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Buffer {
    #[serde(rename = "byteLength")]
    pub byte_length: u32,
    #[serde(rename = "type")]
    pub type_id: String,
    pub uri: String,
}
    
#[derive(Debug, Deserialize, Serialize)]
pub struct BufferView {
    pub buffer: String,
    #[serde(rename = "byteLength")]
    pub byte_length: u32,
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,
    pub target: u32,
}

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

#[derive(Debug, Deserialize, Serialize)]
pub struct States {
    pub enable: Vec<u32>,
    pub functions: StateFunctions, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Material {
    pub name: String,
    pub technique: String,
    pub values: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mesh {
    pub name: String,
    pub primitives: Vec<Primitive>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub meshes: Vec<String>,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameter {
    // "PROJECTION" etc.
    pub semantic: Option<String>,
    // gl::FLOAT_VEC4 etc.
    #[serde(rename = "type")]
    pub type_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Program {
    pub attributes: Vec<String>,
    #[serde(rename = "fragmentShader")]
    pub fragment_shader: String,
    #[serde(rename = "vertexShader")]
    pub vertex_shader: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Primitive {
    pub attributes: Map<String, String>,
    pub indices: Option<String>,
    pub material: String,
    pub mode: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Shader {
    #[serde(rename = "type")]
    pub type_id: u32,
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Technique {
    pub attributes: Map<String, String>,
    pub uniforms: Map<String, String>,
    pub parameters: Map<String, Parameter>,
    pub program: String,
}

/// 'Raw' glTF data structure that closely matches the structure of a .gltf file.
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
    /// Loads a glTF asset from the host file system.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let raw_gltf = gltf::raw::new("foo.gltf").expect("Parse error");
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

