use crate::{Document, Mesh};
use json::extensions::kittycad_boundary_representation as kcad;

#[doc(inline)]
pub use kcad::Orientation;

#[doc(inline)]
pub use curve::Curve;

#[doc(inline)]
pub use surface::Surface;

/// Curves.
pub mod curve {
    use crate::Document;
    use euler::{DVec3, DVec4};
    use json::extensions::kittycad_boundary_representation as kcad;

    #[doc(inline)]
    pub use kcad::curve::Domain;

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
    #[derive(Clone, Debug)]
    pub struct Circle<'a> {
        /// The corresponding JSON struct.
        pub(crate) json: &'a kcad::curve::Circle,

        /// The curve domain.
        pub(crate) domain: Option<Domain>,
    }

    impl<'a> Circle<'a> {
        /// Position at the center of the circle.
        pub fn origin(&self) -> Option<[f64; 3]> {
            self.json.origin
        }

        /// Distance from the center position to all points on the circle.
        pub fn radius(&self) -> f64 {
            self.json.radius
        }

        /// Normal vector to the plane containing the circle.
        ///
        /// This serves as the Z basis in the parametric co-ordinate space.
        pub fn normal(&self) -> [f64; 3] {
            self.json.normal
        }

        /// Unit vector in the direction from the origin to the point on
        /// the circle at λ(0).
        ///
        /// Due to floating point precision, this vector may not lie exactly
        /// in the plane. If this is the case then the X vector is treated
        /// as the projection of this vector onto the plane.
        pub fn xbasis(&self) -> [f64; 3] {
            self.json.xbasis
        }

        /// Evaluate the curve at parameter value `t`.
        pub fn evaluate(&self, t: f64) -> [f64; 3] {
            let radius = self.json.radius;
            let origin = DVec3::from(self.json.origin.unwrap_or_default());
            let xbasis = DVec3::from(self.json.xbasis);
            let normal = DVec3::from(self.json.normal);
            let ybasis = normal.cross(xbasis);
            let (sine, cosine) = t.sin_cos();
            (origin + (xbasis * cosine + ybasis * sine) * radius).into()
        }

        /// Point evaluated at the domain minimum value.
        pub fn start(&self) -> [f64; 3] {
            if let Some(Domain { min, .. }) = self.domain {
                self.evaluate(min)
            } else {
                self.evaluate(0.0)
            }
        }

        /// Point evaluated at the domain maximum value.
        pub fn end(&self) -> [f64; 3] {
            if let Some(Domain { max, .. }) = self.domain {
                self.evaluate(max)
            } else {
                self.start()
            }
        }
    }

    /// Defines a line curve.
    #[derive(Clone, Debug)]
    pub struct Line<'a> {
        /// The corresponding JSON struct.
        pub(crate) json: &'a kcad::curve::Line,

        /// The curve domain.
        pub(crate) domain: Domain,
    }

    impl<'a> Line<'a> {
        /// Returns the line origin.
        pub fn start(&self) -> [f64; 3] {
            self.json.start
        }

        /// Evaluate the curve at parameter value `t`.
        pub fn evaluate(&self, t: f64) -> [f64; 3] {
            let start = DVec3::from(self.start());
            let end = DVec3::from(self.end());
            (start + t * (end - start)).into()
        }

        /// Returns the line end point.
        ///
        /// If `direction` was set, this will be computed from the trim domain.
        pub fn end(&self) -> [f64; 3] {
            if let Some(end) = self.json.end {
                end
            } else {
                let start = DVec3::from(self.start());
                let direction = DVec3::from(self.json.direction.unwrap());
                let end = start + direction * (self.domain.max - self.domain.min);
                end.into()
            }
        }

        /// Returns the line direction.
        ///
        /// If `end` was set, this will be computed.
        pub fn direction(&self) -> [f64; 3] {
            if let Some(direction) = self.json.direction {
                direction
            } else {
                let start = DVec3::from(self.start());
                let end = DVec3::from(self.json.end.unwrap());
                let difference = end + start * -1.0;
                let direction = difference.normalize();
                direction.into()
            }
        }
    }

    /// Defines a non-uniform rational B-spline (NURBS) curve.
    #[derive(Clone, Debug)]
    pub struct Nurbs<'a> {
        /// The corresponding JSON struct.
        #[allow(dead_code)]
        pub(crate) json: &'a kcad::curve::Nurbs,
    }

    impl<'a> Nurbs<'a> {
        /// Evaluate the curve at parameter value `t`.
        pub fn evaluate(&self, t: f64) -> [f64; 3] {
            // Min/max knot value
            let (umin, umax) = {
                let u = self.knot_vector();
                (*u.first().unwrap(), *u.last().unwrap())
            };

            if t == umin {
                self.start()
            } else if t == umax {
                self.end()
            } else {
                // Degree
                let d = (self.json.order - 1) as usize;

                // Padded knot vector
                let mut u = self.knot_vector().to_vec();
                for _ in 0..d {
                    u.insert(0, umin);
                    u.push(umax);
                }

                // Index of knot interval
                let k = u
                    .windows(2)
                    .position(|ui| t >= ui[0] && t < ui[1])
                    .expect("t does not lie between any knot");

                // Multiplicity
                let m = u
                    .windows(2)
                    .rev()
                    .position(|ui| t >= ui[0] && t < ui[1])
                    .map(|ui| k - ui)
                    .expect("t does not lie between any knot");

                // New control points
                let mut p = self
                    .control_points()
                    .iter()
                    .cloned()
                    .map(|[x, y, z, w]| DVec4::new(x * w, y * w, z * w, w))
                    .collect::<Vec<_>>();

                let h = d - m;
                for r in 1..=h {
                    for i in (r..=h).rev() {
                        let upper = t - u[k + i - d];
                        let lower = u[k + i + 1 - r] - u[k + i - d];
                        let a = if lower == 0.0 { 0.0 } else { upper / lower };
                        p[i] = (1.0 - a) * p[i - 1] + a * p[i];
                    }
                }

                (p[d].xyz() / p[d].w).into()
            }
        }

        /// Returns the curve start point, i.e., the first control point.
        pub fn start(&self) -> [f64; 3] {
            // TODO: evaluate for domain.
            let v = self.json.control_points[0];
            [v[0], v[1], v[2]]
        }

        /// Returns the curve end point, i.e., the last control point.
        pub fn end(&self) -> [f64; 3] {
            // TODO: evaluate for domain.
            let v = self.json.control_points[self.json.control_points.len() - 1];
            [v[0], v[1], v[2]]
        }

        /// Returns the NURBS control points.
        pub fn control_points(&self) -> &[[f64; 4]] {
            &self.json.control_points
        }

        /// Returns the NURBS knot vector.
        pub fn knot_vector(&self) -> &[f64] {
            &self.json.knot_vector
        }

        /// Returns the order of the basis splines.
        ///
        /// # Notes
        ///
        /// The degree of the basis splines is one less than the order.
        pub fn order(&self) -> u32 {
            self.json.order
        }
    }

    /// Curve kind.
    #[derive(Clone, Debug)]
    pub enum Geometry<'a> {
        /// Circular curve.
        Circle(Circle<'a>),
        /// Linear curve.
        Line(Line<'a>),
        /// Non-uniform rational B-spline (NURBS) curve.
        Nurbs(Nurbs<'a>),
    }

    /// Abstract curve.
    #[derive(Clone, Debug)]
    pub struct Curve<'a> {
        /// The parent `Document` struct.
        #[allow(dead_code)]
        document: &'a Document,

        /// The corresponding JSON index.
        index: usize,

        /// The corresponding JSON struct.
        json: &'a kcad::Curve,
    }

    impl<'a> Curve<'a> {
        /// Constructs a `Curve`.
        pub fn new(document: &'a Document, index: usize, json: &'a kcad::Curve) -> Self {
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
        pub fn name(&self) -> Option<&'a str> {
            self.json.name.as_deref()
        }

        /// Evaluates the curve start point.
        pub fn start(&self) -> [f64; 3] {
            match self.geometry() {
                Geometry::Circle(circle) => circle.start(),
                Geometry::Line(line) => line.start(),
                Geometry::Nurbs(nurbs) => nurbs.start(),
            }
        }

        /// Evaluates the curve end point.
        pub fn end(&self) -> [f64; 3] {
            match self.geometry() {
                Geometry::Circle(circle) => circle.end(),
                Geometry::Line(line) => line.end(),
                Geometry::Nurbs(nurbs) => nurbs.end(),
            }
        }

        /// Evaluate the curve at parameter value `t`.
        pub fn evaluate(&self, t: f64) -> [f64; 3] {
            match self.geometry() {
                Geometry::Circle(circle) => circle.evaluate(t),
                Geometry::Line(line) => line.evaluate(t),
                Geometry::Nurbs(nurbs) => nurbs.evaluate(t),
            }
        }

        /// Returns the specific curve parameters.
        pub fn geometry(&self) -> Geometry<'a> {
            match self.json.type_.unwrap() {
                kcad::curve::Type::Circle => {
                    let json = self.json.circle.as_ref().unwrap();
                    let domain = self.json.domain.clone();
                    Geometry::Circle(Circle { json, domain })
                }
                kcad::curve::Type::Line => {
                    let json = self.json.line.as_ref().unwrap();
                    let domain = self.json.domain.clone().unwrap_or_default();
                    Geometry::Line(Line { json, domain })
                }
                kcad::curve::Type::Nurbs => {
                    let json = self.json.nurbs.as_ref().unwrap();
                    Geometry::Nurbs(Nurbs { json })
                }
            }
        }

        /// Returns the range of the curve evaluation parameter.
        ///
        /// When the domain is `None`, assume 0 <= t <= 1.
        pub fn domain(&self) -> Option<Domain> {
            self.json.domain.clone()
        }
    }

    #[cfg(test)]
    mod tests {
        use gltf_json::extensions::kittycad_boundary_representation as kcad_json;
        use std::f64::consts::{FRAC_1_SQRT_2, PI};

        macro_rules! all_relative_eq {
            ($expected:expr, $actual:expr) => {{
                $expected
                    .iter()
                    .copied()
                    .zip($actual.iter().copied())
                    .all(|(a, b)| approx::relative_eq!(a, b, epsilon = 0.001))
            }};

            ($expected:expr, $actual:expr, epsilon = $epsilon:expr) => {{
                $expected
                    .iter()
                    .copied()
                    .zip($actual.iter().copied())
                    .all(|(a, b)| approx::relative_eq!(a, b, epsilon = $epsilon))
            }};
        }

        #[test]
        fn evaluate_nurbs_arc_quadratic() {
            let inner = kcad_json::curve::Nurbs {
                control_points: vec![
                    [1.0, 0.0, 0.0, 1.0],
                    [1.0, 1.0, 0.0, FRAC_1_SQRT_2],
                    [0.0, 1.0, 0.0, 1.0],
                ],
                knot_vector: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
                order: 3,
            };
            let curve = super::Nurbs { json: &inner };

            let test_points = [
                (0.0, [1.0, 0.0, 0.0]),
                (0.25, [(PI * 0.125).cos(), (PI * 0.125).sin(), 0.0]),
                (0.5, [(PI * 0.25).cos(), (PI * 0.25).sin(), 0.0]),
                (0.75, [(PI * 0.375).cos(), (PI * 0.375).sin(), 0.0]),
                (1.0, [0.0, 1.0, 0.0]),
            ];

            for (i, (a, b)) in test_points.iter().copied().enumerate() {
                if !all_relative_eq!(curve.evaluate(a), b, epsilon = 0.1) {
                    panic!(
                        "test_points[{i}]: curve.evaluate({a:?}) = {:?} != {b:?}",
                        curve.evaluate(a)
                    );
                }
            }
        }

        #[test]
        fn evaluate_nurbs_half_circle_cubic() {
            let inner = kcad_json::curve::Nurbs {
                control_points: vec![
                    [1.0, 0.0, 0.0, 1.0],
                    [1.0, 2.0, 0.0, 1.0 / 3.0],
                    [-1.0, 2.0, 0.0, 1.0 / 3.0],
                    [-1.0, 0.0, 0.0, 1.0],
                ],
                knot_vector: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
                order: 4,
            };
            let curve = super::Nurbs { json: &inner };

            let test_points = [
                (0.0, [1.0, 0.0, 0.0]),
                (0.25, [0.8, 0.6, 0.0]),
                (0.5, [0.0, 1.0, 0.0]),
                (0.75, [-0.8, 0.6, 0.0]),
                (1.0, [-1.0, 0.0, 0.0]),
            ];

            for (i, (a, b)) in test_points.iter().copied().enumerate() {
                if !all_relative_eq!(curve.evaluate(a), b, epsilon = 0.1) {
                    panic!(
                        "test_points[{i}]: curve.evaluate({a:?}) = {:.2?} != {b:.2?}",
                        curve.evaluate(a)
                    );
                }
            }
        }

        #[test]
        fn evaluate_circle_basic() {
            let curve = super::Circle {
                json: &kcad_json::curve::Circle {
                    origin: Some([0.0, 0.0, 0.0]),
                    radius: 2.0,
                    normal: [0.0, 0.0, 1.0],
                    xbasis: [1.0, 0.0, 0.0],
                },
                domain: None,
            };

            let test_points = [
                (0.0, [2.0, 0.0, 0.0]),
                (0.5 * PI, [0.0, 2.0, 0.0]),
                (PI, [-2.0, 0.0, 0.0]),
                (-0.5 * PI, [0.0, -2.0, 0.0]),
            ];

            for (i, (a, b)) in test_points.iter().copied().enumerate() {
                if !all_relative_eq!(curve.evaluate(a), b) {
                    panic!(
                        "test_points[{i}]: curve.evaluate({a:?}) = {:?} != {b:?}",
                        curve.evaluate(a)
                    );
                }
            }
        }

        #[test]
        fn evaluate_circle_offset() {
            let curve = super::Circle {
                json: &kcad_json::curve::Circle {
                    origin: Some([1.2, 3.4, 5.6]),
                    radius: 2.0,
                    normal: [0.0, 0.0, 1.0],
                    xbasis: [1.0, 0.0, 0.0],
                },
                domain: None,
            };

            let test_points = [
                (0.0, [3.2, 3.4, 5.6]),
                (0.5 * PI, [1.2, 5.4, 5.6]),
                (PI, [-0.8, 3.4, 5.6]),
                (-0.5 * PI, [1.2, 1.4, 5.6]),
            ];

            for (i, (a, b)) in test_points.iter().copied().enumerate() {
                if !all_relative_eq!(curve.evaluate(a), b) {
                    panic!(
                        "test_points[{i}]: curve.evaluate({a:?}) = {:?} != {b:?}",
                        curve.evaluate(a)
                    );
                }
            }
        }
    }
}

