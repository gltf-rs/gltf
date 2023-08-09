use crate::{json, Document, Mesh};

/// Curves.
pub mod curve {
    use crate::{json, Document};

    /// Defines a linear curve.
    #[derive(Clone, Debug)]
    pub struct Linear<'a> {
        /// The corresponding JSON struct.
        #[allow(dead_code)]
        pub(crate) json: &'a json::extensions::kittycad_boundary_representation::curve::Linear,
    }

    /// Defines a non-uniform rational B-spline (NURBS) curve.
    #[derive(Clone, Debug)]
    pub struct Nurbs<'a> {
        /// The corresponding JSON struct.
        #[allow(dead_code)]
        pub(crate) json: &'a json::extensions::kittycad_boundary_representation::curve::Nurbs,
    }

    /// Curve kind.
    #[derive(Clone, Debug)]
    pub enum Geometry<'a> {
        /// Linear curve.
        Linear(Linear<'a>),
        /// Non-uniform rational B-spline (NURBS) curve.
        Nurbs(Nurbs<'a>),
    }

    /// Abstract curve..
    #[derive(Clone, Debug)]
    pub struct Curve<'a> {
        /// The parent `Document` struct.
        #[allow(unused)]
        document: &'a Document,

        /// The corresponding JSON index.
        index: usize,

        /// The corresponding JSON struct.
        json: &'a json::extensions::kittycad_boundary_representation::Curve,
    }

    impl<'a> Curve<'a> {
        /// Constructs a `Curve`.
        pub(crate) fn new(
            document: &'a Document,
            index: usize,
            json: &'a json::extensions::kittycad_boundary_representation::Curve,
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
        pub fn name(&self) -> Option<&'a str> {
            self.json.name.as_deref()
        }

        /// Returns the specific curve parameters.
        pub fn geometry(&self) -> Geometry<'a> {
            match self.json.type_.unwrap() {
                json::extensions::kittycad_boundary_representation::curve::Type::Linear => {
                    let json = self.json.linear.as_ref().unwrap();
                    Geometry::Linear(Linear { json })
                }
                json::extensions::kittycad_boundary_representation::curve::Type::Nurbs => {
                    let json = self.json.nurbs.as_ref().unwrap();
                    Geometry::Nurbs(Nurbs { json })
                }
            }
        }
    }
}

/// Surfaces.
pub mod surface {
    use crate::{json, Document};

    /// Defines a planar surface.
    #[derive(Clone, Debug)]
    pub struct Plane<'a> {
        /// The corresponding JSON struct.
        #[allow(dead_code)]
        pub(crate) json: &'a json::extensions::kittycad_boundary_representation::surface::Plane,
    }

    impl<'a> Plane<'a> {
        /// Returns the normal vector to the plane.
        pub fn normal(&self) -> [f32; 3] {
            self.json.normal
        }

        /// Returns the value of `d` in the plane equation `n.r + d = 0`.
        pub fn constant(&self) -> f32 {
            todo!()
        }
    }

    /// Defines a non-uniform rational B-spline (NURBS) surface.
    #[derive(Clone, Debug)]
    pub struct Nurbs<'a> {
        /// The corresponding JSON struct.
        #[allow(dead_code)]
        pub(crate) json: &'a json::extensions::kittycad_boundary_representation::surface::Nurbs,
    }

    /// Surface manifold.
    #[derive(Clone, Debug)]
    pub enum Manifold<'a> {
        /// Planar surface.
        Plane(Plane<'a>),
        /// Non-uniform rational B-spline (NURBS) surface.
        Nurbs(Nurbs<'a>),
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
        json: &'a json::extensions::kittycad_boundary_representation::Surface,
    }

    impl<'a> Surface<'a> {
        /// Constructs a `Surface`.
        pub(crate) fn new(
            document: &'a Document,
            index: usize,
            json: &'a json::extensions::kittycad_boundary_representation::Surface,
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
        pub fn name(&self) -> Option<&'a str> {
            self.json.name.as_deref()
        }

        /// Returns the specific surface manifold.
        pub fn manifold(&self) -> Manifold<'a> {
            match self.json.type_.unwrap() {
                json::extensions::kittycad_boundary_representation::surface::Type::Plane => {
                    let json = self.json.plane.as_ref().unwrap();
                    Manifold::Plane(Plane { json })
                }
                json::extensions::kittycad_boundary_representation::surface::Type::Nurbs => {
                    let json = self.json.nurbs.as_ref().unwrap();
                    Manifold::Nurbs(Nurbs { json })
                }
            }
        }
    }
}

pub use curve::Curve;
pub use surface::Surface;

/// Boundary representation of a solid.
#[derive(Clone, Debug)]
pub struct BRep<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::extensions::kittycad_boundary_representation::BRep,
}

impl<'a> BRep<'a> {
    /// Constructs a `BRep`.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::extensions::kittycad_boundary_representation::BRep,
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
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns an `Iterator` that visits the faces of the B-rep.
    pub fn faces(&self) -> impl Iterator<Item = Face> {
        self.json
            .faces
            .iter()
            .map(|json| Face::new(self.clone(), json))
    }

    /// Returns the mesh approximation of this solid if defined.
    pub fn mesh(&self) -> Option<Mesh<'a>> {
        self.json
            .mesh
            .map(|index| self.document.meshes().nth(index.value()).unwrap())
    }
}

/// Set of vertices on a face plus trim curves.
#[derive(Clone, Debug)]
pub struct Loop<'a> {
    /// The parent `Face` struct.
    face: Face<'a>,

    /// The corresponding JSON struct.
    json: &'a json::extensions::kittycad_boundary_representation::brep::Loop,
}

impl<'a> Loop<'a> {
    /// Constructs a `Loop`.
    pub(crate) fn new(
        face: Face<'a>,
        json: &'a json::extensions::kittycad_boundary_representation::brep::Loop,
    ) -> Self {
        Self { face, json }
    }

    /// Returns the set of curves that define the loop.
    pub fn edges(&self) -> impl Iterator<Item = Edge> {
        self.json
            .edges
            .iter()
            .map(|json| Edge::new(self.clone(), json))
    }
}

/// Boundary representation of a solid.
#[derive(Clone, Debug)]
pub struct Face<'a> {
    /// The parent `BRep` struct.
    brep: BRep<'a>,

    /// The corresponding JSON struct.
    json: &'a json::extensions::kittycad_boundary_representation::brep::Face,
}

impl<'a> Face<'a> {
    /// Constructs a `Face`.
    pub(crate) fn new(
        brep: BRep<'a>,
        json: &'a json::extensions::kittycad_boundary_representation::brep::Face,
    ) -> Self {
        Self { brep, json }
    }

    /// Returns the face outer loop.
    pub fn outer_loop(&self) -> Loop<'a> {
        Loop::new(self.clone(), &self.json.outer_loop)
    }

    /// The surface this face is defined upon.
    pub fn surface(&self) -> Surface<'a> {
        self.brep
            .document
            .surfaces()
            .unwrap()
            .nth(self.json.surface.value())
            .unwrap()
    }
}

/// Face bound.
#[derive(Clone, Debug)]
pub struct Edge<'a> {
    /// The parent `Loop` struct.
    parent: Loop<'a>,

    /// The corresponding JSON struct.
    json: &'a json::extensions::kittycad_boundary_representation::brep::Edge,
}

impl<'a> Edge<'a> {
    /// Constructs an `Edge`.
    pub(crate) fn new(
        parent: Loop<'a>,
        json: &'a json::extensions::kittycad_boundary_representation::brep::Edge,
    ) -> Self {
        Self { parent, json }
    }
}
