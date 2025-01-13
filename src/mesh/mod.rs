//! # Basic usage
//!
//! Listing the attributes of each mesh primitive in a glTF asset.
//!
//! ```
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let gltf = gltf::Gltf::open("examples/Box.gltf")?;
//! for mesh in gltf.meshes() {
//!    println!("Mesh #{}", mesh.index());
//!    for primitive in mesh.primitives() {
//!        println!("- Primitive #{}", primitive.index());
//!        for (semantic, _) in primitive.attributes() {
//!            println!("-- {:?}", semantic);
//!        }
//!    }
//! }
//! # Ok(())
//! # }
//! # fn main() {
//! #    let _ = run().expect("runtime error");
//! # }
//! ```
//!
//! # Reader utility
//!
//! Printing the vertex positions of each primitive of each mesh in
//! a glTF asset.
//!
//! ```
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let (gltf, buffers, _) = gltf::import("examples/Box.gltf")?;
//! for mesh in gltf.meshes() {
//!    println!("Mesh #{}", mesh.index());
//!    for primitive in mesh.primitives() {
//!        println!("- Primitive #{}", primitive.index());
//!        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
//!        if let Some(iter) = reader.read_positions() {
//!            for vertex_position in iter {
//!                println!("{:?}", vertex_position);
//!            }
//!        }
//!    }
//! }
//! # Ok(())
//! # }
//! # fn main() {
//! #    let _ = run().expect("runtime error");
//! # }
//! ```

/// Iterators.
pub mod iter;

/// Utility functions.
#[cfg(feature = "utils")]
#[cfg_attr(docsrs, doc(cfg(feature = "utils")))]
pub mod util;

use crate::{Accessor, Buffer, Document, Material};

#[cfg(feature = "utils")]
use crate::accessor;

pub use json::mesh::{Mode, Semantic};
use json::validation::Checked;
#[cfg(feature = "extensions")]
use serde_json::{Map, Value};

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
pub struct Mesh<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

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
pub struct Primitive<'a> {
    /// The parent `Mesh` struct.
    mesh: Mesh<'a>,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Primitive,
}

/// Mesh primitive reader.
#[derive(Clone, Debug)]
pub struct Reader<'a, 's, F>
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    #[allow(dead_code)]
    pub(crate) primitive: &'a Primitive<'a>,
    #[allow(dead_code)]
    pub(crate) get_buffer_data: F,
}

impl<'a> Mesh<'a> {
    /// Constructs a `Mesh`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a json::mesh::Mesh) -> Self {
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

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(docsrs, doc(cfg(feature = "names")))]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Defines the geometry to be renderered with a material.
    pub fn primitives(&self) -> iter::Primitives<'a> {
        iter::Primitives {
            mesh: self.clone(),
            iter: self.json.primitives.iter().enumerate(),
        }
    }

    /// Defines the weights to be applied to the morph targets.
    pub fn weights(&self) -> Option<&'a [f32]> {
        self.json.weights.as_deref()
    }
}

impl<'a> Primitive<'a> {
    /// Constructs a `Primitive`.
    pub(crate) fn new(mesh: Mesh<'a>, index: usize, json: &'a json::mesh::Primitive) -> Self {
        Self { mesh, index, json }
    }

    /// Returns the bounds of the `POSITION` vertex attribute.
    pub fn bounding_box(&self) -> BoundingBox {
        // NOTE: cannot panic if validated "minimally"
        let pos_accessor_index = self
            .json
            .attributes
            .get(&Checked::Valid(Semantic::Positions))
            .unwrap();
        let pos_accessor = self
            .mesh
            .document
            .accessors()
            .nth(pos_accessor_index.value())
            .unwrap();
        let min: [f32; 3] = json::deserialize::from_value(pos_accessor.min().unwrap()).unwrap();
        let max: [f32; 3] = json::deserialize::from_value(pos_accessor.max().unwrap()).unwrap();
        Bounds { min, max }
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

    /// Return the accessor with the given semantic.
    pub fn get(&self, semantic: &Semantic) -> Option<Accessor<'a>> {
        self.json
            .attributes
            .get(&json::validation::Checked::Valid(semantic.clone()))
            .map(|index| self.mesh.document.accessors().nth(index.value()).unwrap())
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the accessor containing the primitive indices, if provided.
    pub fn indices(&self) -> Option<Accessor<'a>> {
        self.json
            .indices
            .as_ref()
            .map(|index| self.mesh.document.accessors().nth(index.value()).unwrap())
    }

