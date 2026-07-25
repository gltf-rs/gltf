use crate::validation::{Checked, Error};
use crate::{accessor, extensions, material, Extras, Index};
use gltf_derive::Validate;
use serde::{de, ser, Serialize};
use serde_derive::{Deserialize, Serialize};
use serde_json::from_value;
use std::collections::BTreeMap;
use std::fmt;

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
pub const VALID_MODES: &[u32] = &[
    POINTS,
    LINES,
    LINE_LOOP,
    LINE_STRIP,
    TRIANGLES,
    TRIANGLE_STRIP,
    TRIANGLE_FAN,
];

/// All valid base semantic names for Morph targets.
///
/// Per the glTF 2.0 specification, a morph target dictionary may contain
/// `POSITION`, `NORMAL`, `TANGENT`, plus numbered `TEXCOORD_<n>` and
/// `COLOR_<n>` attribute deviations.
pub const VALID_MORPH_TARGETS: &[&str] = &["POSITION", "NORMAL", "TANGENT", "TEXCOORD", "COLOR"];

/// The type of primitives to render.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum Mode {
    /// Corresponds to `GL_POINTS`.
    Points = 1,

    /// Corresponds to `GL_LINES`.
    Lines,

    /// Corresponds to `GL_LINE_LOOP`.
    LineLoop,

    /// Corresponds to `GL_LINE_STRIP`.
    LineStrip,

    /// Corresponds to `GL_TRIANGLES`.
    Triangles,

    /// Corresponds to `GL_TRIANGLE_STRIP`.
    TriangleStrip,

    /// Corresponds to `GL_TRIANGLE_FAN`.
    TriangleFan,
}

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Mesh {
    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::mesh::Mesh>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// Defines the geometry to be renderered with a material.
    pub primitives: Vec<Primitive>,

    /// Defines the weights to be applied to the morph targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f32>>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[gltf(validate_hook = "primitive_validate_hook")]
pub struct Primitive {
    /// Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    pub attributes: BTreeMap<Checked<Semantic>, Index<accessor::Accessor>>,

    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::mesh::Primitive>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
    pub extras: Extras,

    /// The index of the accessor that contains the indices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<Index<accessor::Accessor>>,

    /// The index of the material to apply to this primitive when rendering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<Index<material::Material>>,

    /// The type of primitives to render.
    #[serde(default, skip_serializing_if = "is_primitive_mode_default")]
    pub mode: Checked<Mode>,

    /// An array of Morph Targets, each Morph Target is a dictionary mapping
    /// attributes (`POSITION`, `NORMAL`, `TANGENT`, and numbered `TEXCOORD_<n>`
    /// / `COLOR_<n>` attributes) to their deviations in the Morph Target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<MorphTarget>>,
}

fn is_primitive_mode_default(mode: &Checked<Mode>) -> bool {
    *mode == Checked::Valid(Mode::Triangles)
}

fn primitive_validate_hook<P, R>(primitive: &Primitive, root: &crate::Root, path: P, report: &mut R)
where
    P: Fn() -> crate::Path,
    R: FnMut(&dyn Fn() -> crate::Path, crate::validation::Error),
{
    let position_path = &|| path().field("attributes").key("POSITION");
    if let Some(pos_accessor_index) = primitive
        .attributes
        .get(&Checked::Valid(Semantic::Positions))
    {
        // The accessor index is validated by `primitive.attributes.validate(..)`
        // above, but that reports `Error::IndexOutOfRange` without
        // short-circuiting. Use `.get(..)` here so we don't panic before
        // the caller can return the accumulated errors.
        let Some(pos_accessor) = root.accessors.get(pos_accessor_index.value()) else {
            return;
        };

        // spec: POSITION accessor **must** have `min` and `max` properties defined.
        let min_path = &|| position_path().field("min");
        if let Some(ref min) = pos_accessor.min {
            if from_value::<[f32; 3]>(min.clone()).is_err() {
                report(min_path, Error::Invalid);
            }
        } else {
            report(min_path, Error::Missing);
        }

        let max_path = &|| position_path().field("max");
        if let Some(ref max) = pos_accessor.max {
            if from_value::<[f32; 3]>(max.clone()).is_err() {
                report(max_path, Error::Invalid);
            }
        } else {
            report(max_path, Error::Missing);
        }
    } else {
        report(position_path, Error::Missing);
    }
}

/// A dictionary mapping attributes to their deviations in the Morph Target.
///
/// Per the glTF 2.0 specification, a morph target dictionary may contain the
/// fixed attributes `POSITION`, `NORMAL` and `TANGENT`, plus numbered
/// `TEXCOORD_<n>` and `COLOR_<n>` attribute deviations. The numbered
/// attributes are stored in [`MorphTarget::tex_coords`] and
/// [`MorphTarget::colors`] as maps keyed by the set index `n`.
#[derive(Clone, Debug, Validate)]
pub struct MorphTarget {
    /// XYZ vertex position displacements of type `[f32; 3]`.
    pub positions: Option<Index<accessor::Accessor>>,

    /// XYZ vertex normal displacements of type `[f32; 3]`.
    pub normals: Option<Index<accessor::Accessor>>,

    /// XYZ vertex tangent displacements of type `[f32; 3]`.
    pub tangents: Option<Index<accessor::Accessor>>,

    /// UV texture co-ordinate displacements of type `[f32; 2]`, keyed by
    /// `TEXCOORD_<n>` set index.
    pub tex_coords: BTreeMap<u32, Index<accessor::Accessor>>,

    /// Vertex color displacements of type `[f32; 3]` or `[f32; 4]` (RGB or
    /// RGBA), keyed by `COLOR_<n>` set index.
    pub colors: BTreeMap<u32, Index<accessor::Accessor>>,
}

impl Default for MorphTarget {
    fn default() -> Self {
        MorphTarget {
            positions: None,
            normals: None,
            tangents: None,
            tex_coords: BTreeMap::new(),
            colors: BTreeMap::new(),
        }
    }
}

impl Serialize for MorphTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        use ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(ref p) = self.positions {
            map.serialize_entry("POSITION", p)?;
        }
        if let Some(ref n) = self.normals {
            map.serialize_entry("NORMAL", n)?;
        }
        if let Some(ref t) = self.tangents {
            map.serialize_entry("TANGENT", t)?;
        }
        for (set, index) in &self.tex_coords {
            map.serialize_entry(&format!("TEXCOORD_{}", set), index)?;
        }
        for (set, index) in &self.colors {
            map.serialize_entry(&format!("COLOR_{}", set), index)?;
        }
        map.end()
    }
}

