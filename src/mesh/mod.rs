/// Utility functions.
#[cfg(feature = "utils")]
#[cfg_attr(docsrs, doc(cfg(feature = "utils")))]
pub mod util;

use crate::validation::{Error, Validate};
use crate::{accessor, material, Extras, Index, Root, UnrecognizedExtensions};
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

/// Vertex position bounding box.
pub type BoundingBox = Bounds<[f32; 3]>;

/// The minimum and maximum values for a generic accessor.
#[derive(Clone, Debug, PartialEq)]
pub struct Bounds<T> {
    /// Minimum value.
    pub min: T,

    /// Maximum value.
    pub max: T,
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
#[derive(Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate)]
#[gltf(validate_hook = "primitive_validate_hook")]
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

fn primitive_validate_hook<P, R>(primitive: &Primitive, root: &crate::Root, path: P, report: &mut R)
where
    P: Fn() -> crate::Path,
    R: FnMut(&dyn Fn() -> crate::Path, crate::validation::Error),
{
    let position_path = &|| path().field("attributes").key("POSITION");
    if let Some(pos_accessor_index) = primitive.attributes.get(&Semantic::Positions) {
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

#[cfg(feature = "utils")]
impl Primitive {
    /// Returns the bounds of the `POSITION` vertex attribute.
    #[cfg(feature = "utils")]
    #[cfg_attr(docsrs, doc(cfg(feature = "utils")))]
    pub fn bounding_box(&self, root: &Root) -> BoundingBox {
        let index = self
            .attributes
            .get(&Semantic::Positions)
            .expect("primitive has no POSITION attribute");
        let accessor = root.get(*index).expect("index out of range");
        let min: [f32; 3] =
            serde_json::from_value(accessor.min.clone().expect("accessor.min missing"))
                .expect("failed to parse accessor.min");
        let max: [f32; 3] =
            serde_json::from_value(accessor.max.clone().expect("accessor.max missing"))
                .expect("failed to parse accessor.max");
        Bounds { min, max }
    }

    /// Constructs the primitive reader.
    #[cfg(feature = "utils")]
    #[cfg_attr(docsrs, doc(cfg(feature = "utils")))]
    pub fn reader<'a, 's, F>(&'a self, root: &'a Root, get_buffer_data: F) -> Reader<'a, 's, F>
    where
        F: Clone + Fn(Index<crate::buffer::Buffer>) -> Option<&'s [u8]>,
    {
        Reader {
            root,
            primitive: self,
            get_buffer_data,
        }
    }
}

/// Mesh primitive reader.
#[cfg(feature = "utils")]
#[derive(Clone, Debug)]
pub struct Reader<'a, 's, F>
where
    F: Clone + Fn(Index<crate::buffer::Buffer>) -> Option<&'s [u8]>,
{
    pub(crate) root: &'a Root,
    pub(crate) primitive: &'a Primitive,
    pub(crate) get_buffer_data: F,
}

#[cfg(feature = "utils")]
impl<'a, 's, F> Reader<'a, 's, F>
where
    F: Clone + Fn(Index<crate::buffer::Buffer>) -> Option<&'s [u8]>,
{
    /// Visits the vertex positions of a primitive.
    pub fn read_positions(&self) -> Option<util::ReadPositions<'s>> {
        self.primitive
            .attributes
            .get(&Semantic::Positions)
            .and_then(|accessor| {
                accessor::Iter::new(self.root, *accessor, self.get_buffer_data.clone())
            })
    }

    /// Visits the vertex normals of a primitive.
    pub fn read_normals(&self) -> Option<util::ReadNormals<'s>> {
        self.primitive
            .attributes
            .get(&Semantic::Normals)
            .and_then(|accessor| {
                accessor::Iter::new(self.root, *accessor, self.get_buffer_data.clone())
            })
    }

    /// Visits the vertex tangents of a primitive.
    pub fn read_tangents(&self) -> Option<util::ReadTangents<'s>> {
        self.primitive
            .attributes
            .get(&Semantic::Tangents)
            .and_then(|accessor| {
                accessor::Iter::new(self.root, *accessor, self.get_buffer_data.clone())
            })
    }