    /// Returns an `Iterator` that visits the vertex attributes.
    pub fn attributes(&self) -> iter::Attributes<'a> {
        iter::Attributes {
            document: self.mesh.document,
            prim: self.clone(),
            iter: self.json.attributes.iter(),
        }
    }

    /// Returns the material to apply to this primitive when rendering
    pub fn material(&self) -> Material<'a> {
        self.json
            .material
            .as_ref()
            .map(|index| self.mesh.document.materials().nth(index.value()).unwrap())
            .unwrap_or_else(|| Material::default(self.mesh.document))
    }

    /// The type of primitives to render.
    pub fn mode(&self) -> Mode {
        self.json.mode.unwrap()
    }

    /// Returns an `Iterator` that visits the morph targets of the primitive.
    pub fn morph_targets(&self) -> iter::MorphTargets<'a> {
        if let Some(slice) = self.json.targets.as_ref() {
            iter::MorphTargets {
                document: self.mesh.document,
                iter: slice.iter(),
            }
        } else {
            iter::MorphTargets {
                document: self.mesh.document,
                iter: ([]).iter(),
            }
        }
    }

    /// Get the material variants.
    #[cfg(feature = "KHR_materials_variants")]
    #[cfg_attr(docsrs, doc(cfg(feature = "KHR_materials_variants")))]
    pub fn mappings(&self) -> iter::Mappings<'a> {
        let iter = self
            .json
            .extensions
            .as_ref()
            .and_then(|extensions| extensions.khr_materials_variants.as_ref())
            .map(|variants| variants.mappings.iter())
            .unwrap_or_else(|| ([]).iter());

        iter::Mappings {
            document: self.mesh.document,
            iter,
        }
    }

    /// Constructs the primitive reader.
    #[cfg(feature = "utils")]
    #[cfg_attr(docsrs, doc(cfg(feature = "utils")))]
    pub fn reader<'s, F>(&'a self, get_buffer_data: F) -> Reader<'a, 's, F>
    where
        F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
    {
        Reader {
            primitive: self,
            get_buffer_data,
        }
    }
}

