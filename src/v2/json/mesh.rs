
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use v2::json::{accessor, material, Extras, Index, Root};
use v2::validation::{Error, JsonPath, Validate};

/// Corresponds to `GL_POINTS`.
pub const POINTS: u32 = 0;

/// Corresponds to `GL_LINES`.
pub const LINES: u32 = 1;

/// Corresponds to `GL_LINE_LOOP`.
pub const LINE_LOOP: u32 = 2;

/// Corresponds to `GL_LINE_STRIP`.
pub const LINE_STRIP: u32 = 3;

/// Corresponds to `GL_TRIANGLES`.
pub const TRIANGLES: u32 = 4;

/// Corresponds to `GL_TRIANGLE_STRIP`.
pub const TRIANGLE_STRIP: u32 = 5;

/// Corresponds to `GL_TRIANGLE_FAN`.
pub const TRIANGLE_FAN: u32 = 6;

/// All valid primitive rendering modes.
pub const VALID_MODES: &'static [u32] = &[
    POINTS,
    LINES,
    LINE_LOOP,
    LINE_STRIP,
    TRIANGLES,
    TRIANGLE_STRIP,
    TRIANGLE_FAN,
];

/// All valid semantic names for Morph targets.
pub const VALID_MORPH_TARGETS: &'static [&'static str] = &[
    "POSITION",
    "NORMAL",
    "TANGENT",
];

/// Extension specific data for `Mesh`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct MeshExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Mesh {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: MeshExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// Optional user-defined name for this object.
    pub name: Option<String>,
    
    /// Defines the geometry to be renderered with a material.
    pub primitives: Vec<Primitive>,

    /// Defines the weights to be applied to the morph targets.
    pub weights: Option<Vec<f32>>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Primitive {
    /// Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    pub attributes: Attributes,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: PrimitiveExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// The index of the accessor that contains the indices.
    pub indices: Option<Index<accessor::Accessor>>,
    
    /// The index of the material to apply to this primitive when rendering
    pub material: Option<Index<material::Material>>,
    
    /// The type of primitives to render.
    #[serde(default)]
    pub mode: Mode,
    
    /// An array of Morph Targets, each  Morph Target is a dictionary mapping
    /// attributes (only `POSITION`, `NORMAL`, and `TANGENT` supported) to their
    /// deviations in the Morph Target.
    pub targets: Option<Vec<MorphTargets>>,
}

/// Extension specific data for `Primitive`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct PrimitiveExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// Map of attribute semantic names to the `Accessor`s containing the
/// corresponding attribute data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Attributes(pub HashMap<Semantic, Index<accessor::Accessor>>);

/// Vertex attribute semantic name.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Semantic(pub String);

/// The type of primitives to render.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Mode(pub u32);

/// A dictionary mapping attributes to their deviations in the Morph Target.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MorphTargets(pub HashMap<Semantic, Index<accessor::Accessor>>);

impl Validate for Attributes {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        for (semantic, index) in self.0.iter() {
            index.validate(root, || path().key(semantic.as_str()), report);
            semantic.validate(root, || path(), report);
        }
    }
}

impl Default for Mode {
    fn default() -> Mode {
        Mode(TRIANGLES)
    }
}

impl Validate for Mode {
    fn validate<P, R>(&self, _: &Root, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if !VALID_MODES.contains(&self.0) {
            report(Error::invalid_value(path(), self.0));
        }
    }
}

impl Validate for MorphTargets {
    fn validate<P, R>(&self, _: &Root, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        for attr in self.0.keys() {
            let name = attr.0.as_str();
            if !VALID_MORPH_TARGETS.contains(&name) {
                report(Error::invalid_value(path().key(name), name.to_string()));
            }
        }
    }
}

impl Semantic {
    fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Validate for Semantic {
    fn validate<P, R>(&self, _: &Root, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        let name = self.0.as_str();
        let set = |name: &str, prefix: &str| name[prefix.len()..].parse::<u32>();
        for prefix in &["COLOR_", "TEXCOORD_", "JOINTS_", "WEIGHTS_"] {
            if name.starts_with(prefix) {
                if set(name, prefix).is_err() {
                    // Set index is not a number
                    report(Error::invalid_semantic_name(path(), self.0.clone()));
                }
                return;
            }
        }
        match name {
            "NORMAL" | "POSITION" | "TANGENT" => {},
            _ if name.starts_with("_") => {},
            _ => report(Error::invalid_semantic_name(path(), self.0.clone())),
        }
    }
}
