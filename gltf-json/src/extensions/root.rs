use gltf_derive::Validate;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};

use crate::extensions::kittycad_boundary_representation as kcad;

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
#[derive(Clone, Debug, Default, Deserialize, JsonSchema, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct KittyCadBoundaryRepresentation {
    /// Solid boundary representation instances.
    pub solids: Vec<kcad::Solid>,

    /// Shell definitions.
    pub shells: Vec<kcad::Shell>,

    /// Face definitions.
    pub faces: Vec<kcad::Face>,

    /// Loop definitions.
    pub loops: Vec<kcad::Loop>,

    /// Edge definitions.
    pub edges: Vec<kcad::Edge>,

    /// Vertices in 3D space joining edges.
    pub vertices: Vec<kcad::Vertex>,

    /// Surface definitions.
    pub surfaces: Vec<kcad::Surface>,

    /// Curve definitions.
    pub curves: Vec<kcad::Curve>,
}

macro_rules! impl_get_for_kcad {
    ($ty:ty, $field:ident) => {
        #[cfg(feature = "KITTYCAD_boundary_representation")]
        impl crate::root::Get<$ty> for crate::Root {
            fn get(&self, index: crate::Index<$ty>) -> Option<&$ty> {
                self.extensions
                    .as_ref()?
                    .kittycad_boundary_representation
                    .as_ref()?
                    .$field
                    .get(index.value())
            }
        }

        #[cfg(feature = "KITTYCAD_boundary_representation")]
        impl crate::root::Get<$ty> for KittyCadBoundaryRepresentation {
            fn get(&self, index: crate::Index<$ty>) -> Option<&$ty> {
                self.$field.get(index.value())
            }
        }
    };
}

impl_get_for_kcad!(kcad::Solid, solids);
impl_get_for_kcad!(kcad::Shell, shells);
impl_get_for_kcad!(kcad::Face, faces);
impl_get_for_kcad!(kcad::Loop, loops);
impl_get_for_kcad!(kcad::Edge, edges);
impl_get_for_kcad!(kcad::Vertex, vertices);
impl_get_for_kcad!(kcad::Surface, surfaces);
impl_get_for_kcad!(kcad::Curve, curves);
