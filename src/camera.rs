use crate::Document;

#[cfg(feature = "extensions")]
use serde_json::{Map, Value};

/// A camera's projection.
#[derive(Clone, Debug)]
pub enum Projection<'a> {
    /// Describes an orthographic projection.
    Orthographic(Orthographic<'a>),

    /// Describes a perspective projection.
    Perspective(Perspective<'a>),
}

/// A camera's projection.  A node can reference a camera to apply a transform to
/// place the camera in the scene.
#[derive(Clone, Debug)]
pub struct Camera<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::camera::Camera,
}

///  Values for an orthographic camera projection.
#[derive(Clone, Debug)]
pub struct Orthographic<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a json::camera::Orthographic,
}

/// Values for a perspective camera projection.
#[derive(Clone, Debug)]
pub struct Perspective<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a json::camera::Perspective,
}

impl<'a> Camera<'a> {
    /// Constructs a `Camera`.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::camera::Camera,
    ) -> Self {
        Self {
            document,
            index,
            json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(docsrs, doc(cfg(feature = "names")))]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns the camera's projection.
    pub fn projection(&self) -> Projection {
        match self.json.type_.unwrap() {
            json::camera::Type::Orthographic => {
                let json = self.json.orthographic.as_ref().unwrap();
                Projection::Orthographic(Orthographic::new(self.document, json))
            }
            json::camera::Type::Perspective => {
                let json = self.json.perspective.as_ref().unwrap();
                Projection::Perspective(Perspective::new(self.document, json))
            }
        }
    }

    /// Returns extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        let ext = self.json.extensions.as_ref()?;
        Some(&ext.others)
    }

    /// Queries extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extension_value(&self, ext_name: &str) -> Option<&Value> {
        let ext = self.json.extensions.as_ref()?;
        ext.others.get(ext_name)
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

impl<'a> Orthographic<'a> {
    /// Constructs a `Orthographic` camera projection.
    pub(crate) fn new(document: &'a Document, json: &'a json::camera::Orthographic) -> Self {
        Self { document, json }
    }

    ///  The horizontal magnification of the view.
    pub fn xmag(&self) -> f32 {
        self.json.xmag
    }

    ///  The vertical magnification of the view.
    pub fn ymag(&self) -> f32 {
        self.json.ymag
    }

    ///  The distance to the far clipping plane.
    pub fn zfar(&self) -> f32 {
        self.json.zfar
    }

    ///  The distance to the near clipping plane.
    pub fn znear(&self) -> f32 {
        self.json.znear
    }

    /// Returns extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        let ext = self.json.extensions.as_ref()?;
        Some(&ext.others)
    }

    /// Queries extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extension_value(&self, ext_name: &str) -> Option<&Value> {
        let ext = self.json.extensions.as_ref()?;
        ext.others.get(ext_name)
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

impl<'a> Perspective<'a> {
    /// Constructs a `Perspective` camera projection.
    pub(crate) fn new(document: &'a Document, json: &'a json::camera::Perspective) -> Self {
        Self { document, json }
    }

    ///  Aspect ratio of the field of view.
    pub fn aspect_ratio(&self) -> Option<f32> {
        self.json.aspect_ratio
    }

    ///  The vertical field of view in radians.
    pub fn yfov(&self) -> f32 {
        self.json.yfov
    }

    ///  The distance to the far clipping plane.
    pub fn zfar(&self) -> Option<f32> {
        self.json.zfar
    }

    ///  The distance to the near clipping plane.
    pub fn znear(&self) -> f32 {
        self.json.znear
    }

    /// Returns extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        let ext = self.json.extensions.as_ref()?;
        Some(&ext.others)
    }

    /// Queries extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extension_value(&self, ext_name: &str) -> Option<&Value> {
        let ext = self.json.extensions.as_ref()?;
        ext.others.get(ext_name)
    }

    ///  Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}
