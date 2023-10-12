//! Boundary representations of solid objects

use crate::validation::{Error, Validate};
use crate::{Index, Path, Root};
use gltf_derive::Validate;
use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};

/// 2D and 3D curve definitions.
pub mod curve {
    use crate::validation::{Checked, Error, Validate};
    use crate::{Path, Root};
    use gltf_derive::Validate;
    use serde::{de, ser};
    use serde_derive::{Deserialize, Serialize};
    use std::fmt;

    pub const VALID_CURVE_TYPES: &[&str] = &["circle", "linear", "nurbs"];

    /// Discriminant for `Curve` data.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
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
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
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
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Nurbs {
        /// Array of control vertices.
        pub control_points: Vec<[f64; 4]>,
        /// Knot vector.
        pub knot_vector: Vec<f64>,
        /// Order of basis splines.
        pub order: u32,
    }

    /// Curve parameter domain.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
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
    use serde::{de, ser};
    use serde_derive::{Deserialize, Serialize};
    use std::fmt;

    pub const VALID_SURFACE_TYPES: &[&str] = &["cylinder", "nurbs", "plane", "sphere", "torus"];

    /// Domain of surface parameters.
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
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
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Cylinder {
        /// The extruded circle.
        pub circle: super::curve::Circle,
        /// Height of the extruded circle.
        pub height: f64,
    }

    /// NURBS surface definition.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
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
    #[derive(Clone, Debug, Deserialize, Serialize, gltf_derive::Validate)]
    #[serde(rename_all = "camelCase")]
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
    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
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
    #[derive(Clone, Debug, Deserialize, Serialize, gltf_derive::Validate)]
    #[serde(rename_all = "camelCase")]
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
    #[derive(Clone, Debug, Deserialize, Serialize, gltf_derive::Validate)]
    #[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    /// Same-sense orientation.
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
/// This extension takes advantage of the sign bit in all floating point numbers,
/// including positive and negative zero, to represent objects in their reverse
/// orientation. Positive indicates 'same-sense' orientation and negative indicates
/// 'reverse-sense' orientation.
#[derive(Clone, Copy, Debug)]
pub struct IndexWithOrientation<T: Validate> {
    /// Index absolute value.
    pub index: Index<T>,

    /// Selected orientation of the indexed item.
    pub orientation: Orientation,
}

impl<T: Validate> IndexWithOrientation<T> {
    /// Explicit constructor.
    pub fn new(index: Index<T>, orientation: Orientation) -> Self {
        Self { index, orientation }
    }

    /// Create an index with same-sense orientation.
    pub fn same(index: Index<T>) -> Self {
        Self::new(index, Orientation::Same)
    }

    /// Create an index with reverse-sense orientation.
    pub fn reverse(index: Index<T>) -> Self {
        Self::new(index, Orientation::Reverse)
    }

    /// Query whether the index has same-sense orientation.
    pub fn is_same(&self) -> bool {
        self.orientation.is_same()
    }

    /// Query whether the index has reverse-sense orientation.
    pub fn is_reverse(&self) -> bool {
        self.orientation.is_reverse()
    }
}

impl<'de, T: Validate> de::Deserialize<'de> for IndexWithOrientation<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        Ok(Self {
            index: Index::new(value.abs() as u32),
            orientation: if value.is_sign_negative() {
                Orientation::Reverse
            } else {
                Orientation::Same
            },
        })
    }
}

impl<T: Validate> ser::Serialize for IndexWithOrientation<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self.orientation {
            Orientation::Reverse => (-(self.index.value() as f64)).serialize(serializer),
            Orientation::Same => (self.index.value() as f64).serialize(serializer),
        }
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
        self.index.validate(root, path, report);
    }
}

/// Edge loop.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Loop {
    /// Oriented edges forming the loop.
    pub edges: Vec<IndexWithOrientation<Edge>>,

    /// Optional 1:1 pairing of UV curves to edges.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uv_curves: Vec<IndexWithOrientation<Curve>>,
}

/// Set of loops defined on an abstract surface.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Face {
    /// Surface the face edges and vertices reside on.
    pub surface: IndexWithOrientation<Surface>,

    /// Face outer bound.
    pub outer_loop: IndexWithOrientation<Loop>,

    /// Face inner bounds.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inner_loops: Vec<IndexWithOrientation<Loop>>,
}

/// Boundary representation volume.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Shell {
    /// Set of connected faces forming a closed 'watertight' volume.
    pub faces: Vec<IndexWithOrientation<Face>>,

    /// Optional name for this shell.
    #[cfg(feature = "names")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Solid boundary representation structure.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Solid {
    /// The outer boundary of the solid surface.
    pub outer_shell: IndexWithOrientation<Shell>,

    /// Optional set of inner shells, defining hollow regions of
    /// otherwise wholly solid objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inner_shells: Vec<IndexWithOrientation<Shell>>,

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
