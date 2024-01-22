use crate::validation::{Error, Validate};
use crate::{Extras, Path, Root, UnrecognizedExtensions};

/// Projection matrix parameters.
#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, gltf_derive::Wrap)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Projection {
    /// Perspective projection.
    Perspective {
        /// Perspective projection parameters.
        perspective: Perspective,
    },
    /// Orthographic projection.
    Orthographic {
        /// Orthographic projection parameters.
        orthographic: Orthographic,
    },
}

/// A viewpoint in the scene.
///
/// A node can reference a camera to apply a transform to place the camera in the
/// scene.
#[derive(
    Clone,
    Debug,
    gltf_derive::Deserialize,
    gltf_derive::Serialize,
    gltf_derive::Validate,
    gltf_derive::Wrap,
)]
#[gltf(indexed)]
pub struct Camera {
    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// Projection matrix parameters.
    #[serde(flatten)]
    pub projection: Projection,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

/// Values for an orthographic camera.
#[derive(
    Clone,
    Debug,
    gltf_derive::Deserialize,
    gltf_derive::Serialize,
    gltf_derive::Validate,
    gltf_derive::Wrap,
)]
pub struct Orthographic {
    /// The horizontal magnification of the view.
    pub xmag: f32,

    /// The vertical magnification of the view.
    pub ymag: f32,

    /// The distance to the far clipping plane.
    pub zfar: f32,

    /// The distance to the near clipping plane.
    pub znear: f32,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

/// Values for a perspective camera.
#[derive(
    Clone,
    Debug,
    gltf_derive::Deserialize,
    gltf_derive::Serialize,
    gltf_derive::Validate,
    gltf_derive::Wrap,
)]
pub struct Perspective {
    /// Aspect ratio of the field of view.
    pub aspect_ratio: Option<f32>,

    /// The vertical field of view in radians.
    pub yfov: f32,

    /// The distance to the far clipping plane.
    pub zfar: Option<f32>,

    /// The distance to the near clipping plane.
    pub znear: f32,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

impl Validate for Projection {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        match self {
            Self::Perspective { perspective } => {
                perspective.validate(root, || path().field("perspective"), report);
            }
            Self::Orthographic { orthographic } => {
                orthographic.validate(root, || path().field("orthographic"), report);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn serialize() {
        let camera = super::Camera {
            name: None,
            extras: None,
            projection: super::Projection::Perspective {
                perspective: super::Perspective {
                    aspect_ratio: None,
                    yfov: 0.785,
                    zfar: Some(10.0),
                    znear: 0.01,
                    extras: None,
                    unrecognized_extensions: Default::default(),
                },
            },
            unrecognized_extensions: Default::default(),
        };
        let json = serde_json::to_string(&camera).unwrap();
        assert_eq!(
            r#"{"type":"perspective","perspective":{"yfov":0.785,"zfar":10.0,"znear":0.01}}"#,
            json
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"orthographic","orthographic":{"xmag":1.0,"ymag":1.0,"zfar":10.0,"znear":0.01}}"#;
        let camera = serde_json::from_str::<super::Camera>(json).unwrap();
        match camera.projection {
            super::Projection::Perspective { perspective: _ } => {
                panic!("expected orthographic projection")
            }
            super::Projection::Orthographic { orthographic } => {
                assert_eq!(orthographic.xmag, 1.0);
                assert_eq!(orthographic.ymag, 1.0);
                assert_eq!(orthographic.zfar, 10.0);
                assert_eq!(orthographic.znear, 0.01);
            }
        }
    }
}
