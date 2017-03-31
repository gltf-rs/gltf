
use v1::Extras;
 
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ImageExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image<E: Extras> {
    /// The uri of the image.
    ///
    /// Relative paths are relative to the .gltf file. Instead of referencing an
    /// external file, the uri can also be a data-uri. The image format must be
    /// jpg, png, bmp, or gif.
    pub uri: String,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., an image and a buffer could have
    /// the same name, or two images could even have the same name.
    pub name: Option<String>, 

    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: ImageExtensions,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Image,
}

