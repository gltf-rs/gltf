use gltf_derive::Validate;
use serde::{de, ser};
use serde_derive::{Serialize, Deserialize};
use std::fmt;
use crate::validation::{Checked, Error, Validate};
use crate::{extensions, Extras, Root, Path};

/// All valid camera types.
pub const VALID_CAMERA_TYPES: &'static [&'static str] = &[
    "perspective",
    "orthographic",
];

/// Specifies the camera type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    /// A perspective projection.
    Perspective = 1,

    /// An orthographic projection.
    Orthographic,
}

/// A camera's projection.
///
/// A node can reference a camera to apply a transform to place the camera in the
/// scene.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Camera {
    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(feature = "names", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    /// An orthographic camera containing properties to create an orthographic
    /// projection matrix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orthographic: Option<Orthographic>,

    /// A perspective camera containing properties to create a perspective
    /// projection matrix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perspective: Option<Perspective>,

    /// Specifies if the camera uses a perspective or orthographic projection.
    #[serde(rename = "type")]
    pub type_: Checked<Type>,

    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::camera::Camera>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

/// Values for an orthographic camera.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Orthographic {
    /// The horizontal magnification of the view.
    pub xmag: f32,

    /// The vertical magnification of the view.
    pub ymag: f32,

    /// The distance to the far clipping plane.
    pub zfar: f32,

    /// The distance to the near clipping plane.
    pub znear: f32,

    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::camera::Orthographic>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

/// Values for a perspective camera.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Perspective {
    /// Aspect ratio of the field of view.
    #[serde(rename = "aspectRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<f32>,

    /// The vertical field of view in radians.
    pub yfov: f32,

    /// The distance to the far clipping plane.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zfar: Option<f32>,

    /// The distance to the near clipping plane.
    pub znear: f32,

    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::camera::Perspective>,

    /// Optional application specific data.
    #[serde(default)]
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

impl Validate for Camera {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&dyn Fn() -> Path, Error)
    {
        if self.orthographic.is_none() && self.perspective.is_none() {
            report(&path, Error::Missing);
        }

        self.orthographic.validate(root, || path().field("orthographic"), report);
        self.perspective.validate(root, || path().field("perspective"), report);
        self.type_.validate(root, || path().field("type"), report);
        self.extensions.validate(root, || path().field("extensions"), report);
        self.extras.validate(root, || path().field("extras"), report);
    }
}

impl<'de> de::Deserialize<'de> for Checked<Type> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Type>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_CAMERA_TYPES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::Type::*;
                use crate::validation::Checked::*;
                Ok(match value {
                    "perspective" => Valid(Perspective),
                    "orthographic" => Valid(Orthographic),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl ser::Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        match *self {
            Type::Perspective => serializer.serialize_str("perspective"),
            Type::Orthographic => serializer.serialize_str("orthographic"),
        }
    }
}
