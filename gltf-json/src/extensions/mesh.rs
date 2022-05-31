use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Mesh {}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Primitive {
    #[cfg(feature = "KHR_materials_variants")]
    #[serde(
        default,
        rename = "KHR_materials_variants",
        skip_serializing_if = "Option::is_none"
    )]
    pub khr_materials_variants: Option<KhrMaterialsVariants>,
    #[cfg(feature = "KHR_draco_mesh_compression")]
    #[serde(
        default,
        rename = "KHR_draco_mesh_compression",
        skip_serializing_if = "Option::is_none"
    )]
    pub khr_draco_mesh_compression: Option<KhrDracoMeshCompression>,
}

#[cfg(feature = "KHR_materials_variants")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct KhrMaterialsVariants {
    pub mappings: Vec<Mapping>,
}

#[cfg(feature = "KHR_materials_variants")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Mapping {
    pub material: u32,
    pub variants: Vec<u32>,
}

#[cfg(feature = "KHR_draco_mesh_compression")]
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct KhrDracoMeshCompression {
    #[serde(rename = "bufferView")]
    pub buffer_view: crate::Index<crate::buffer::View>,
    pub attributes:
        std::collections::HashMap<crate::validation::Checked<crate::mesh::Semantic>, u32>,
}
