use crate::{json, Document, Mesh};

/// Iterators.
pub mod iter {
    /// An `Iterator` that visits the faces of a `BRep`.
    #[derive(Clone, Debug)]
    pub struct Faces<'a> {
        /// The parent `BRep` struct.
        pub(crate) brep: super::BRep<'a>,

        /// The internal JSON primitive iterator.
        pub(crate) iter: std::iter::Enumerate<
            std::slice::Iter<'a, json::extensions::kittycad_boundary_representation::brep::Face>,
        >,
    }

    impl<'a> ExactSizeIterator for Faces<'a> {}
    impl<'a> Iterator for Faces<'a> {
        type Item = super::Face<'a>;
        fn next(&mut self) -> Option<Self::Item> {
            self.iter
                .next()
                .map(|(index, json)| super::Face::new(self.brep.clone(), index, json))
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
                .map(|(index, json)| super::Face::new(self.brep.clone(), index, json))
        }
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.iter
                .nth(n)
                .map(|(index, json)| super::Face::new(self.brep.clone(), index, json))
        }
    }
}

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
            iter: self.json.faces.iter().enumerate(),
        }
    }

    /// Returns the mesh approximation of this solid if defined.
    pub fn mesh(&self) -> Option<Mesh<'a>> {
        self.json
            .mesh
            .map(|index| self.document.meshes().nth(index.value()).unwrap())
    }
}

/// Defines a planar surface.
#[derive(Clone, Debug)]
pub struct Plane<'a> {
    /// The corresponding JSON struct.
    #[allow(dead_code)]
    pub(crate) json: &'a json::extensions::kittycad_boundary_representation::surface::Plane,
}

/// Defines a non-uniform rational B-spline (NURBS) surface.
#[derive(Clone, Debug)]
pub struct Nurbs<'a> {
    /// The corresponding JSON struct.
    #[allow(dead_code)]
    pub(crate) json: &'a json::extensions::kittycad_boundary_representation::surface::Nurbs,
}

/// Boundary representation of a solid.
#[derive(Clone, Debug)]
pub struct Face<'a> {
    /// The parent `Document` struct.
    brep: BRep<'a>,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::extensions::kittycad_boundary_representation::brep::Face,
}

impl<'a> Face<'a> {
    /// Constructs a `Face`.
    pub(crate) fn new(
        brep: BRep<'a>,
        index: usize,
        json: &'a json::extensions::kittycad_boundary_representation::brep::Face,
    ) -> Self {
        Self { brep, index, json }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
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

/// Surface manifold.
#[derive(Clone, Debug)]
pub enum Manifold<'a> {
    /// Planar surface.
    Plane(Plane<'a>),
    /// Non-uniform rational B-spline (NURBS) surface.
    Nurbs(Nurbs<'a>),
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
