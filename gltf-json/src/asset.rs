use {extensions, Extras};

/// Metadata about the glTF asset.
#[derive(Clone, Debug, Deserialize, Validate)]
pub struct Asset {
    /// A copyright message suitable for display to credit the content creator.
    pub copyright: Option<String>,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::asset::Asset,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// Tool that generated this glTF model.
    pub generator: Option<String>,

    /// The minimum glTF version that this asset targets.
    #[serde(rename = "minVersion")]
    pub min_version: Option<String>,
    
    /// The glTF version of this asset.
    pub version: String,
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            copyright: None,
            extensions: Default::default(),
            extras: Default::default(),
            generator: None,
            min_version: None,
            version: "2.0".to_string(),
        }
    }
}