impl<'de> de::Deserialize<'de> for MorphTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct MorphTargetVisitor;

        impl<'de> de::Visitor<'de> for MorphTargetVisitor {
            type Value = MorphTarget;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a morph target attribute dictionary")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut positions = None;
                let mut normals = None;
                let mut tangents = None;
                let mut tex_coords: BTreeMap<u32, Index<accessor::Accessor>> = BTreeMap::new();
                let mut colors: BTreeMap<u32, Index<accessor::Accessor>> = BTreeMap::new();
                while let Some(key) = access.next_key::<String>()? {
                    match key.as_str() {
                        "POSITION" => {
                            positions = Some(access.next_value()?);
                        }
                        "NORMAL" => {
                            normals = Some(access.next_value()?);
                        }
                        "TANGENT" => {
                            tangents = Some(access.next_value()?);
                        }
                        other => {
                            if let Some(rest) = other.strip_prefix("TEXCOORD_") {
                                match rest.parse::<u32>() {
                                    Ok(set) => {
                                        let index = access.next_value()?;
                                        tex_coords.insert(set, index);
                                    }
                                    Err(_) => {
                                        return Err(de::Error::custom(format!(
                                            "invalid TEXCOORD set index: {}",
                                            other
                                        )))
                                    }
                                }
                            } else if let Some(rest) = other.strip_prefix("COLOR_") {
                                match rest.parse::<u32>() {
                                    Ok(set) => {
                                        let index = access.next_value()?;
                                        colors.insert(set, index);
                                    }
                                    Err(_) => {
                                        return Err(de::Error::custom(format!(
                                            "invalid COLOR set index: {}",
                                            other
                                        )))
                                    }
                                }
                            } else {
                                // Unknown key: skip its value so the
                                // deserializer stays forward-compatible with
                                // new fixed attributes added by future spec
                                // revisions.
                                let _: de::IgnoredAny = access.next_value()?;
                            }
                        }
                    }
                }
                Ok(MorphTarget {
                    positions,
                    normals,
                    tangents,
                    tex_coords,
                    colors,
                })
            }
        }

        deserializer.deserialize_map(MorphTargetVisitor)
    }
}

/// Vertex attribute semantic name.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Semantic {
    /// Extra attribute name.
    #[cfg(feature = "extras")]
    Extras(String),

    /// XYZ vertex positions.
    Positions,

    /// XYZ vertex normals.
    Normals,

    /// XYZW vertex tangents where the `w` component is a sign value indicating the
    /// handedness of the tangent basis.
    Tangents,

    /// RGB or RGBA vertex color.
    Colors(u32),

    /// UV texture co-ordinates.
    TexCoords(u32),

    /// Joint indices.
    Joints(u32),

    /// Joint weights.
    Weights(u32),
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Triangles
    }
}

