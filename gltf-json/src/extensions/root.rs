use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};

/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Root {
    #[cfg(feature = "KHR_lights_punctual")]
    #[serde(default, rename = "KHR_lights_punctual", skip_serializing_if = "Option::is_none")]
    pub khr_lights_punctual: Option<KhrLightsPunctual>,

    #[cfg(feature = "KHR_materials_variants")]
    #[serde(default, rename = "KHR_materials_variants", skip_serializing_if = "Option::is_none")]
    pub khr_materials_variants: Option<KhrMaterialsVariants>,
}

#[cfg(feature = "KHR_lights_punctual")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct KhrLightsPunctual {
    /// Lights at this node.
    pub lights: Vec<crate::extensions::scene::khr_lights_punctual::Light>,
}

#[cfg(feature = "KHR_lights_punctual")]
impl crate::root::Get<crate::extensions::scene::khr_lights_punctual::Light> for crate::Root {
    fn get(&self, id: crate::Index<crate::extensions::scene::khr_lights_punctual::Light>)
        -> Option<&crate::extensions::scene::khr_lights_punctual::Light>
    {
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
    fn get(&self, id: crate::Index<crate::extensions::scene::khr_materials_variants::Variant>)
        -> Option<&crate::extensions::scene::khr_materials_variants::Variant>
    {
        if let Some(extensions) = self.extensions.as_ref() {
            if let Some(khr_materials_variants) = extensions.khr_materials_variants.as_ref() {
                khr_materials_variants.variants.get(id.value())
            } else {
                None
            }
        } else {
            None
        }
    }
}
