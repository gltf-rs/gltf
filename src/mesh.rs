use crate::validation::{Error, Validate};
use crate::{accessor, material, Extras, Index, Path, Root, UnrecognizedExtensions};
use serde::ser;
use serde_json::from_value;
use std::collections::BTreeMap;

/// Support for the `KHR_materials_variants` extension.
pub mod khr_materials_variants {
    use crate::{Extras, Index, Material, UnrecognizedExtensions};

    /// Identifies all material variants applicable to a particular primitive.
    #[derive(
        Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate,
    )]
    pub struct Variants {
        /// Applicable material variant mappings.
        pub mappings: Vec<Mapping>,

        /// Unrecognized extension data.
        pub unrecognized_extensions: UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<Extras>,
    }

    /// Identifies a single material variant applicable to a particular primitive.
    #[derive(
        Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate,
    )]
    pub struct Mapping {
        /// Base material index.
        pub material: Index<Material>,

        /// Applicable material variants.
        pub variants: Vec<Index<crate::root::khr_materials_variants::Variant>>,

        /// Unrecognized extension data.
        pub unrecognized_extensions: UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<Extras>,
    }
}

/// The type of primitives to render.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    serde_repr::Deserialize_repr,
    Eq,
    PartialEq,
    serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum Mode {
    /// Corresponds to `GL_POINTS`.
    Points = 0,

    /// Corresponds to `GL_LINES`.
    Lines = 1,

    /// Corresponds to `GL_LINE_LOOP`.
    LineLoop = 2,

    /// Corresponds to `GL_LINE_STRIP`.
    LineStrip = 3,

    /// Corresponds to `GL_TRIANGLES`.
    #[default]
    Triangles = 4,

    /// Corresponds to `GL_TRIANGLE_STRIP`.
    TriangleStrip = 5,

    /// Corresponds to `GL_TRIANGLE_FAN`.
    TriangleFan = 6,
}

impl Validate for Mode {}

impl Mode {
    /// Returns the equivalent `GLenum`.
    pub fn as_gl_enum(self) -> u32 {
        self as u32
    }
}

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate)]
pub struct Mesh {
    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// Defines the geometry to be renderered with a material.
    pub primitives: Vec<Primitive>,

    /// Defines the weights to be applied to the morph targets.
    pub weights: Vec<f32>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize)]
pub struct Primitive {
    /// Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    pub attributes: BTreeMap<Semantic, Index<accessor::Accessor>>,

    /// The index of the accessor that contains the indices.
    pub indices: Option<Index<accessor::Accessor>>,

    /// The index of the material to apply to this primitive when rendering
    pub material: Option<Index<material::Material>>,

    /// The type of primitives to render.
    #[gltf(default)]
    pub mode: Mode,

    /// An array of Morph Targets, each  Morph Target is a dictionary mapping
    /// attributes (only `POSITION`, `NORMAL`, and `TANGENT` supported) to their
    /// deviations in the Morph Target.
    pub targets: Vec<MorphTarget>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,

    /// Support for the `KHR_materials_variants` extension.
    #[gltf(extension = "KHR_materials_variants")]
    pub variants: Option<khr_materials_variants::Variants>,
}

impl Validate for Primitive {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        // Generated part
        self.attributes
            .validate(root, || path().field("attributes"), report);
        self.extras
            .validate(root, || path().field("extras"), report);
        self.indices
            .validate(root, || path().field("indices"), report);
        self.material
            .validate(root, || path().field("material"), report);
        self.mode.validate(root, || path().field("mode"), report);
        self.targets
            .validate(root, || path().field("targets"), report);

        // Custom part
        let position_path = &|| path().field("attributes").key("POSITION");
        if let Some(pos_accessor_index) = self.attributes.get(&Semantic::Positions) {
            // spec: POSITION accessor **must** have `min` and `max` properties defined.
            let pos_accessor = &root.accessors[pos_accessor_index.value()];

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
}

/// A dictionary mapping attributes to their deviations in the Morph Target.
#[derive(Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate)]
pub struct MorphTarget {
    /// XYZ vertex position displacements of type `[f32; 3]`.
    #[serde(rename = "POSITION")]
    pub positions: Option<Index<accessor::Accessor>>,

    /// XYZ vertex normal displacements of type `[f32; 3]`.
    #[serde(rename = "NORMAL")]
    pub normals: Option<Index<accessor::Accessor>>,

    /// XYZ vertex tangent displacements of type `[f32; 3]`.
    #[serde(rename = "TANGENT")]
    pub tangents: Option<Index<accessor::Accessor>>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

/// Vertex attribute semantic name.
#[derive(Clone, Debug, serde_with::DeserializeFromStr, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Semantic {
    /// Extra attribute name.
    Extras(String),

    /// Extension attribute name.
    Extensions(String),

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

impl Validate for Semantic {}

impl std::str::FromStr for Semantic {
    type Err = <u32 as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NORMAL" => Ok(Self::Normals),
            "POSITION" => Ok(Self::Positions),
            "TANGENT" => Ok(Self::Tangents),
            _ if s.starts_with("COLOR_") => s["COLOR_".len()..].parse().map(Self::Colors),
            _ if s.starts_with("TEXCOORD_") => s["TEXCOORD_".len()..].parse().map(Self::TexCoords),
            _ if s.starts_with("JOINTS_") => s["JOINTS_".len()..].parse().map(Self::Joints),
            _ if s.starts_with("WEIGHTS_") => s["WEIGHTS_".len()..].parse().map(Self::Weights),
            _ if s.starts_with('_') => Ok(Self::Extras(s[1..].to_owned())),
            _ => Ok(Self::Extensions(s.to_owned())),
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
        match *self {
            Self::Positions => "POSITION".into(),
            Self::Normals => "NORMAL".into(),
            Self::Tangents => "TANGENT".into(),
            Self::Colors(set) => format!("COLOR_{}", set),
            Self::TexCoords(set) => format!("TEXCOORD_{}", set),
            Self::Joints(set) => format!("JOINTS_{}", set),
            Self::Weights(set) => format!("WEIGHTS_{}", set),
            Self::Extras(ref name) => format!("_{name}"),
            Self::Extensions(ref name) => name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Semantic;

    #[test]
    fn semantic() {
        let test_cases = [
            ("POSITION", Semantic::Positions),
            ("NORMAL", Semantic::Normals),
            ("TANGENT", Semantic::Tangents),
            ("COLOR_0", Semantic::Colors(0)),
            ("TEXCOORD_1", Semantic::TexCoords(1)),
            ("JOINTS_2", Semantic::Joints(2)),
            ("WEIGHTS_3", Semantic::Weights(3)),
            ("_EXTRA", Semantic::Extras("EXTRA".to_string())),
            ("EXTENSION", Semantic::Extensions("EXTENSION".to_string())),
        ];

        for (name, semantic) in test_cases {
            assert_eq!(Ok(semantic.clone()), name.parse());
            assert_eq!(name, &semantic.to_string());
        }
    }
}
