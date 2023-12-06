//! Boundary representations of solid objects

use crate::validation::{Error, Validate};
use crate::{Index, Path, Root};
use gltf_derive::Validate;
use schemars::gen::SchemaGenerator;
use schemars::schema::Schema;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 2D and 3D curve definitions.
pub mod curve {
    use crate::validation::{Checked, Error, Validate};
    use crate::{Path, Root};
    use gltf_derive::Validate;
    use schemars::JsonSchema;
    use serde::{de, ser};
    use serde_derive::{Deserialize, Serialize};
    use std::fmt;

    pub const VALID_CURVE_TYPES: &[&str] = &["circle", "linear", "nurbs"];

    /// Discriminant for `Curve` data.
    #[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq)]
    #[schemars(rename = "curve.type")]
    pub enum Type {
        /// Circular curve.
        Circle = 1,
        /// Line curve.
        Line,
        /// NURBS curve.
        Nurbs,
    }

    impl Type {
        pub fn as_str(self) -> &'static str {
            match self {
                Type::Circle => "circle",
                Type::Line => "line",
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
                        "circle" => Checked::Valid(Type::Circle),
                        "line" => Checked::Valid(Type::Line),
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

    /// Circular curve definition.
    ///
    /// λ(u) := O + R(cos(u)x + sin(u)y), where:
    /// * O = `self.origin`,
    /// * R = `self.radius`,
    /// * x = `self.xbasis`,
    /// * y = `self.normal` × `self.xbasis`,
    /// * u ∈ {0, 2π}.
    ///
    /// The `xbasis` and `normal` vectors form an orthonormal set.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    #[schemars(rename = "curve.circle")]
    pub struct Circle {
        /// Position at the center of the circle.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin: Option<[f64; 3]>,
        /// Distance from the center position to all points on the circle.
        pub radius: f64,
        /// Unit vector normal to the plane containing the circle.
        ///
        /// This serves as the Z basis in the parametric co-ordinate space.
        pub normal: [f64; 3],
        /// Unit vector in the direction from the origin to the point on
        /// the circle evaluated at λ(0).
        ///
        /// Due to floating point precision, this vector may not lie exactly
        /// in the plane. If this is the case then the X vector will be treated
        /// as the projection of this vector onto the plane.
        pub xbasis: [f64; 3],
    }

    /// Line curve definition.
    ///
    /// Either end or direction must be set.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
    #[serde(rename_all = "camelCase")]
    #[schemars(rename = "curve.line")]
    pub struct Line {
        /// Origin position.
        pub start: [f64; 3],
        /// Unit vector pointing away from the origin position.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub direction: Option<[f64; 3]>,
        /// End position.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub end: Option<[f64; 3]>,
    }

    impl Validate for Line {
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
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    #[schemars(rename = "curve.nurbs")]
    pub struct Nurbs {
        /// Array of control vertices.
        pub control_points: Vec<[f64; 4]>,
        /// Knot vector.
        pub knot_vector: Vec<f64>,
        /// Order of basis splines.
        pub order: u32,
    }

    /// Curve parameter domain.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    #[schemars(rename = "curve.domain")]
    pub struct Domain {
        /// Minimum domain value.
        pub min: f64,

        /// Maximum domain value.
        pub max: f64,
    }

    impl Default for Domain {
        fn default() -> Self {
            Self { min: 0.0, max: 1.0 }
        }
    }

    /// Abstract curve data.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    #[schemars(rename = "curve")]
    pub struct Curve {
        /// Discriminant.
        #[serde(rename = "type")]
        pub type_: Checked<Type>,

        /// Optional name for this surface.
        #[cfg(feature = "names")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// Additional parameters for a circular curve.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub circle: Option<Circle>,

        /// Additional parameters for a line curve.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub line: Option<Line>,

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
    use gltf_derive::Validate;
    use schemars::JsonSchema;
    use serde::{de, ser};
    use serde_derive::{Deserialize, Serialize};
    use std::fmt;

    pub const VALID_SURFACE_TYPES: &[&str] = &["cylinder", "nurbs", "plane", "sphere", "torus"];

    /// Domain of surface parameters.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    #[schemars(rename = "surface.domain")]
    pub struct Domain {
        /// Minimum domain values.
        pub min: [f64; 2],

        /// Maximum domain values.
        pub max: [f64; 2],
    }

    impl Default for Domain {
        fn default() -> Self {
            Self {
                min: [0.0, 0.0],
                max: [1.0, 1.0],
            }
        }
    }

    /// Discriminant for `Surface` data.
    #[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq)]
    #[schemars(rename = "surface.type")]
    pub enum Type {
        /// Cylindrical surface.
        Cylinder = 1,
        /// NURBS surface.
        Nurbs,
        /// Planar surface.
        Plane,
        /// Spherical surface.
        Sphere,
        /// Torus surface.
        Torus,
    }

    impl Type {
        pub fn as_str(self) -> &'static str {
            match self {
                Type::Cylinder => "cylinder",
                Type::Nurbs => "nurbs",
                Type::Plane => "plane",
                Type::Sphere => "sphere",
                Type::Torus => "torus",
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
                        "cylinder" => Checked::Valid(Type::Cylinder),
                        "nurbs" => Checked::Valid(Type::Nurbs),
                        "plane" => Checked::Valid(Type::Plane),
                        "sphere" => Checked::Valid(Type::Sphere),
                        "torus" => Checked::Valid(Type::Torus),
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

    /// Parametric cylindrical surface definition.
    ///
    /// σ(u, v) := O + R(cos(u)x + sin(u)y) + vz, where:
    /// * O = `self.circle.origin`,
    /// * R = `self.circle.radius`,
    /// * x = `self.circle.xbasis`,
    /// * y = `self.circle.normal` × `self.circle.xbasis`,
    /// * z = `self.circle.normal`,
    /// * h = `self.height`,
    /// * u ∈ {0, 2π},
    /// * v ∈ {0, h}.
    ///
    /// Cylinders are defined in reference to a circle that is extruded
    /// along the circle normal vector.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    #[schemars(rename = "surface.cylinder")]
    pub struct Cylinder {
        /// The extruded circle.
        pub circle: super::curve::Circle,
        /// Height of the extruded circle.
        pub height: f64,
    }

    /// NURBS surface definition.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
    #[serde(rename_all = "camelCase")]
    #[schemars(rename = "surface.nurbs")]
    pub struct Nurbs {
        /// Matrix of control point vertices.
        pub control_points: Vec<[f64; 4]>,
        /// Dimensions of control point vertex matrix.
        pub num_control_points: [u32; 2],
        /// Number of knots in U and V.
        pub num_knots: [u32; 2],
        /// Knot vector.
        pub knot_vector: Vec<f64>,
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

    /// Plane surface definition.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, gltf_derive::Validate)]
    #[serde(rename_all = "camelCase")]
    #[schemars(rename = "surface.plane")]
    pub struct Plane {
        /// Normal vector to the plane.
        pub normal: [f64; 3],
        /// The value of `d` in the plane equation `n.r = d`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub constant: Option<f64>,
        /// An arbitrary point that lies on the plane.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub point: Option<[f64; 3]>,
    }

    /// Parametric spherical surface definition.
    ///
    /// σ(u, v) := O + Rcos(v)(cos(u)x + sin(u)y) + Rsin(v)z, where:
    /// * O = `self.horizon.origin`,
    /// * R = `self.horizon.radius`,
    /// * x = `self.horizon.xbasis`,
    /// * y = `self.horizon.normal` × `self.horizon.xbasis`,
    /// * z = `self.horizon.normal`,
    /// * u ∈ {0, 2π},
    /// * v ∈ {0, 2π}.
    ///
    /// Spheres are defined in reference to a circle at zero inclination.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    #[schemars(rename = "surface.sphere")]
    pub struct Sphere {
        /// The circle at zero inclination.
        pub horizon: super::curve::Circle,
    }

    /// Toroidal surface definition.
    ///
    /// σ(u, v) := O + (R + rcos(v))(cos(u)x + sin(u)y) + rsin(v)z, where:
    /// * O = `self.origin`,
    /// * R = `self.radius`,
    /// * r = `self.circle.radius`,
    /// * x = `self.circle.xbasis`,
    /// * y = `self.circle.normal` × `self.circle.xbasis`,
    /// * z = `self.circle.normal`,
    /// * u, v ∈ {0, 2π}.
    ///
    /// Tori are defined in reference to a circle that is revolved about
    /// an origin at a specified distance. This distance is called the
    /// major radius. The radius of the circle of revolution is called the
    /// minor radius.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, gltf_derive::Validate)]
    #[serde(rename_all = "camelCase")]
    #[schemars(rename = "surface.torus")]
    pub struct Torus {
        /// The center of the torus.
        ///
        /// The axis of revolution passes through the origin of the torus.
        pub origin: [f64; 3],
        /// Circle of revolution.
        pub circle: super::curve::Circle,
        /// Distance from the torus origin to the origin of the revolved circle.
        pub radius: f64,
    }

    /// Abstract surface data.
    #[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, gltf_derive::Validate)]
    #[serde(rename_all = "camelCase")]
    #[schemars(rename = "surface")]
    pub struct Surface {
        /// Discriminant.
        #[serde(rename = "type")]
        pub type_: Checked<Type>,
        /// Optional name for this surface.
        #[cfg(feature = "names")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Arguments for a cylindrical surface.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cylinder: Option<Cylinder>,
        /// Arguments for a NURBS surface.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nurbs: Option<Nurbs>,
        /// Arguments for a planar surface.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub plane: Option<Plane>,
        /// Arguments for a spherical surface.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sphere: Option<Sphere>,
        /// Arguments for a toroidal surface.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub torus: Option<Torus>,
        /// Surface parameter domain.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub domain: Option<Domain>,
    }
}

/// Pair of vertices on a face plus an optional trim domain.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "edge")]
pub struct Edge {
    /// The edge curve geometry in 3D (or homogeneous 4D) space.
    pub curve: IndexWithOrientation<Curve>,

    /// Edge start vertex.
    pub start: Option<Index<Vertex>>,

    /// Edge end vertex.
    pub end: Option<Index<Vertex>>,

    /// Marker for a closed edge.
    pub closed: bool,

    /// Optional domain to select a subset of the edge curve geometry.
    ///
    /// When `None`, the domain is the same as the edge curve geometry
    /// domain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<curve::Domain>,
}

/// Junctions of edges in 3D space.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "vertex")]
pub struct Vertex(pub [f64; 3]);

impl Validate for Vertex {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
    }
}

/// Selected orientation of an orientable item.
#[derive(
    Clone, Copy, Debug, Default, Deserialize_repr, Eq, JsonSchema, PartialEq, Serialize_repr,
)]
#[repr(i8)]
#[schemars(rename = "orientation")]
pub enum Orientation {
    /// Same-sense orientation.
    #[default]
    Same = 1,

    /// Reverse-sense orientation.
    Reverse = -1,
}

impl Orientation {
    /// Query whether the orientation is in the same-sense state.
    pub fn is_same(self) -> bool {
        matches!(self, Orientation::Same)
    }

    /// Query whether the orientation is in the reverse-sense state.
    pub fn is_reverse(self) -> bool {
        matches!(self, Orientation::Reverse)
    }
}

/// Index for orientable items.
///
/// The JSON representation is an array of two numbers: the index followed by its orientation.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct IndexWithOrientation<T: Validate>(pub Index<T>, #[serde(default)] pub Orientation);

impl<T: Validate> IndexWithOrientation<T> {
    /// Explicit constructor.
    pub fn new(index: Index<T>, orientation: Orientation) -> Self {
        Self(index, orientation)
    }

    /// Create an index with same-sense orientation.
    pub fn same(index: Index<T>) -> Self {
        Self(index, Orientation::Same)
    }

    /// Create an index with reverse-sense orientation.
    pub fn reverse(index: Index<T>) -> Self {
        Self::new(index, Orientation::Reverse)
    }

    /// Returns the index.
    pub fn index(&self) -> Index<T> {
        self.0
    }

    /// Returns the orientation.
    pub fn orientation(&self) -> Orientation {
        self.1
    }

    /// Query whether the index has same-sense orientation.
    pub fn is_same(&self) -> bool {
        self.1.is_same()
    }

    /// Query whether the index has reverse-sense orientation.
    pub fn is_reverse(&self) -> bool {
        self.1.is_reverse()
    }
}

impl<T: Validate> From<(Index<T>, Orientation)> for IndexWithOrientation<T> {
    fn from((index, orientation): (Index<T>, Orientation)) -> Self {
        IndexWithOrientation(index, orientation)
    }
}

impl<T: Validate> From<IndexWithOrientation<T>> for (Index<T>, Orientation) {
    fn from(item: IndexWithOrientation<T>) -> Self {
        (item.index(), item.orientation())
    }
}

impl<T: Validate> JsonSchema for IndexWithOrientation<T> {
    fn schema_name() -> String {
        "IndexWithOrientation".to_owned()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        #[derive(Deserialize, JsonSchema, Serialize)]
        #[schemars(rename = "indexWithOrientation")]
        struct NonGenericIndexWithOrientation(pub u32, #[serde(default)] pub Orientation);
        NonGenericIndexWithOrientation::json_schema(generator)
    }
}

impl<T: Validate> Validate for IndexWithOrientation<T>
where
    crate::Root: crate::root::Get<T>,
{
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        self.0.validate(root, path, report);
    }
}

/// Edge loop.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "loop")]
pub struct Loop {
    /// Oriented edges forming the loop.
    pub edges: Vec<IndexWithOrientation<Edge>>,

    /// Optional 1:1 pairing of UV curves to edges.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uv_curves: Vec<IndexWithOrientation<Curve>>,
}

/// Set of loops defined on an abstract surface.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "face")]
pub struct Face {
    /// Surface the face edges and vertices reside on.
    pub surface: IndexWithOrientation<Surface>,

    /// Face bounds.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loops: Vec<IndexWithOrientation<Loop>>,
}

/// Boundary representation volume.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "shell")]
pub struct Shell {
    /// Set of connected faces forming a closed 'watertight' volume.
    pub faces: Vec<IndexWithOrientation<Face>>,

    /// Optional name for this shell.
    #[cfg(feature = "names")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Solid boundary representation structure.
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "solid")]
pub struct Solid {
    /// The boundaries of the solid volume.
    pub shells: Vec<IndexWithOrientation<Shell>>,

    /// Optional name for this solid.
    #[cfg(feature = "names")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional mesh approximation of this solid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Index<crate::Mesh>>,
}

pub use curve::Curve;
pub use surface::Surface;
