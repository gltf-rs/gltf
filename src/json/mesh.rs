
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::collections::HashMap;
use std::marker::PhantomData;

use json::{self, accessor, material, Extras, Index, Root};
use validation::{Error, JsonPath, Validate};

use self::accessor::{FLOAT, UNSIGNED_BYTE, UNSIGNED_SHORT};

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
pub struct MeshExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Mesh<'a> {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: MeshExtensions<'a>,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
    
    /// Optional user-defined name for this object.
    pub name: Option<Cow<'a, str>>,
    
    /// Defines the geometry to be renderered with a material.
    pub primitives: Vec<Primitive<'a>>,

    /// Defines the weights to be applied to the morph targets.
    pub weights: Option<Vec<f32>>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Primitive<'a> {
    /// Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    pub attributes: Attributes<'a>,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: PrimitiveExtensions<'a>,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras<'a>,
    
    /// The index of the accessor that contains the indices.
    pub indices: Option<Index<accessor::Accessor<'a>>>,
    
    /// The index of the material to apply to this primitive when rendering
    pub material: Option<Index<material::Material<'a>>>,
    
    /// The type of primitives to render.
    #[serde(default)]
    pub mode: Mode,
    
    /// An array of Morph Targets, each  Morph Target is a dictionary mapping
    /// attributes (only `POSITION`, `NORMAL`, and `TANGENT` supported) to their
    /// deviations in the Morph Target.
    pub targets: Option<Vec<MorphTargets<'a>>>,
}

/// Extension specific data for `Primitive`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct PrimitiveExtensions<'a> {
    #[serde(default)]
    _allow_unknown_fields: PhantomData<&'a ()>,
}

/// Map of attribute semantic names to the `Accessor`s containing the
/// corresponding attribute data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Attributes<'a>(pub HashMap<Semantic<'a>, Index<accessor::Accessor<'a>>>);

/// Vertex attribute semantic name.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Semantic<'a>(pub Cow<'a, str>);

/// The type of primitives to render.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Mode(pub u32);

/// A dictionary mapping attributes to their deviations in the Morph Target.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MorphTargets<'a>(pub HashMap<Semantic<'a>, Index<accessor::Accessor<'a>>>);

impl<'a> Validate<'a> for Attributes<'a> {
    fn validate<P, R>(&self, root: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        for (semantic, index) in self.0.iter() {
            let path = || path().key(semantic.as_str());
            let attribute = if let Ok(accessor) = root.try_get(index) {
                accessor
            } else {
                report(Error::index_out_of_bounds(path()));
                return;
            };
            semantic.validate_against_attribute(attribute, path, report);
        }
    }
}

impl Default for Mode {
    fn default() -> Mode {
        Mode(TRIANGLES)
    }
}

impl<'a> Validate<'a> for Mode {
    fn validate<P, R>(&self, _: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if !VALID_MODES.contains(&self.0) {
            report(Error::invalid_enum(path(), self.0));
        }
    }
}

impl<'a> Validate<'a> for MorphTargets<'a> {
    fn validate<P, R>(&self, root: &Root<'a>, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        for (semantic, index) in &self.0 {
            let path = || path().key(semantic.as_str());
            let target = if let Ok(accessor) = root.try_get(index) {
                accessor
            } else {
                report(Error::index_out_of_bounds(path()));
                return;
            };
            semantic.validate_against_morph_target(target, path, report);
        }
    }
}

