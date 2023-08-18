use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};

/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Root {
    #[cfg(feature = "KHR_lights_punctual")]
    #[serde(
        default,
        rename = "KHR_lights_punctual",
        skip_serializing_if = "Option::is_none"
    )]
    pub khr_lights_punctual: Option<KhrLightsPunctual>,

    #[cfg(feature = "KHR_materials_variants")]
    #[serde(
        default,
        rename = "KHR_materials_variants",
        skip_serializing_if = "Option::is_none"
    )]
    pub khr_materials_variants: Option<KhrMaterialsVariants>,

    #[cfg(feature = "KITTYCAD_boundary_representation")]
    #[serde(
        default,
        rename = "KITTYCAD_boundary_representation",
        skip_serializing_if = "Option::is_none"
    )]
    pub kittycad_boundary_representation: Option<KittyCadBoundaryRepresentation>,
}

#[cfg(feature = "KHR_lights_punctual")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct KhrLightsPunctual {
    /// Lights at this node.
    pub lights: Vec<crate::extensions::scene::khr_lights_punctual::Light>,
}

#[cfg(feature = "KHR_lights_punctual")]
impl crate::root::Get<crate::extensions::scene::khr_lights_punctual::Light> for crate::Root {
    fn get(
        &self,
        id: crate::Index<crate::extensions::scene::khr_lights_punctual::Light>,
    ) -> Option<&crate::extensions::scene::khr_lights_punctual::Light> {
        if let Some(extensions) = self.extensions.as_ref() {
            if let Some(khr_lights_punctual) = extensions.khr_lights_punctual.as_ref() {
                khr_lights_punctual.lights.get(id.value())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(feature = "KHR_materials_variants")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct KhrMaterialsVariants {
    pub variants: Vec<crate::extensions::scene::khr_materials_variants::Variant>,
}

#[cfg(feature = "KHR_materials_variants")]
impl crate::root::Get<crate::extensions::scene::khr_materials_variants::Variant> for crate::Root {
    fn get(
        &self,
        id: crate::Index<crate::extensions::scene::khr_materials_variants::Variant>,
    ) -> Option<&crate::extensions::scene::khr_materials_variants::Variant> {
        self.extensions
            .as_ref()?
            .khr_materials_variants
            .as_ref()?
            .variants
            .get(id.value())
    }
}

#[cfg(feature = "KITTYCAD_boundary_representation")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct KittyCadBoundaryRepresentation {
    /// Solid boundary representation instances.
    pub breps: Vec<crate::extensions::kittycad_boundary_representation::BRep>,
    /// Abstract curve definitions.
    pub curves: Vec<crate::extensions::kittycad_boundary_representation::Curve>,
    /// Abstract surface definitions.
    pub surfaces: Vec<crate::extensions::kittycad_boundary_representation::Surface>,
    /// Boundary representation edges.
    pub edges: Vec<crate::extensions::kittycad_boundary_representation::brep::Edge>,
    /// Vertices in 3D space joining edges.
    pub edge_vertices: Vec<crate::extensions::kittycad_boundary_representation::brep::EdgeVertex>,
}

#[cfg(feature = "KITTYCAD_boundary_representation")]
impl crate::root::Get<crate::extensions::kittycad_boundary_representation::BRep> for crate::Root {
    fn get(
        &self,
        id: crate::Index<crate::extensions::kittycad_boundary_representation::BRep>,
    ) -> Option<&crate::extensions::kittycad_boundary_representation::BRep> {
        self.extensions
            .as_ref()?
            .kittycad_boundary_representation
            .as_ref()?
            .breps
            .get(id.value())
    }
}

#[cfg(feature = "KITTYCAD_boundary_representation")]
impl crate::root::Get<crate::extensions::kittycad_boundary_representation::Curve> for crate::Root {
    fn get(
        &self,
        id: crate::Index<crate::extensions::kittycad_boundary_representation::Curve>,
    ) -> Option<&crate::extensions::kittycad_boundary_representation::Curve> {
        self.extensions
            .as_ref()?
            .kittycad_boundary_representation
            .as_ref()?
            .curves
            .get(id.value())
    }
}

#[cfg(feature = "KITTYCAD_boundary_representation")]
impl crate::root::Get<crate::extensions::kittycad_boundary_representation::Surface>
    for crate::Root
{
    fn get(
        &self,
        id: crate::Index<crate::extensions::kittycad_boundary_representation::Surface>,
    ) -> Option<&crate::extensions::kittycad_boundary_representation::Surface> {
        self.extensions
            .as_ref()?
            .kittycad_boundary_representation
            .as_ref()?
            .surfaces
            .get(id.value())
    }
}

#[cfg(feature = "KITTYCAD_boundary_representation")]
impl crate::root::Get<crate::extensions::kittycad_boundary_representation::brep::Edge>
    for crate::Root
{
    fn get(
        &self,
        id: crate::Index<crate::extensions::kittycad_boundary_representation::brep::Edge>,
    ) -> Option<&crate::extensions::kittycad_boundary_representation::brep::Edge> {
        self.extensions
            .as_ref()?
            .kittycad_boundary_representation
            .as_ref()?
            .edges
            .get(id.value())
    }
}

#[cfg(feature = "KITTYCAD_boundary_representation")]
impl crate::root::Get<crate::extensions::kittycad_boundary_representation::brep::EdgeVertex>
    for crate::Root
{
    fn get(
        &self,
        id: crate::Index<crate::extensions::kittycad_boundary_representation::brep::EdgeVertex>,
    ) -> Option<&crate::extensions::kittycad_boundary_representation::brep::EdgeVertex> {
        self.extensions
            .as_ref()?
            .kittycad_boundary_representation
            .as_ref()?
            .edge_vertices
            .get(id.value())
    }
}