    /// Visits the vertex colors of a primitive.
    pub fn read_colors(&self, set: u32) -> Option<util::ReadColors<'s>> {
        use self::util::ReadColors;
        use accessor::AttributeType::{Vec3, Vec4};
        use accessor::ComponentType::{F32, U16, U8};
        let index = self.primitive.attributes.get(&Semantic::Colors(set))?;
        let accessor = self.root.get(*index)?;
        match (accessor.component_type, accessor.attribute_type) {
            (U8, Vec3) => accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                .map(ReadColors::RgbU8),
            (U16, Vec3) => accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                .map(ReadColors::RgbU16),
            (F32, Vec3) => accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                .map(ReadColors::RgbF32),
            (U8, Vec4) => accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                .map(ReadColors::RgbaU8),
            (U16, Vec4) => accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                .map(ReadColors::RgbaU16),
            (F32, Vec4) => accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                .map(ReadColors::RgbaF32),
            _ => unreachable!(),
        }
    }

    /// Visits the vertex draw sequence of a primitive.
    pub fn read_indices(&self) -> Option<util::ReadIndices<'s>> {
        use self::util::ReadIndices;
        use accessor::ComponentType;
        let index = self.primitive.indices?;
        let accessor = self.root.get(index)?;
        match accessor.component_type {
            ComponentType::U8 => {
                accessor::Iter::new(self.root, index, self.get_buffer_data.clone())
                    .map(ReadIndices::U8)
            }
            ComponentType::U16 => {
                accessor::Iter::new(self.root, index, self.get_buffer_data.clone())
                    .map(ReadIndices::U16)
            }
            ComponentType::U32 => {
                accessor::Iter::new(self.root, index, self.get_buffer_data.clone())
                    .map(ReadIndices::U32)
            }
            _ => unreachable!(),
        }
    }

    /// Visits the joint indices of the primitive.
    pub fn read_joints(&self, set: u32) -> Option<util::ReadJoints<'s>> {
        use self::util::ReadJoints;
        use accessor::ComponentType;
        let index = self.primitive.attributes.get(&Semantic::Joints(set))?;
        let accessor = self.root.get(*index)?;
        match accessor.component_type {
            ComponentType::U8 => {
                accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                    .map(ReadJoints::U8)
            }
            ComponentType::U16 => {
                accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                    .map(ReadJoints::U16)
            }
            _ => unreachable!(),
        }
    }

    /// Visits the vertex texture co-ordinates of a primitive.
    pub fn read_tex_coords(&self, set: u32) -> Option<util::ReadTexCoords<'s>> {
        use self::util::ReadTexCoords;
        use accessor::ComponentType;
        let index = self.primitive.attributes.get(&Semantic::TexCoords(set))?;
        let accessor = self.root.get(*index)?;
        match accessor.component_type {
            ComponentType::U8 => {
                accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                    .map(ReadTexCoords::U8)
            }
            ComponentType::U16 => {
                accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                    .map(ReadTexCoords::U16)
            }
            ComponentType::F32 => {
                accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                    .map(ReadTexCoords::F32)
            }
            _ => unreachable!(),
        }
    }

    /// Visits the joint weights of the primitive.
    pub fn read_weights(&self, set: u32) -> Option<util::ReadWeights<'s>> {
        use self::accessor::ComponentType;
        use self::util::ReadWeights;
        let index = self.primitive.attributes.get(&Semantic::Weights(set))?;
        let accessor = self.root.get(*index)?;
        match accessor.component_type {
            ComponentType::U8 => {
                accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                    .map(ReadWeights::U8)
            }
            ComponentType::U16 => {
                accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                    .map(ReadWeights::U16)
            }
            ComponentType::F32 => {
                accessor::Iter::new(self.root, *index, self.get_buffer_data.clone())
                    .map(ReadWeights::F32)
            }
            _ => unreachable!(),
        }
    }

    /// Visits the morph targets of the primitive.
    pub fn read_morph_targets(&self) -> util::ReadMorphTargets<'a, 's, F> {
        util::ReadMorphTargets {
            index: 0,
            reader: self.clone(),
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