/// Surfaces.
pub mod surface {
    use crate::Document;
    use euler::DVec3;
    use json::extensions::kittycad_boundary_representation as kcad;

    #[doc(inline)]
    pub use kcad::surface::Domain;

    /// Parametric cylindrical surface definition.
    ///
    /// σ(u, v) := O + R(cos(u)x + sin(u)y) + vz, where:
    /// * O = `self.circle().origin()`,
    /// * R = `self.circle().radius()`,
    /// * x = `self.circle().xbasis()`,
    /// * y = `self.circle().ybasis()`,
    /// * z = `self.circle().zbasis()`.
    ///
    /// In the field documentation, the 'base circle' is
    /// defined as the cycle defined at σ(u, 0).
    ///
    /// The vectors `xbasis`, `ybasis`, and `zbasis` form
    /// an orthonormal set.
    #[derive(Clone, Debug)]
    pub struct Cylinder<'a> {
        /// The corresponding JSON struct.
        pub(crate) json: &'a kcad::surface::Cylinder,
    }

    impl<'a> Cylinder<'a> {
        /// The extruded circle.
        pub fn circle(&self) -> super::curve::Circle<'a> {
            super::curve::Circle {
                json: &self.json.circle,
                domain: None,
            }
        }

        /// Height of the extruded circle.
        pub fn height(&self) -> f64 {
            self.json.height
        }
    }

    /// Defines a planar surface.
    #[derive(Clone, Debug)]
    pub struct Plane<'a> {
        /// The corresponding JSON struct.
        pub(crate) json: &'a kcad::surface::Plane,
    }

    impl<'a> Plane<'a> {
        /// Returns the normal vector to the plane.
        pub fn normal(&self) -> [f64; 3] {
            self.json.normal
        }

        /// Returns the value of `d` in the plane equation `n.r = d`.
        pub fn constant(&self) -> f64 {
            // TODO: compute constant where not provided.
            self.json.constant.unwrap()
        }

        /// Returns an arbitrary point that lies on the plane.
        pub fn point(&self) -> [f64; 3] {
            // TODO: compute point where not provided.
            self.json.point.unwrap()
        }
    }

    /// Parametric spherical surface definition.
    ///
    /// σ(u, v) := O + Rcos(v)(cos(u)x + sin(u)y) + Rsin(v)z, where:
    /// * O = `self.horizon().origin()`,
    /// * R = `self.horizon().radius()`,
    /// * x = `self.horizon().xbasis()`,
    /// * y = `self.horizon().normal()` × `self.horizon().xbasis()`,
    /// * z = `self.horizon().normal()`,
    /// * u ∈ {0, 2π},
    /// * v ∈ {0, 2π}.
    ///
    /// Spheres are defined in reference to a circle at zero inclination.
    #[derive(Clone, Debug)]
    pub struct Sphere<'a> {
        /// The corresponding JSON struct.
        pub(crate) json: &'a kcad::surface::Sphere,
    }

    impl<'a> Sphere<'a> {
        /// The circle at zero inclination.
        pub fn horizon(&self) -> super::curve::Circle<'a> {
            super::curve::Circle {
                json: &self.json.horizon,
                domain: None,
            }
        }

        /// Evaluate the sphere at (u, v).
        pub fn evaluate(&self, [u, v]: [f64; 2]) -> [f64; 3] {
            let horizon = self.horizon();
            let r = horizon.radius();
            let a = DVec3::from(horizon.origin().unwrap_or_default());
            let x = DVec3::from(horizon.xbasis());
            let z = DVec3::from(horizon.normal());
            let y = z.cross(x);
            let (sin_u, cos_u) = u.sin_cos();
            let (sin_v, cos_v) = v.sin_cos();
            let b = a + r * cos_v * (cos_u * x + sin_u * y) + r * sin_v * z;
            b.into()
        }

        /// Find (u, v) for a point (x, y, z) on the sphere.
        ///
        /// The result is unspecified if (x, y, z) does not lie on the sphere
        /// within a reasonable tolerance.
        pub fn evaluate_inverse(&self, [x, y, z]: [f64; 3]) -> [f64; 2] {
            let horizon = self.horizon();
            let r = horizon.radius();
            let a = DVec3::from(horizon.origin().unwrap_or_default());
            let b = DVec3::new(x, y, z);
            let x = DVec3::from(horizon.xbasis());
            let z = DVec3::from(horizon.normal());
            let y = z.cross(x);
            let ab = b - a;
            let u = ab.dot(y).atan2(ab.dot(x));
            let v = (ab.dot(z) / r).asin();
            [u, v]
        }
    }

    /// Toroidal surface definition.
    ///
    /// σ(u, v) := O + (R + rcos(v))(cos(u)x + sin(u)y) + rsin(v)z, where:
    /// * O = `self.origin()`,
    /// * R = `self.circle().major_radius()`,
    /// * r = `self.circle().minor_radius()`,
    /// * x = `self.circle().xbasis()`,
    /// * y = `self.circle().ybasis()`,
    /// * z = `self.circle().zbasis()`.
    #[derive(Clone, Debug)]
    pub struct Torus<'a> {
        /// The corresponding JSON struct.
        pub(crate) json: &'a kcad::surface::Torus,
    }

    impl<'a> Torus<'a> {
        /// The center of the torus.
        pub fn origin(&self) -> [f64; 3] {
            self.json.origin
        }

        /// The revolved circle.
        pub fn circle(&self) -> super::curve::Circle<'a> {
            super::curve::Circle {
                json: &self.json.circle,
                domain: None,
            }
        }

        /// Distance from the origin to the origin of the base circle.
        pub fn radius(&self) -> f64 {
            self.json.radius
        }
    }

    /// Defines a non-uniform rational B-spline (NURBS) surface.
    #[derive(Clone, Debug)]
    pub struct Nurbs<'a> {
        /// The corresponding JSON struct.
        #[allow(dead_code)]
        pub(crate) json: &'a kcad::surface::Nurbs,
    }

    impl<'a> Nurbs<'a> {
        /// Returns the matrix of control points.
        pub fn control_points(&self) -> &[[f64; 4]] {
            &self.json.control_points
        }

        /// Returns the dimensions of the control point matrix.
        pub fn num_control_points(&self) -> [u32; 2] {
            self.json.num_control_points
        }

        /// Returns the number of knots in the U and V curves respectively.
        pub fn num_knots(&self) -> [u32; 2] {
            self.json.num_knots
        }

        /// Returns the knot vectors for the U and V curves respectively.
        pub fn knot_vectors(&self) -> (&[f64], &[f64]) {
            self.json
                .knot_vector
                .split_at(self.json.num_knots[0] as usize)
        }

        /// Returns the order of basis splines for the U and V curves respectively.
        pub fn orders(&self) -> [u32; 2] {
            self.json.order
        }
    }

    /// Specific surface geometry.
    #[derive(Clone, Debug)]
    pub enum Geometry<'a> {
        /// Cylindrical surface.
        Cylinder(Cylinder<'a>),
        /// Non-uniform rational B-spline (NURBS) surface.
        Nurbs(Nurbs<'a>),
        /// Planar surface.
        Plane(Plane<'a>),
        /// Spherical surface.
        Sphere(Sphere<'a>),
        /// Toroidal surface.
        Torus(Torus<'a>),
    }

    /// Abstract surface.
    #[derive(Clone, Debug)]
    pub struct Surface<'a> {
        /// The parent `Document` struct.
        #[allow(unused)]
        document: &'a Document,

        /// The corresponding JSON index.
        index: usize,

        /// The corresponding JSON struct.
        json: &'a kcad::Surface,
    }

    impl<'a> Surface<'a> {
        /// Constructs a `Surface`.
        pub(crate) fn new(document: &'a Document, index: usize, json: &'a kcad::Surface) -> Self {
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
        pub fn name(&self) -> Option<&'a str> {
            self.json.name.as_deref()
        }

        /// Returns the specific surface geometry.
        pub fn geometry(&self) -> Geometry<'a> {
            match self.json.type_.unwrap() {
                kcad::surface::Type::Cylinder => {
                    let json = self.json.cylinder.as_ref().unwrap();
                    Geometry::Cylinder(Cylinder { json })
                }
                kcad::surface::Type::Nurbs => {
                    let json = self.json.nurbs.as_ref().unwrap();
                    Geometry::Nurbs(Nurbs { json })
                }
                kcad::surface::Type::Plane => {
                    let json = self.json.plane.as_ref().unwrap();
                    Geometry::Plane(Plane { json })
                }
                kcad::surface::Type::Sphere => {
                    let json = self.json.sphere.as_ref().unwrap();
                    Geometry::Sphere(Sphere { json })
                }
                kcad::surface::Type::Torus => {
                    let json = self.json.torus.as_ref().unwrap();
                    Geometry::Torus(Torus { json })
                }
            }
        }

        /// Returns the range of the surface evaluation parameters.
        ///
        /// When the domain is `None`, assume 0 <= u <= 1 and 0 <= v <= 1.
        pub fn domain(&self) -> Option<Domain> {
            self.json.domain.clone()
        }
    }

    #[cfg(test)]
    mod tests {
        use gltf_json::extensions::kittycad_boundary_representation as kcad_json;
        use std::f64::consts::PI;

        macro_rules! all_relative_eq {
            ($expected:expr, $actual:expr) => {{
                $expected
                    .iter()
                    .copied()
                    .zip($actual.iter().copied())
                    .all(|(a, b)| approx::relative_eq!(a, b, epsilon = 0.001))
            }};
        }

        #[test]
        fn evaluate_sphere_basic() {
            let sphere = super::Sphere {
                json: &kcad_json::surface::Sphere {
                    horizon: kcad_json::curve::Circle {
                        origin: Some([0.0, 0.0, 0.0]),
                        radius: 2.0,
                        normal: [0.0, 0.0, 1.0],
                        xbasis: [1.0, 0.0, 0.0],
                    },
                },
            };

            let test_points = [
                ([0.0, 0.0], [2.0, 0.0, 0.0]),
                ([0.5 * PI, 0.0], [0.0, 2.0, 0.0]),
                ([PI, 0.0], [-2.0, 0.0, 0.0]),
                ([-0.5 * PI, 0.0], [0.0, -2.0, 0.0]),
                ([0.0, 0.5 * PI], [0.0, 0.0, 2.0]),
                ([0.0, -0.5 * PI], [0.0, 0.0, -2.0]),
            ];

            for (i, (a, b)) in test_points.iter().copied().enumerate() {
                if !all_relative_eq!(sphere.evaluate(a), b) {
                    panic!(
                        "test_points[{i}]: sphere.evaluate({a:?}) = {:?} != {b:?}",
                        sphere.evaluate(a)
                    );
                }
                if !all_relative_eq!(sphere.evaluate_inverse(b), a) {
                    panic!(
                        "test_points[{i}]: sphere.evaluate_inverse({b:?}) = {:?} != {a:?}",
                        sphere.evaluate_inverse(b)
                    );
                }
            }
        }

        #[test]
        fn evaluate_sphere_offset() {
            let offset = [1.2, 3.4, 5.6];
            let sphere = super::Sphere {
                json: &kcad_json::surface::Sphere {
                    horizon: kcad_json::curve::Circle {
                        origin: Some(offset),
                        radius: 2.0,
                        normal: [0.0, 0.0, 1.0],
                        xbasis: [1.0, 0.0, 0.0],
                    },
                },
            };

            let test_points = [
                ([0.0, 0.0], [3.2, 3.4, 5.6]),
                ([0.5 * PI, 0.0], [1.2, 5.4, 5.6]),
                ([PI, 0.0], [-0.8, 3.4, 5.6]),
                ([-0.5 * PI, 0.0], [1.2, 1.4, 5.6]),
                ([0.0, 0.5 * PI], [1.2, 3.4, 7.6]),
                ([0.0, -0.5 * PI], [1.2, 3.4, 3.6]),
            ];

            for (i, (a, b)) in test_points.iter().copied().enumerate() {
                if !all_relative_eq!(sphere.evaluate(a), b) {
                    panic!(
                        "test_points[{i}]: sphere.evaluate({a:?}) = {:?} != {b:?}",
                        sphere.evaluate(a)
                    );
                }
                if !all_relative_eq!(sphere.evaluate_inverse(b), a) {
                    panic!(
                        "test_points[{i}]: sphere.evaluate_inverse({b:?}) = {:?} != {a:?}",
                        sphere.evaluate_inverse(b)
                    );
                }
            }
        }
    }
}

/// Solid boundary representation structure.
#[derive(Clone, Debug)]
pub struct Solid<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a kcad::Solid,
}

