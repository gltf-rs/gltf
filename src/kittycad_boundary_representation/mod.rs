use crate::{json, Document, Mesh};

/// Curves.
pub mod curve {
    use crate::{json, Document};

    /// Defines  non-uniform rational B-spline (NURBS) curve.
    #[derive(Clone, Debug)]
    pub struct Nurbs<'a> {
        /// The corresponding JSON struct.
        #[allow(dead_code)]
        pub(crate) json: &'a json::extensions::kittycad_boundary_representation::curve::Nurbs,
    }

    /// Curve kind.
    #[derive(Clone, Debug)]
    pub enum Kind<'a> {
        /// Linear curve.
        Linear,
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

        /// Returns the start point.
        pub fn start(&self) -> [f32; 4] {
            self.json.start
        }

        /// Returns the end point.
        pub fn end(&self) -> [f32; 4] {
            self.json.end
        }

        /// Returns the specific curve parameters.
        pub fn kind(&self) -> Kind<'a> {
            match self.json.type_.unwrap() {
                json::extensions::kittycad_boundary_representation::curve::Type::Linear => {
                    Kind::Linear
                }
                json::extensions::kittycad_boundary_representation::curve::Type::Nurbs => {
                    let json = self.json.nurbs.as_ref().unwrap();
                    Kind::Nurbs(Nurbs { json })
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
            self.json.constant
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

/// Iterators.
pub mod iter {
    /// An `Iterator` that visits the faces of a `BRep`.
    #[derive(Clone, Debug)]
    pub struct Faces<'a> {
        /// The parent `BRep` struct.
        pub(crate) brep: super::BRep<'a>,

        /// The internal JSON primitive iterator.
        pub(crate) iter:
            std::slice::Iter<'a, json::extensions::kittycad_boundary_representation::brep::Face>,
    }

    impl<'a> ExactSizeIterator for Faces<'a> {}
    impl<'a> Iterator for Faces<'a> {
        type Item = super::Face<'a>;
        fn next(&mut self) -> Option<Self::Item> {
            self.iter
                .next()
                .map(|json| super::Face::new(self.brep.clone(), json))
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
        fn count(self) -> usize {
            self.iter.count()
        }
        fn last(self) -> Option<Self::Item> {
            self.iter
                .clone()
                .last()
                .map(|json| super::Face::new(self.brep.clone(), json))
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.iter
                .nth(n)
                .map(|json| super::Face::new(self.brep.clone(), json))
        }
    }

    /// An `Iterator` that visits the curves of a `Loop`.
    #[derive(Clone, Debug)]
    pub struct Curves<'a> {
        /// The parent `Loop` struct.
        pub(crate) parent: super::Loop<'a>,

        /// The internal JSON primitive iterator.
        pub(crate) iter: std::slice::Iter<
            'a,
            json::Index<json::extensions::kittycad_boundary_representation::Curve>,
        >,
    }

    impl<'a> ExactSizeIterator for Curves<'a> {}
    impl<'a> Iterator for Curves<'a> {
        type Item = super::Curve<'a>;
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map(|index| {
                self.parent
                    .face
                    .brep
                    .document
                    .curves()
                    .unwrap()
                    .nth(index.value())
                    .unwrap()
            })
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
        fn count(self) -> usize {
            self.iter.count()
        }
        fn last(self) -> Option<Self::Item> {
            self.iter.clone().last().map(|index| {
                self.parent
                    .face
                    .brep
                    .document
                    .curves()
                    .unwrap()
                    .nth(index.value())
                    .unwrap()
            })
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.iter.nth(n).map(|index| {
                self.parent
                    .face
                    .brep
                    .document
                    .curves()
                    .unwrap()
                    .nth(index.value())
                    .unwrap()
            })
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
    pub fn faces(&self) -> iter::Faces<'a> {
        iter::Faces {
            brep: self.clone(),
            iter: self.json.faces.iter(),
        }
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
    pub fn curves(&self) -> iter::Curves {
        iter::Curves {
            parent: self.clone(),
            iter: self.json.curves.iter(),
        }
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
