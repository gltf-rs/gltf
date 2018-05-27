/// Iterators.
pub mod iter;

/// Utility functions.
#[cfg(feature = "utils")]
pub mod util;

use json;
use {Accessor, Gltf, Material};

#[cfg(feature = "utils")]
use accessor;

pub use json::mesh::{Mode, Semantic};
use json::validation::Checked;

/// Vertex attribute data.
pub type Attribute<'a> = (Semantic, Accessor<'a>);

/// Vertex position bounding box.
pub type BoundingBox = Bounds<[f32; 3]>;

/// The minimum and maximum values for a generic accessor.
#[derive(Clone, Debug, PartialEq)]
pub struct Bounds<T> {
    /// Minimum value.
    pub min: T,

    /// Maximum value.
    pub max: T,
}

/// A set of primitives to be rendered.
#[derive(Clone, Debug)]
pub struct Mesh<'a>  {
    /// The parent `Gltf` struct.
    gltf: &'a Gltf,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Mesh,
}

/// A single morph target for a mesh primitive.
#[derive(Clone, Debug)]
pub struct MorphTarget<'a> {
    /// XYZ vertex position displacements.
    positions: Option<Accessor<'a>>,

    /// XYZ vertex normal displacements.
    normals: Option<Accessor<'a>>,

    /// XYZ vertex tangent displacements.
    tangents: Option<Accessor<'a>>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug)]
pub struct Primitive<'a>  {
    /// The parent `Mesh` struct.
    mesh: &'a Mesh<'a>,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Primitive,
}

impl<'a> Mesh<'a>  {
    /// Constructs a `Mesh`.
    pub(crate) fn new(
        gltf: &'a Gltf,
        index: usize,
        json: &'a json::mesh::Mesh,
    ) -> Self {
        Self {
            gltf: gltf,
            index: index,
            json: json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Defines the geometry to be renderered with a material.
    pub fn primitives(&'a self) -> iter::Primitives<'a> {
        iter::Primitives {
            mesh: self,
            iter: self.json.primitives.iter().enumerate(),
        }
    }

    /// Defines the weights to be applied to the morph targets.
    pub fn weights(&self) -> Option<&[f32]> {
        self.json.weights.as_ref().map(Vec::as_slice)
    }
}

impl<'a> Primitive<'a> {
    /// Constructs a `Primitive`.
    pub(crate) fn new(
        mesh: &'a Mesh<'a>,
        index: usize,
        json: &'a json::mesh::Primitive,
    ) -> Self {
        Self {
            mesh: mesh,
            index: index,
            json: json,
        }
    }

    /// Returns the bounds of the `POSITION` vertex attribute.
    pub fn bounding_box(&self) -> BoundingBox {
        // NOTE: cannot panic if validated "minimally"
        let pos_accessor_index = self.json.attributes.get(&Checked::Valid(Semantic::Positions)).unwrap();
        let pos_accessor = self.mesh.gltf.accessors().nth(pos_accessor_index.value()).unwrap();
        let min: [f32; 3] = json::deserialize::from_value(pos_accessor.min().unwrap()).unwrap();
        let max: [f32; 3] = json::deserialize::from_value(pos_accessor.max().unwrap()).unwrap();
        Bounds { min, max }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Return the accessor with the given semantic.
    pub fn get(&self, semantic: &Semantic) -> Option<Accessor> {
        self.json.attributes
            .get(&json::validation::Checked::Valid(semantic.clone()))
            .map(|index| self.mesh.gltf.accessors().nth(index.value()).unwrap())
    }

    /// Returns the accessor containing the primitive indices, if provided.
    pub fn indices(&self) -> Option<Accessor> {
        self.json.indices
            .as_ref()
            .map(|index| self.mesh.gltf.accessors().nth(index.value()).unwrap())
    }

    /// Returns an `Iterator` that visits the vertex attributes.
    pub fn attributes(&self) -> iter::Attributes {
        iter::Attributes {
            gltf: self.mesh.gltf,
            prim: self,
            iter: self.json.attributes.iter(),
        }
    }

    /// Returns the material to apply to this primitive when rendering
    pub fn material(&self) -> Material {
        self.json.material
            .as_ref()
            .map(|index| self.mesh.gltf.materials().nth(index.value()).unwrap())
            .unwrap_or_else(|| Material::default(self.mesh.gltf))
    }

    /// The type of primitives to render.
    pub fn mode(&self) -> Mode {
        self.json.mode.unwrap()
    }

    /// Returns an `Iterator` that visits the morph targets of the primitive.
    pub fn morph_targets(&self) -> iter::MorphTargets {
        if let Some(slice) = self.json.targets.as_ref() {
            iter::MorphTargets {
                gltf: self.mesh.gltf,
                iter: slice.iter(),
            }
        } else {
            iter::MorphTargets {
                gltf: self.mesh.gltf,
                iter: (&[]).iter(),
            }
        }
    }

    /// Visits the vertex positions of a primitive.
    #[cfg(feature = "utils")]
    pub fn iter_positions<'s>(
        &'a self,
        buffer_data: &'s [u8],
    ) -> Option<util::IterPositions<'s>> {
        self.get(&Semantic::Positions)
            .map(|accessor| accessor::Iter::new(accessor, buffer_data))
    }

    /// Visits the vertex normals of a primitive.
    #[cfg(feature = "utils")]
    pub fn iter_normals<'s>(
        &'a self,
        buffer_data: &'s [u8],
    ) -> Option<util::IterNormals<'s>> {
        self.get(&Semantic::Normals)
            .map(|accessor| accessor::Iter::new(accessor, buffer_data))
    }