impl<'a> Solid<'a> {
    /// Constructs a `BRep`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a kcad::Solid) -> Self {
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
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns an `Iterator` that visits the solid's shells.
    pub fn shells(&self) -> impl ExactSizeIterator<Item = (Shell<'a>, Orientation)> {
        self.json
            .shells
            .iter()
            .map(|kcad::IndexWithOrientation(index, orientation)| {
                let shell = self.document.shells().unwrap().nth(index.value()).unwrap();
                (shell, *orientation)
            })
    }

    /// Returns the mesh approximation of this solid if defined.
    pub fn mesh(&self) -> Option<Mesh<'a>> {
        self.json
            .mesh
            .map(|index| self.document.meshes().nth(index.value()).unwrap())
    }
}

/// Closed boundary representation volume.
#[derive(Clone, Debug)]
pub struct Shell<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a kcad::Shell,
}

impl<'a> Shell<'a> {
    /// Constructs a `Shell`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a kcad::Shell) -> Self {
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
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns an `Iterator` that visits the faces of the shell.
    pub fn faces(&self) -> impl ExactSizeIterator<Item = (Face<'a>, Orientation)> {
        self.json
            .faces
            .iter()
            .map(|kcad::IndexWithOrientation(index, orientation)| {
                let face = self.document.faces().unwrap().nth(index.value()).unwrap();
                (face, *orientation)
            })
    }
}