impl Mode {
    /// Returns the equivalent `GLenum`.
    pub fn as_gl_enum(self) -> u32 {
        match self {
            Mode::Points => POINTS,
            Mode::Lines => LINES,
            Mode::LineLoop => LINE_LOOP,
            Mode::LineStrip => LINE_STRIP,
            Mode::Triangles => TRIANGLES,
            Mode::TriangleStrip => TRIANGLE_STRIP,
            Mode::TriangleFan => TRIANGLE_FAN,
        }
    }
}

impl<'de> de::Deserialize<'de> for Checked<Mode> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl de::Visitor<'_> for Visitor {
            type Value = Checked<Mode>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_MODES)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::Mode::*;
                use crate::validation::Checked::*;
                Ok(match value as u32 {
                    POINTS => Valid(Points),
                    LINES => Valid(Lines),
                    LINE_LOOP => Valid(LineLoop),
                    LINE_STRIP => Valid(LineStrip),
                    TRIANGLES => Valid(Triangles),
                    TRIANGLE_STRIP => Valid(TriangleStrip),
                    TRIANGLE_FAN => Valid(TriangleFan),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_u64(Visitor)
    }
}

impl ser::Serialize for Mode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_u32(self.as_gl_enum())
    }
}

impl Semantic {
    fn checked(s: &str) -> Checked<Self> {
        use self::Semantic::*;
        use crate::validation::Checked::*;
        match s {
            "NORMAL" => Valid(Normals),
            "POSITION" => Valid(Positions),
            "TANGENT" => Valid(Tangents),
            #[cfg(feature = "extras")]
            _ if s.starts_with('_') => Valid(Extras(s[1..].to_string())),
            _ if s.starts_with("COLOR_") => match s["COLOR_".len()..].parse() {
                Ok(set) => Valid(Colors(set)),
                Err(_) => Invalid,
            },
            _ if s.starts_with("TEXCOORD_") => match s["TEXCOORD_".len()..].parse() {
                Ok(set) => Valid(TexCoords(set)),
                Err(_) => Invalid,
            },
            _ if s.starts_with("JOINTS_") => match s["JOINTS_".len()..].parse() {
                Ok(set) => Valid(Joints(set)),
                Err(_) => Invalid,
            },
            _ if s.starts_with("WEIGHTS_") => match s["WEIGHTS_".len()..].parse() {
                Ok(set) => Valid(Weights(set)),
                Err(_) => Invalid,
            },
            _ => Invalid,
        }
    }
}

impl ser::Serialize for Semantic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl ToString for Semantic {
    fn to_string(&self) -> String {
        use self::Semantic::*;
        match *self {
            Positions => "POSITION".into(),
            Normals => "NORMAL".into(),
            Tangents => "TANGENT".into(),
            Colors(set) => format!("COLOR_{}", set),
            TexCoords(set) => format!("TEXCOORD_{}", set),
            Joints(set) => format!("JOINTS_{}", set),
            Weights(set) => format!("WEIGHTS_{}", set),
            #[cfg(feature = "extras")]
            Extras(ref name) => format!("_{}", name),
        }
    }
}

impl ToString for Checked<Semantic> {
    fn to_string(&self) -> String {
        match *self {
            Checked::Valid(ref semantic) => semantic.to_string(),
            Checked::Invalid => "<invalid semantic name>".into(),
        }
    }
}

impl<'de> de::Deserialize<'de> for Checked<Semantic> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl de::Visitor<'_> for Visitor {
            type Value = Checked<Semantic>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "semantic name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Semantic::checked(value))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::validation::{Error, Validate};
    use crate::{Path, Root};

    #[test]
    fn primitive_with_oob_position_accessor_is_rejected() {
        // The mesh primitive's `POSITION` references accessor index 1, but
        // no `accessors` array is present. `Primitive::validate` previously
        // panicked on direct slice indexing here; it must now report errors.
        let json = r#"{
            "asset": {"version": "2.0"},
            "meshes": [{
                "primitives": [{
                    "attributes": { "POSITION": 1 }
                }]
            }]
        }"#;
        let root: Root = serde_json::from_str(json).expect("parse root");
        let mut errors = Vec::new();
        root.validate(&root, Path::new, &mut |path, error| {
            errors.push((path(), error));
        });
        // The accessor index validation reports `IndexOutOfBounds`; what
        // matters is that `validate` returned without panicking.
        assert!(
            errors.iter().any(|(_, e)| *e == Error::IndexOutOfBounds),
            "expected IndexOutOfBounds, got {:?}",
            errors
        );
    }
}
