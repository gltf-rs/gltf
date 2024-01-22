use crate::validation::{Error, USize64, Validate};
use crate::{Extras, Index, Path, Root, UnrecognizedExtensions};

/// The minimum byte stride.
pub const MIN_BYTE_STRIDE: usize = 4;

/// The maximum byte stride.
pub const MAX_BYTE_STRIDE: usize = 252;

/// Specifies the target a GPU buffer should be bound to.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde_repr::Deserialize_repr, serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum Target {
    /// Corresponds to `GL_ARRAY_BUFFER`.
    ArrayBuffer = 34_962,

    /// Corresponds to `GL_ELEMENT_ARRAY_BUFFER`.
    ElementArrayBuffer = 34_963,
}
impl Validate for Target {}

/// Distance between individual items in a buffer view, measured in bytes.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
    serde_derive::Deserialize,
    serde_derive::Serialize,
)]
pub struct Stride(pub usize);

impl Validate for Stride {
    fn validate<P, R>(&self, _root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        if self.0 < MIN_BYTE_STRIDE || self.0 > MAX_BYTE_STRIDE {
            report(&path, Error::Invalid);
        }
    }
}

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate)]
pub struct Buffer {
    /// The length of the buffer in bytes.
    #[serde(rename = "byteLength")]
    pub length: USize64,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The uri of the buffer.  Relative paths are relative to the .gltf file.
    /// Instead of referencing an external file, the uri can also be a data-uri.
    pub uri: Option<String>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

/// A view into a buffer generally representing a subset of the buffer.
///
/// <https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#reference-bufferview>
///
#[derive(Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate)]
pub struct View {
    /// The parent `Buffer`.
    pub buffer: Index<Buffer>,

    /// The length of the `BufferView` in bytes.
    #[serde(rename = "byteLength")]
    pub length: USize64,

    /// Offset into the parent buffer in bytes.
    #[serde(default, rename = "byteOffset")]
    pub offset: USize64,

    /// The stride in bytes between vertex attributes or other interleavable data.
    ///
    /// When zero, data is assumed to be tightly packed.
    #[serde(rename = "byteStride")]
    pub stride: Option<Stride>,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// Optional target the buffer should be bound to.
    pub target: Option<Target>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

mod tests {
    #[test]
    fn serialize_target() {
        assert_eq!(
            "34962",
            serde_json::to_string(&super::Target::ArrayBuffer).unwrap(),
        );
    }

    #[test]
    fn deserialize_target() {
        assert_eq!(
            super::Target::ElementArrayBuffer,
            serde_json::from_str("34963").unwrap(),
        );

        assert!(serde_json::from_str::<super::Target>("123").is_err());
    }

    #[test]
    fn serialize_buffer() {
        let user_data = serde_json::json!({ "bar": 42 });
        let example = super::Buffer {
            length: 12usize.into(),
            name: Some("foo".to_owned()),
            uri: None,
            extras: Some(serde_json::value::to_raw_value(&user_data).unwrap()),
            unrecognized_extensions: Default::default(),
        };
        assert_eq!(
            r#"{"byteLength":12,"name":"foo","extras":{"bar":42}}"#,
            serde_json::to_string(&example).unwrap(),
        );
    }

    #[test]
    fn deserialize_buffer() {
        let json = r#"{"byteLength":12,"name":"foo","extras":{"bar":42}}"#;
        let buffer = serde_json::from_str::<super::Buffer>(json).unwrap();
        assert_eq!(buffer.length, super::USize64(12));
        assert_eq!(buffer.name.as_deref(), Some("foo"));
        assert_eq!(buffer.uri, None);
        assert_eq!(
            buffer
                .extras
                .as_deref()
                .map(serde_json::value::RawValue::get),
            Some(r#"{"bar":42}"#)
        );
    }
}