impl<'a> Semantic<'a> {
    /// Validates a semantic name against an accessor describing a vertex attribute.
    pub fn validate_against_attribute<P, R>(
        &self,
        accessor: &json::accessor::Accessor<'a>,
        path: P,
        report: &mut R,
    ) where
        P: Fn() -> JsonPath,
        R: FnMut(Error)
    {
        // Validate the correctness of the semantic name
        let set = |name: &str, prefix: &str| name[prefix.len()..].parse::<u8>();
        let name = self.as_str();
        match name {
            "NORMAL" | "POSITION" | "TANGENT" => {},
            _ if name.starts_with("_") => {
                // There are no requirements on user accessors
                return;
            },
            _ => {
                let mut valid = false;
                for prefix in &["COLOR_", "TEXCOORD_", "JOINTS_", "WEIGHTS_"] {
                    if name.starts_with(prefix) {
                        if set(name, prefix).is_ok() {
                            valid = true;
                            break;
                        }
                    }
                }
                
                if !valid {
                    report(Error::invalid_semantic_name(path(), name.to_string()));
                    // Don't validate an accessor against an invalid semantic
                    return;
                }
            },
        }
        
        // Validate the correctness of the accessor type against the semantic name
        let semantic_discriminant = &name[..2];
        let accessor_kind = (
            accessor.type_.0.as_ref(),
            accessor.component_type.0,
            accessor.normalized,
        );
        let analysis = match semantic_discriminant {
            // Position
            "PO" => match accessor_kind {
                ("VEC3", FLOAT, false) => Ok(()),
                _ => Err("must be `VEC3` of `FLOAT`"),
            },
            // Normal
            "NO" =>  match accessor_kind {
                ("VEC3", FLOAT, false) => Ok(()),
                _ => Err("must be `VEC3` of `FLOAT`"),
            },
            // Tangent
            "TA" => match accessor_kind {
                ("VEC4", FLOAT, false) => Ok(()),
                _ => Err("must be `VEC4` of `FLOAT`"),
            },
            // Color
            "CO" => match accessor_kind {
                ("VEC3", FLOAT, false) => Ok(()),
                ("VEC4", FLOAT, false) => Ok(()),
                ("VEC3", UNSIGNED_BYTE, true) => Ok(()),
                ("VEC4", UNSIGNED_BYTE, true) => Ok(()),
                ("VEC3", UNSIGNED_SHORT, true) => Ok(()),
                ("VEC4", UNSIGNED_SHORT, true) => Ok(()),
                _ => Err("must be `VEC3` or `VEC4` of `FLOAT`, normalized `UNSIGNED_BYTE`, or normalized `UNSIGNED_SHORT`"),
            },
            // Texture co-ordinate
            "TE" => match accessor_kind {
                ("VEC2", FLOAT, false) => Ok(()),
                ("VEC2", UNSIGNED_BYTE, true) => Ok(()),
                ("VEC2", UNSIGNED_SHORT, true) => Ok(()),
                _ => Err("must be `VEC2` of `FLOAT`, normalized `UNSIGNED_BYTE`, or normalized `UNSIGNED_SHORT`"),
            },
            // Joint indices
            "JO" => match accessor_kind {
                ("VEC4", UNSIGNED_BYTE, false) => Ok(()),
                ("VEC4", UNSIGNED_SHORT, false) => Ok(()),
                _ => Err("must be `VEC4` of `UNSIGNED_BYTE` or `UNSIGNED_SHORT`"),
            },
            // Joint weights
            "WE" => match accessor_kind {
                ("VEC4", FLOAT, false) => Ok(()),
                ("VEC4", UNSIGNED_BYTE, true) => Ok(()),
                ("VEC4", UNSIGNED_SHORT, true) => Ok(()),
                _ => Err("must be `VEC4` of `FLOAT`, normalized `UNSIGNED_BYTE`, or normalized `UNSIGNED_SHORT`"),
            },
            _ => unreachable!(),
        };
        match analysis {
            Ok(()) => {},
            Err(reason) => report(
                Error::invalid_value(
                    path(),
                    name.to_string(),
                    reason.to_string(),
                )
            ),
        }
    }
    
    /// Validates a semantic name against an accessor describing a Morph target.
    pub fn validate_against_morph_target<P, R>(
        &self,
        accessor: &json::accessor::Accessor<'a>,
        path: P,
        report: &mut R,
    ) where
        P: Fn() -> JsonPath,
        R: FnMut(Error)
    {
        let name = self.as_str();
        let valid = match (
            name,
            accessor.type_.0.as_ref(),
            accessor.component_type.0,
            accessor.normalized,
        ) {
            ("POSITION", "VEC3", FLOAT, false) => true,
            ("NORMAL", "VEC3", FLOAT, false) => true,
            ("TANGENT", "VEC3", FLOAT, false) => true,
            _ => false,
        };

        if !valid {
            report(
                Error::invalid_value(
                    path(),
                    name.to_string(),
                    format!("must be `VEC3` of `FLOAT` and only `POSITION`, `NORMAL`, and `TANGENT` are supported"),
                )
            )
        }
    }

    /// Returns the internal representation.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}
