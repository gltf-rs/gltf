// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use traits::{Extensions, Extras};

enum_number! {
    ParameterType {
        Byte = 5120,
        UnsignedByte = 5121,
        Short = 5122,
        UnsignedShort = 5123,
        Integer = 5124,
        UnsignedInteger = 5125,
        Float = 5126,
        Double = 5127,
        FloatVec2 = 35664,
        FloatVec3 = 35665,
        FloatVec4 = 35666,
        IntVec2 = 35667,
        IntVec3 = 35668,
        IntVec4 = 35669,
        Bool = 35670,
        BoolVec2 = 35671,
        BoolVec3 = 35672,
        BoolVec4 = 35673,
        FloatMat2 = 35674,
        FloatMat3 = 35675,
        FloatMat4 = 35676,
        Sampler2d = 35678,
    }
}

impl Default for ParameterType {
    fn default() -> ParameterType {
        ParameterType::Byte
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameter<E: Extensions, X: Extras> {
    /// When defined, the parameter is an array of count elements of the
    /// specified type
    pub count: Option<u32>,

    /// The id of the node whose transform is used as the parameter's value.
    ///
    /// When this is defined, type must be 35676 (FLOAT_MAT4), therefore, when
    /// the semantic is "MODELINVERSETRANSPOSE", "MODELVIEWINVERSETRANSPOSE", or
    /// "VIEWPORT", the node property can't be defined.
    pub node: Option<String>,

    /// The datatype.
    #[serde(rename = "type")]
    #[serde(default)]
    pub kind: ParameterType,

    /// Identifies a parameter with a well-known meaning.
    ///
    /// Uniform semantics include:
    /// "LOCAL" (FLOAT_MAT4)
    /// "MODEL" (FLOAT_MAT4)
    /// "VIEW" (FLOAT_MAT4)
    /// "PROJECTION" (FLOAT_MAT4)
    /// "MODELVIEW" (FLOAT_MAT4)
    /// "MODELVIEWPROJECTION" (FLOAT_MAT4)
    /// "MODELINVERSE" (FLOAT_MAT4)
    /// "VIEWINVERSE" (FLOAT_MAT4)
    /// "PROJECTIONINVERSE" (FLOAT_MAT4)
    /// "MODELVIEWINVERSE" (FLOAT_MAT4)
    /// "MODELVIEWPROJECTIONINVERSE" (FLOAT_MAT4)
    /// "MODELINVERSETRANSPOSE" (FLOAT_MAT3)
    /// "MODELVIEWINVERSETRANSPOSE" (FLOAT_MAT3)
    /// "VIEWPORT" (FLOAT_VEC4)
    /// "JOINTMATRIX" (FLOAT_MAT4)
    ///
    /// Attribute semantics include:
    /// "POSITION"
    /// "NORMAL"
    /// "TEXCOORD"
    /// "COLOR"
    /// "JOINT"
    /// "WEIGHT"
    ///
    /// Attribute semantics can be of the form [semantic]_[set_index] for
    /// example "TEXCOORD_0".
    pub semantic: Option<String>,

    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: <E as Extensions>::TechniqueParameter,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <X as Extras>::TechniqueParameter,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Function<E: Extensions, X: Extras> {
    /// Floating-point values passed to blendColor(). [red, green, blue, alpha]
    #[serde(rename = "blendColor")]
    #[serde(default = "function_blend_color_default")]
    pub blend_color: [f32; 4],

    /// Integer values passed to blendEquationSeparate(). [rgb, alpha].
    ///
    /// Valid values correspond to WebGL enums:
    /// 32774 (FUNC_ADD)
    /// 32778 (FUNC_SUBTRACT)
    /// 32779 (FUNC_REVERSE_SUBTRACT)
    #[serde(rename = "blendEquationSeparate")]
    #[serde(default = "function_blend_equation_separate_default")]
    pub blend_equation_separate: [u32; 2],

    /// Integer values passed to blendFuncSeparate().
    ///
    /// [srcRGB, dstRGB, srcAlpha, dstAlpha].
    ///
    /// Valid values correspond to WebGL enums:
    /// 0 (ZERO)
    /// 1 (ONE)
    /// 768 (SRC_COLOR)
    /// 769 (ONE_MINUS_SRC_COLOR)
    /// 774 (DST_COLOR)
    /// 775 (ONE_MINUS_DST_COLOR)
    /// 770 (SRC_ALPHA)
    /// 771 (ONE_MINUS_SRC_ALPHA)
    /// 772 (DST_ALPHA)
    /// 773 (ONE_MINUS_DST_ALPHA)
    /// 32769 (CONSTANT_COLOR)
    /// 32770 (ONE_MINUS_CONSTANT_COLOR)
    /// 32771 (CONSTANT_ALPHA)
    /// 32772 (ONE_MINUS_CONSTANT_ALPHA)
    /// 776 (SRC_ALPHA_SATURATE).
    #[serde(rename = "blendFuncSeparate")]
    #[serde(default = "function_blend_func_separate_default")]
    pub blend_func_separate: [u32; 4],

    /// Boolean values passed to colorMask(). [red, green, blue, alpha].
    #[serde(rename = "colorMask")]
    #[serde(default = "function_color_mask_default")]
    pub color_mask: [bool; 4],

    /// Integer value passed to cullFace().
    ///
    /// Valid values correspond to WebGL enums:
    /// 1028 (FRONT)
    /// 1029 (BACK)
    /// 1032 (FRONT_AND_BACK)
    #[serde(rename = "cullFace")]
    #[serde(default = "function_cull_face_default")]
    pub cull_face: [u32; 1],

    /// Integer values passed to depthFunc().
    ///
    /// Valid values correspond to WebGL enums:
    /// 512 (NEVER)
    /// 513 (LESS)
    /// 515 (LEQUAL)
    /// 514 (EQUAL)
    /// 516 (GREATER)
    /// 517 (NOTEQUAL)
    /// 518 (GEQUAL)
    /// 519 (ALWAYS)
    #[serde(rename = "depthFunc")]
    #[serde(default = "function_depth_func_default")]
    pub depth_func: [u32; 1],

    /// Boolean value passed to depthMask().
    #[serde(rename = "depthMask")]
    #[serde(default = "function_depth_mask_default")]
    pub depth_mask: [bool; 1],

    /// Floating-point values passed to depthRange(). [zNear, zFar]
    #[serde(rename = "depthRange")]
    #[serde(default = "function_depth_range_default")]
    pub depth_range: [f32; 2],

    /// Integer value passed to frontFace().
    ///
    /// Valid values correspond to WebGL enums: 2304 (CW) and 2305 (CCW).
    #[serde(rename = "frontFace")]
    #[serde(default = "function_front_face_default")]
    pub front_face: [u32; 1],

    /// Floating-point value passed to lineWidth().
    #[serde(rename = "lineWidth")]
    #[serde(default = "function_line_width_default")]
    pub line_width: [u32; 1],

    /// Floating-point value passed to polygonOffset(). [factor, units]
    #[serde(rename = "polygonOffset")]
    #[serde(default = "function_polygon_offset_default")]
    pub polygon_offset: [u32; 2],

    /// Floating-point value passed to scissor().
    ///
    /// [x, y, width, height].
    ///
    /// The default is the dimensions of the canvas when the WebGL context is
    /// created. width and height must be greater than zero.
    #[serde(default = "function_scissor_default")]
    pub scissor: [f32; 4],

    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: <E as Extensions>::TechniqueFunction,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <X as Extras>::TechniqueFunction,
}

fn function_blend_color_default() -> [f32; 4] {
    [0.0, 0.0, 0.0, 0.0]
}

fn function_blend_equation_separate_default() -> [u32; 2] {
    [32774, 32774]
}

fn function_blend_func_separate_default() -> [u32; 4] {
    [1, 0, 1, 0]
}

fn function_color_mask_default() -> [bool; 4] {
    [true, true, true, true]
}

fn function_cull_face_default() -> [u32; 1] {
    [1029]
}

fn function_depth_func_default() -> [u32; 1] {
    [513]
}

fn function_depth_mask_default() -> [bool; 1] {
    [true]
}

fn function_depth_range_default() -> [f32; 2] {
    [0.0, 1.0]
}

fn function_front_face_default() -> [u32; 1] {
    [2305]
}

fn function_line_width_default() -> [u32; 1] {
    [1]
}

fn function_polygon_offset_default() -> [u32; 2] {
    [0, 0]
}

fn function_scissor_default() -> [f32; 4] {
    [0.0, 0.0, 0.0, 0.0]
}

#[derive(Debug, Deserialize, Serialize)]
pub struct State<E: Extensions, X: Extras> {
    /// WebGL states to enable.
    ///
    /// States not in the array are disabled.
    ///
    /// Valid values for each element correspond to WebGL enums:
    /// 3042 (BLEND)
    /// 2884 (CULL_FACE)
    /// 2929 (DEPTH_TEST)
    /// 32823 (POLYGON_OFFSET_FILL)
    /// 32926 (SAMPLE_ALPHA_TO_COVERAGE)
    /// 3089 (SCISSOR_TEST)
    #[serde(default)]
    enable: Vec<u32>,

    /// Arguments for fixed-function rendering state functions other than
    /// enable() / disable().
    functions: Option<Function<E, X>>,

    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: <E as Extensions>::TechniqueState,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <X as Extras>::TechniqueState,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Technique<E: Extensions, X: Extras> {
    /// A dictionary object of technique.parameters objects.
    ///
    /// Each parameter defines an attribute or uniform input, and an optional
    /// semantic and value.
    #[serde(default)]
    parameters: HashMap<String, Parameter<E, X>>,

    /// A dictionary object of strings that maps GLSL attribute names to
    /// technique parameter IDs.
    #[serde(default)]
    attributes: HashMap<String, String>,

    /// The ID of the program.
    program: String,

    /// A dictionary object of strings that maps GLSL uniform names to technique
    /// parameter IDs.
    #[serde(default)]
    uniforms: HashMap<String, String>,

    /// Fixed-function rendering states.
    #[serde(default)]
    states: Option<State<E, X>>,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., a technique and a buffer could
    /// have the same name, or two techniques could even have the same name.
    name: Option<String>,

    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: <E as Extensions>::Technique,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <X as Extras>::Technique,
}