/// Set of vertices on a face plus trim curves.
#[derive(Clone, Debug)]
pub struct Loop<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a kcad::Loop,
}

impl<'a> Loop<'a> {
    /// Constructs a `Loop`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a kcad::Loop) -> Self {
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

    /// Returns an iterator that visits the edges of the loop.
    pub fn edges(&self) -> impl ExactSizeIterator<Item = (Edge<'a>, Orientation)> {
        self.json
            .edges
            .iter()
            .map(|kcad::IndexWithOrientation(index, orientation)| {
                let edge = self.document.edges().unwrap().nth(index.value()).unwrap();
                (edge, *orientation)
            })
    }

    /// Returns an iterator that visits the UV curves of the loop.
    pub fn uv_curves(&self) -> impl ExactSizeIterator<Item = (Curve<'a>, Orientation)> {
        self.json
            .uv_curves
            .iter()
            .map(|kcad::IndexWithOrientation(index, orientation)| {
                let curve = self.document.curves().unwrap().nth(index.value()).unwrap();
                (curve, *orientation)
            })
    }
}

/// Boundary representation of a solid.
#[derive(Clone, Debug)]
pub struct Face<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a kcad::Face,
}

impl<'a> Face<'a> {
    /// Constructs a `Face`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a kcad::Face) -> Self {
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

