/// 2D and 3D curve definitions.
pub mod curve {
    use crate::validation::{Checked, Error, Validate};
    use crate::{Path, Root};
    use gltf_derive::Validate;
    use serde::{de, ser};
    use serde_derive::{Deserialize, Serialize};
    use std::fmt;

    pub const VALID_CURVE_TYPES: &[&str] = &["linear", "nurbs"];

    /// Discriminant for `Curve` data.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
    pub enum Type {
        /// Linear curve.
        Linear = 1,
        /// NURBS curve.
        Nurbs = 2,
    }

    impl Type {
        pub fn as_str(self) -> &'static str {
            match self {
                Type::Linear => "linear",
                Type::Nurbs => "nurbs",
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
                        "linear" => Checked::Valid(Type::Linear),
                        "nurbs" => Checked::Valid(Type::Nurbs),
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

    /// Linear curve definition.
    ///
    /// Either end or direction must be set.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Linear {
        /// Origin position.
        pub start: [f32; 3],
        /// Unit vector pointing away from the origin position.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub direction: Option<[f32; 3]>,
        /// End position.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub end: Option<[f32; 3]>,
    }

    impl Validate for Linear {
        fn validate<P, R>(&self, _root: &Root, path: P, report: &mut R)
        where
            P: Fn() -> Path,
            R: FnMut(&dyn Fn() -> Path, Error),
        {
            if self.direction.is_none() && self.end.is_none() {
                report(&|| path().field("end"), Error::Missing);
            }
        }
    }

    /// NURBS curve definition.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Nurbs {
        /// Array of control vertices.
        pub control_points: Vec<[f32; 4]>,
        /// Knot vector.
        pub knot_vector: Vec<f32>,
        /// Order of basis splines.
        pub order: u32,
    }

    /// Curve parameter domain.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Domain {
        /// Minimum domain value.
        pub min: f32,

        /// Maximum domain value.
        pub max: f32,
    }

    impl Default for Domain {
        fn default() -> Self {
            Self { min: 0.0, max: 1.0 }
        }
    }

    /// Abstract curve data.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Curve {
        /// Discriminant.
        #[serde(rename = "type")]
        pub type_: Checked<Type>,

        /// Optional name for this surface.
        #[cfg(feature = "names")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// Additional parameters for a linear curve.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub linear: Option<Linear>,

        /// Additional parameters for a NURBS curve.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nurbs: Option<Nurbs>,

        /// Parameter domain.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub domain: Option<Domain>,
    }
}

/// 3D surface definitions.
pub mod surface {
    use crate::validation::{Checked, Error, Validate};
    use crate::{Path, Root};
    use serde::{de, ser};
    use serde_derive::{Deserialize, Serialize};
    use std::fmt;

    pub const VALID_SURFACE_TYPES: &[&str] = &["nurbs", "plane"];

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
                Type::Nurbs => "nurbs",
                Type::Plane => "plane",
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
                        "nurbs" => Checked::Valid(Type::Nurbs),
                        "plane" => Checked::Valid(Type::Plane),
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Nurbs {
        /// Matrix of control point vertices.
        pub control_points: Vec<[f32; 4]>,
        /// Dimensions of control point vertex matrix.
        pub num_control_points: [u32; 2],
        /// Number of knots in U and V.
        pub num_knots: [u32; 2],
        /// Knot vector.
        pub knot_vector: Vec<f32>,
        /// Order of basis splines.
        pub order: [u32; 2],
    }

    impl Validate for Nurbs {
        fn validate<P, R>(&self, _root: &Root, path: P, report: &mut R)
        where
            P: Fn() -> Path,
            R: FnMut(&dyn Fn() -> Path, Error),
        {
            let expected_control_points = self.num_control_points[0] * self.num_control_points[1];
            if expected_control_points as usize != self.control_points.len() {
                report(&|| path().field("num_control_points"), Error::Invalid);
            }

            let expected_knots = self.num_knots[0] + self.num_knots[1];
            if expected_knots as usize != self.knot_vector.len() {
                report(&|| path().field("num_knots"), Error::Invalid);
            }
        }
    }

    /// Simple planar surface definition.
    #[derive(Clone, Debug, Deserialize, Serialize, gltf_derive::Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Plane {
        /// Normal vector to the plane.
        pub normal: [f32; 3],
        /// The value of `d` in the plane equation `n.r + d = 0`.
        pub constant: Option<f32>,
        /// An arbitrary point that lies on the plane.
        pub point: Option<[f32; 3]>,
    }

    /// Abstract surface data.
    #[derive(Clone, Debug, Deserialize, Serialize, gltf_derive::Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Surface {
        /// Discriminant.
        #[serde(rename = "type")]
        pub type_: Checked<Type>,
        /// Optional name for this surface.
        #[cfg(feature = "names")]
        pub name: Option<String>,
        /// Arguments for a NURBS surface.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nurbs: Option<Nurbs>,
        /// Arguments for a planar surface.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub plane: Option<Plane>,
    }
}

/// Solid boundary representations.
pub mod brep {
    use crate::Index;
    use gltf_derive::Validate;
    use serde_derive::{Deserialize, Serialize};

    /// A trim curve.
    ///
    /// Trim curves define subsets of faces bound by edge.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Trim {
        /// The trim curve geometry.
        pub curve: Index<super::Curve>,

        /// Specifies whether the orientation of the curve should
        /// be reversed for this trim curve.
        pub reverse: bool,
    }

    /// Pair of vertices on a face plus an optional trim curve.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Edge {
        /// The edge curve geometry in 3D (or homogeneous 4D) space.
        pub curve: Index<super::Curve>,

        /// Specifies whether the orientation of the edge curve should
        /// be reversed.
        pub reverse: bool,

        /// Optional trimming curve in 2D (or homogeneous 3D) space.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub trim: Option<Trim>,
    }

    /// Set of edges on a face, each with an optional trim curve.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Loop {
        /// The edge curves forming the loop.
        pub edges: Vec<Edge>,

        /// Specifies whether the winding order of the loop should be
        /// interpreted in reverse order with respect to the face.
        pub reverse: bool,
    }

    /// Set of loops defined on an abstract surface.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Face {
        /// Surface the face edges and vertices reside on.
        pub surface: Index<super::Surface>,

        /// Face outer bound.
        pub outer_loop: Loop,

        /// Face inner bounds.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub inner_loops: Vec<Loop>,
    }

    /// Solid boundary representation structure.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct BRep {
        /// Array of faces forming a solid.
        pub faces: Vec<Face>,

        /// Optional name for this boundary representation.
        #[cfg(feature = "names")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// Optional mesh approximation of this solid.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mesh: Option<Index<crate::Mesh>>,
    }
}

pub use brep::BRep;
pub use curve::Curve;
pub use surface::Surface;
