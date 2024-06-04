use crate::{Extras, UnrecognizedExtensions};
use gltf_derive::{Deserialize, Serialize, Validate};

/// Metadata about the glTF asset.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Asset {
    /// A copyright message suitable for display to credit the content creator.
    pub copyright: Option<String>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,

    /// Tool that generated this glTF model.
    pub generator: Option<String>,

    /// The minimum glTF version that this asset targets.
    pub min_version: Option<String>,

    /// The glTF version of this asset.
    pub version: String,
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            copyright: None,
            unrecognized_extensions: Default::default(),
            extras: None,
            generator: None,
            min_version: None,
            version: "2.0".to_string(),
        }
    }
}

mod tests {
    #[test]
    fn serialize() {
        let asset = super::Asset {
            copyright: Some("X".to_owned()),
            min_version: Some("2.0".to_owned()),
            ..Default::default()
        };
        let json = serde_json::to_string(&asset).unwrap();
        assert_eq!(
            json,
            r#"{"copyright":"X","minVersion":"2.0","version":"2.0"}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"copyright":"X","minVersion":"2.0","version":"2.0"}"#;
        let asset = serde_json::from_str::<super::Asset>(json).unwrap();
        assert_eq!(asset.copyright.as_deref(), Some("X"));
        assert!(asset.extras.is_none());
        assert!(asset.generator.is_none());
        assert_eq!(asset.min_version.as_deref(), Some("2.0"));
        assert_eq!(asset.version.as_str(), "2.0");
    }
}