    /// Returns the face bounds.
    pub fn loops(&self) -> impl ExactSizeIterator<Item = (Loop<'a>, Orientation)> {
        self.json
            .loops
            .iter()
            .map(|kcad::IndexWithOrientation(index, orientation)| {
                let loop_ = self.document.loops().unwrap().nth(index.value()).unwrap();
                (loop_, *orientation)
            })
    }

    /// The surface this face is defined upon.
    pub fn surface(&self) -> (Surface<'a>, Orientation) {
        let surface = self
            .document
            .surfaces()
            .unwrap()
            .nth(self.json.surface.index().value())
            .unwrap();
        (surface, self.json.surface.orientation())
    }
}

/// Vertex in 3D space, joining edges.
#[derive(Clone, Debug)]
pub struct Vertex<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a kcad::Vertex,
}

impl<'a> Vertex<'a> {
    /// Constructs a `Vertex`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a kcad::Vertex) -> Self {
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

    /// Returns the vertex position in 3D space.
    pub fn position(&self) -> [f64; 3] {
        self.json.0
    }
}

/// Face bound.
#[derive(Clone, Debug)]
pub struct Edge<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a kcad::Edge,
}

/// Edge geometry.
pub enum Endpoints<'a> {
    /// This edge forms a loop.
    Closed,
    /// This edge has a distinct start and end vertex.
    Open {
        /// Edge start vertex.
        start: Vertex<'a>,
        /// Edge end vertex.
        end: Vertex<'a>,
    },
}

impl<'a> Edge<'a> {
    /// Constructs an `Edge`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a kcad::Edge) -> Self {
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

    /// Returns the edge curve geometry in 3D (or homogeneous 4D) space.
    pub fn curve(&self) -> (Curve<'a>, Orientation) {
        let kcad::IndexWithOrientation(index, orientation) = self.json.curve;
        let curve = self.document.curves().unwrap().nth(index.value()).unwrap();
        (curve, orientation)
    }

    /// Edge endpoints.
    ///
    /// Returns `None` if the edge is closed.
    pub fn endpoints(&self) -> Endpoints<'a> {
        if self.json.closed {
            Endpoints::Closed
        } else {
            let start = {
                let index = self.json.start.unwrap().value();
                self.document.vertices().unwrap().nth(index).unwrap()
            };
            let end = {
                let index = self.json.end.unwrap().value();
                self.document.vertices().unwrap().nth(index).unwrap()
            };
            Endpoints::Open { start, end }
        }
    }

    /// Returns the optional subdomain that selects a subset of the curve.
    ///
    /// If this is `None` the the edge spans the whole domain of the curve.
    pub fn subdomain(&self) -> Option<curve::Domain> {
        self.json.subdomain.clone()
    }
}
