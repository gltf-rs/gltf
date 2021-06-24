use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};

/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Root {
    #[cfg(feature = "KHR_lights_punctual")]
    #[serde(default, rename = "KHR_lights_punctual", skip_serializing_if = "Option::is_none")]
    pub khr_lights_punctual: Option<KhrLightsPunctual>,

    #[cfg(feature = "CESIUM_RTC")]
    #[serde(default, rename = "CESIUM_RTC", skip_serializing_if = "Option::is_none")]
    pub cesium_rtc: Option<CesiumRtc>
}

#[cfg(feature = "KHR_lights_punctual")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct KhrLightsPunctual {
    /// Lights at this node.
    pub lights: Vec<crate::extensions::scene::khr_lights_punctual::Light>,
}

#[cfg(feature = "CESIUM_RTC")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct CesiumRtc {
    pub center: [f64; 3],
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
