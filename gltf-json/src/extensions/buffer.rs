#[cfg(feature = "EXT_meshopt_compression")]
use crate::validation::USize64;
use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "extensions")]
use serde_json::{Map, Value};

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Buffer {
    #[cfg(feature = "EXT_meshopt_compression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<bool>,

    #[cfg(feature = "extensions")]
    #[serde(default, flatten)]
    pub others: Map<String, Value>,
}

#[cfg(feature = "EXT_meshopt_compression")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeshoptCompressionMode {
    Attributes,
    Triangles,
    Indices,
}

#[cfg(feature = "EXT_meshopt_compression")]
impl crate::validation::Validate for MeshoptCompressionMode {
    fn validate<P, R>(&self, _root: &crate::Root, _path: P, _report: &mut R)
    where
        P: Fn() -> crate::Path,
        R: FnMut(&dyn Fn() -> crate::Path, crate::validation::Error),
    {
        // No validation needed for enum variants
    }
}

#[cfg(feature = "EXT_meshopt_compression")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeshoptCompressionFilter {
    None,
    Octahedral,
    Quaternion,
    Exponential,
}

#[cfg(feature = "EXT_meshopt_compression")]
impl crate::validation::Validate for MeshoptCompressionFilter {
    fn validate<P, R>(&self, _root: &crate::Root, _path: P, _report: &mut R)
    where
        P: Fn() -> crate::Path,
        R: FnMut(&dyn Fn() -> crate::Path, crate::validation::Error),
    {
        // No validation needed for enum variants
    }
}

#[cfg(feature = "EXT_meshopt_compression")]
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeshoptCompression {
    pub buffer: crate::Index<crate::buffer::Buffer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<USize64>,
    pub byte_length: USize64,
    pub byte_stride: u32,
    pub count: u32,
    pub mode: MeshoptCompressionMode,
    #[serde(default)]
    pub filter: Option<MeshoptCompressionFilter>,
}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct View {
    #[cfg(feature = "EXT_meshopt_compression")]
    #[serde(
        rename = "EXT_meshopt_compression",
        skip_serializing_if = "Option::is_none"
    )]
    pub meshopt_compression: Option<MeshoptCompression>,

    #[cfg(feature = "extensions")]
    #[serde(default, flatten)]
    pub others: Map<String, Value>,
}