#[cfg(feature = "utils")]
impl<'a, 's, F> Reader<'a, 's, F>
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    /// Visits the vertex positions of a primitive.
    pub fn read_positions(&self) -> Option<util::ReadPositions<'s>> {
        self.primitive
            .get(&Semantic::Positions)
            .and_then(|accessor| accessor::Iter::new(accessor, self.get_buffer_data.clone()))
    }

    /// Visits the vertex normals of a primitive.
    pub fn read_normals(&self) -> Option<util::ReadNormals<'s>> {
        self.primitive
            .get(&Semantic::Normals)
            .and_then(|accessor| accessor::Iter::new(accessor, self.get_buffer_data.clone()))
    }

    /// Visits the vertex tangents of a primitive.
    pub fn read_tangents(&self) -> Option<util::ReadTangents<'s>> {
        self.primitive
            .get(&Semantic::Tangents)
            .and_then(|accessor| accessor::Iter::new(accessor, self.get_buffer_data.clone()))
    }

    /// Visits the vertex colors of a primitive.
    pub fn read_colors(&self, set: u32) -> Option<util::ReadColors<'s>> {
        use self::util::ReadColors;
        use accessor::DataType::{F32, U16, U8};
        use accessor::Dimensions::{Vec3, Vec4};
        self.primitive
            .get(&Semantic::Colors(set))
            .and_then(
                |accessor| match (accessor.data_type(), accessor.dimensions()) {
                    (U8, Vec3) => accessor::Iter::new(accessor, self.get_buffer_data.clone())
                        .map(ReadColors::RgbU8),
                    (U16, Vec3) => accessor::Iter::new(accessor, self.get_buffer_data.clone())
                        .map(ReadColors::RgbU16),
                    (F32, Vec3) => accessor::Iter::new(accessor, self.get_buffer_data.clone())
                        .map(ReadColors::RgbF32),
                    (U8, Vec4) => accessor::Iter::new(accessor, self.get_buffer_data.clone())
                        .map(ReadColors::RgbaU8),
                    (U16, Vec4) => accessor::Iter::new(accessor, self.get_buffer_data.clone())
                        .map(ReadColors::RgbaU16),
                    (F32, Vec4) => accessor::Iter::new(accessor, self.get_buffer_data.clone())
                        .map(ReadColors::RgbaF32),
                    _ => unreachable!(),
                },
            )
    }

    /// Visits the vertex draw sequence of a primitive.
    pub fn read_indices(&self) -> Option<util::ReadIndices<'s>> {
        use self::util::ReadIndices;
        use accessor::DataType;
        self.primitive
            .indices()
            .and_then(|accessor| match accessor.data_type() {
                DataType::U8 => {
                    accessor::Iter::new(accessor, self.get_buffer_data.clone()).map(ReadIndices::U8)
                }
                DataType::U16 => accessor::Iter::new(accessor, self.get_buffer_data.clone())
                    .map(ReadIndices::U16),
                DataType::U32 => accessor::Iter::new(accessor, self.get_buffer_data.clone())
                    .map(ReadIndices::U32),
                _ => unreachable!(),
            })
    }

    /// Visits the joint indices of the primitive.
    pub fn read_joints(&self, set: u32) -> Option<util::ReadJoints<'s>> {
        use self::util::ReadJoints;
        use accessor::DataType;
        self.primitive
            .get(&Semantic::Joints(set))
            .and_then(|accessor| match accessor.data_type() {
                DataType::U8 => {
                    accessor::Iter::new(accessor, self.get_buffer_data.clone()).map(ReadJoints::U8)
                }
                DataType::U16 => {
                    accessor::Iter::new(accessor, self.get_buffer_data.clone()).map(ReadJoints::U16)
                }
                _ => unreachable!(),
            })
    }

    /// Visits the vertex texture co-ordinates of a primitive.
    pub fn read_tex_coords(&self, set: u32) -> Option<util::ReadTexCoords<'s>> {
        use self::util::ReadTexCoords;
        use accessor::DataType;
        self.primitive
            .get(&Semantic::TexCoords(set))
            .and_then(|accessor| match accessor.data_type() {
                DataType::U8 => accessor::Iter::new(accessor, self.get_buffer_data.clone())
                    .map(ReadTexCoords::U8),
                DataType::U16 => accessor::Iter::new(accessor, self.get_buffer_data.clone())
                    .map(ReadTexCoords::U16),
                DataType::F32 => accessor::Iter::new(accessor, self.get_buffer_data.clone())
                    .map(ReadTexCoords::F32),
                _ => unreachable!(),
            })
    }

    /// Visits the joint weights of the primitive.
    pub fn read_weights(&self, set: u32) -> Option<util::ReadWeights<'s>> {
        use self::accessor::DataType;
        use self::util::ReadWeights;
        self.primitive
            .get(&Semantic::Weights(set))
            .and_then(|accessor| match accessor.data_type() {
                DataType::U8 => {
                    accessor::Iter::new(accessor, self.get_buffer_data.clone()).map(ReadWeights::U8)
                }
                DataType::U16 => accessor::Iter::new(accessor, self.get_buffer_data.clone())
                    .map(ReadWeights::U16),
                DataType::F32 => accessor::Iter::new(accessor, self.get_buffer_data.clone())
                    .map(ReadWeights::F32),
                _ => unreachable!(),
            })
    }

    /// Visits the morph targets of the primitive.
    pub fn read_morph_targets(&self) -> util::ReadMorphTargets<'a, 's, F> {
        util::ReadMorphTargets {
            index: 0,
            reader: self.clone(),
        }
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
