/// 2D free-form curves.
pub mod curve {
    use crate::validation::Checked;
    use crate::{Accessor, Index};
    use gltf_derive::Validate;
    use serde::{de, ser};
    use serde_derive::{Deserialize, Serialize};
    use std::fmt;

    pub const VALID_CURVE_TYPES: &[&str] = &["NURBS"];

    /// Discriminant for `Curve` data.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
    pub enum Type {
        /// NURBS curve.
        Nurbs = 1,
    }

    impl Type {
        pub fn as_str(self) -> &'static str {
            match self {
                Type::Nurbs => "NURBS",
            }
        }
    }

    impl<'de> de::Deserialize<'de> for Checked<Type> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct Visitor;
            impl<'de> de::Visitor<'de> for Visitor {
                type Value = Checked<Type>;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "any of: {:?}", VALID_CURVE_TYPES)
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(match value {
                        "NURBS" => Checked::Valid(Type::Nurbs),
                        _ => Checked::Invalid,
                    })
                }
            }
            deserializer.deserialize_str(Visitor)
        }
    }

    impl ser::Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }

    /// NURBS curve definition.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Nurbs {
        /// Must be `VEC4` of floating point type.
        pub control_points: Index<Accessor>,
        /// Must be index type.
        pub knot_vector: Index<Accessor>,
        /// Order of basis splines.
        pub order: u32,
    }

    /// Abstract curve data.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Curve {
        /// Discriminant.
        #[serde(rename = "type")]
        pub type_: Checked<Type>,
        /// Arguments for a NURBS curve.
        pub nurbs: Option<Nurbs>,
    }
}

/// 3D free-form surfaces.
pub mod surface {
    use crate::validation::Checked;
    use crate::{Accessor, Index};
    use gltf_derive::Validate;
    use serde::{de, ser};
    use serde_derive::{Deserialize, Serialize};
    use std::fmt;

    pub const VALID_SURFACE_TYPES: &[&str] = &["PLANE", "NURBS"];

    /// Discriminant for `Surface` data.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
    pub enum Type {
        /// NURBS surface.
        Nurbs = 1,
        /// Planar surface.
        Plane,
    }

    impl Type {
        pub fn as_str(self) -> &'static str {
            match self {
                Type::Nurbs => "NURBS",
                Type::Plane => "PLANE",
            }
        }
    }

    impl<'de> de::Deserialize<'de> for Checked<Type> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct Visitor;
            impl<'de> de::Visitor<'de> for Visitor {
                type Value = Checked<Type>;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "any of: {:?}", VALID_SURFACE_TYPES)
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(match value {
                        "NURBS" => Checked::Valid(Type::Nurbs),
                        "PLANE" => Checked::Valid(Type::Plane),
                        _ => Checked::Invalid,
                    })
                }
            }
            deserializer.deserialize_str(Visitor)
        }
    }

    impl ser::Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }

    /// NURBS surface definition.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Nurbs {
        /// Must be "VEC4" of floating point type.
        pub control_points: Index<Accessor>,
        /// Must be index type.
        pub knot_vector: Index<Accessor>,
        /// Order of basis splines.
        pub order: u32,
    }

    /// Simple planar surface definition.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Plane {
        /// Normal vector to the plane.
        pub normal: [f32; 3],
        /// The value of `d` in the plane equation `n.r + d = 0`.
        pub constant: f32,
    }

    /// Abstract surface data.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Surface {
        /// Discriminant.
        #[serde(rename = "type")]
        pub type_: Checked<Type>,
        /// Arguments for a NURBS surface.
        pub nurbs: Option<Nurbs>,
        /// Arguments for a planar surface.
        pub plane: Option<Plane>,
    }
}

/// Solid boundary representations.
pub mod brep {
    use crate::{Accessor, Index};
    use gltf_derive::Validate;
    use serde_derive::{Deserialize, Serialize};

    /// Set of vertices on a face plus trim curves.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Loop {
        /// Required: loop vertices.
        pub vertices: Index<Accessor>,
        /// Optional: set of trim curves to refine the loop.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub trim_curves: Vec<Index<super::Curve>>,
    }

    /// Set of loops defined on an abstract surface.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Face {
        /// Surface the face edges and vertices reside on.
        pub surface: Index<super::Surface>,
        /// Edge loops defining the face area.
        pub loops: Vec<Loop>,
    }

    /// Solid boundary representation structure.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct BRep {
        /// Array of faces forming a solid.
        pub faces: Vec<Face>,
        /// Optional name for this boundary representation.
        pub name: Option<String>,
        /// Optional mesh approximation of this solid.
        pub mesh: Option<Index<crate::Mesh>>,
    }
}

pub use brep::BRep;
pub use curve::Curve;
pub use surface::Surface;