    /// Visits the vertex tangents of a primitive.
    #[cfg(feature = "utils")]
    pub fn iter_tangents<'s>(
        &'a self,
        buffer_data: &'s [u8],
    ) -> Option<util::IterTangents<'s>> {
        self.get(&Semantic::Tangents)
            .map(|accessor| accessor::Iter::new(accessor, buffer_data))
    }

    /// Visits the vertex colors of a primitive.
    #[cfg(feature = "utils")]
    pub fn iter_colors<'s>(
        &'a self,
        set: u32,
        buffer_data: &'s [u8],
    ) -> Option<util::IterColors<'s>> {
        use accessor::DataType::{U8, U16, F32};
        use accessor::Dimensions::{Vec3, Vec4};
        use self::util::IterColors;

        self.get(&Semantic::Colors(set))
            .map(|accessor| match (accessor.data_type(), accessor.dimensions()) {
                (U8, Vec3)  => IterColors::RgbU8(accessor::Iter::new(accessor, buffer_data)),
                (U16, Vec3) => IterColors::RgbU16(accessor::Iter::new(accessor, buffer_data)),
                (F32, Vec3) => IterColors::RgbF32(accessor::Iter::new(accessor, buffer_data)),
                (U8, Vec4)  => IterColors::RgbaU8(accessor::Iter::new(accessor, buffer_data)),
                (U16, Vec4) => IterColors::RgbaU16(accessor::Iter::new(accessor, buffer_data)),
                (F32, Vec4) => IterColors::RgbaF32(accessor::Iter::new(accessor, buffer_data)),
                _ => unreachable!(),
            })
    }

    /// Visits the vertex draw sequence of a primitive.
    #[cfg(feature = "utils")]
    pub fn iter_indices<'s>(
        &'a self,
        buffer_data: &'s [u8],
    ) -> Option<util::IterIndices<'s>> {
        use accessor::DataType;
        use self::util::IterIndices;
        self.indices()
            .map(|accessor| match accessor.data_type() {
                DataType::U8  => IterIndices::U8(accessor::Iter::new(accessor, buffer_data)),
                DataType::U16 => IterIndices::U16(accessor::Iter::new(accessor, buffer_data)),
                DataType::U32 => IterIndices::U32(accessor::Iter::new(accessor, buffer_data)),
                _ => unreachable!(),
            })
    }

    /// Visits the joint indices of the primitive.
    #[cfg(feature = "utils")]
    pub fn iter_joints<'s>(
        &'a self,
        set: u32,
        buffer_data: &'s [u8],
    ) -> Option<util::IterJoints<'s>> {
        use accessor::DataType;
        use self::util::IterJoints;
        self.get(&Semantic::Joints(set))
            .map(|accessor| match accessor.data_type() {
                DataType::U8  => IterJoints::U8(accessor::Iter::new(accessor, buffer_data)),
                DataType::U16 => IterJoints::U16(accessor::Iter::new(accessor, buffer_data)),
                _ => unreachable!(),
            })
    }

    /// Visits the vertex texture co-ordinates of a primitive.
    #[cfg(feature = "utils")]
    pub fn iter_tex_coords<'s>(
        &'a self,
        set: u32,
        buffer_data: &'s [u8],
    ) -> Option<util::IterTexCoords<'s>> {
        use accessor::DataType;
        use self::util::IterTexCoords;
        self.get(&Semantic::TexCoords(set))
            .map(|accessor| match accessor.data_type() {
                DataType::U8  => IterTexCoords::U8(accessor::Iter::new(accessor, buffer_data)),
                DataType::U16 => IterTexCoords::U16(accessor::Iter::new(accessor, buffer_data)),
                DataType::F32 => IterTexCoords::F32(accessor::Iter::new(accessor, buffer_data)),
                _ => unreachable!(),
            })
    }

    /// Visits the joint weights of the primitive.
    #[cfg(feature = "utils")]
    pub fn iter_weights<'s>(
        &'a self,
        set: u32,
        buffer_data: &'s [u8],
    ) -> Option<util::IterWeights<'s>> {
        use self::accessor::DataType;
        use self::util::IterWeights;
        self.get(&Semantic::Weights(set))
            .map(|accessor| match accessor.data_type() {
                DataType::U8  => IterWeights::U8(accessor::Iter::new(accessor, buffer_data)),
                DataType::U16 => IterWeights::U16(accessor::Iter::new(accessor, buffer_data)),
                DataType::F32 => IterWeights::F32(accessor::Iter::new(accessor, buffer_data)),
                _ => unreachable!(),
            })
    }
}

impl<'a> MorphTarget<'a> {
    /// Returns the XYZ vertex position displacements.
    pub fn positions(&self) -> Option<Accessor<'a>> {
        self.positions.clone()
    }

    /// Returns the XYZ vertex normal displacements.
    pub fn normals(&self) -> Option<Accessor<'a>> {
        self.normals.clone()
    }

    /// Returns the XYZ vertex tangent displacements.
    pub fn tangents(&self) -> Option<Accessor<'a>> {
        self.tangents.clone()
    }
}
