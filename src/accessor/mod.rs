//! # Basic usage
//!
//! Visiting the accessors of a glTF asset.
//!
//! ```
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let gltf = gltf::Gltf::open("examples/Box.gltf")?;
//! for (index, accessor) in gltf.root.accessors.iter().enumerate() {
//!     println!("Accessor #{index}");
//!     println!("offset: {:?}", accessor.byte_offset);
//!     println!("count: {:?}", accessor.count);
//!     println!("attribute_type: {:?}", accessor.attribute_type);
//!     println!("component_type: {:?}", accessor.component_type);
//! }
//! # Ok(())
//! # }
//! # fn main() {
//! #    let _ = run().expect("runtime error");
//! # }
//! ```
//!
//! # Utility functions
//!
//! Reading the values from the `vec3` accessors of a glTF asset.
//!
//! ## Note
//!
//! The [`Iter`] utility is a low-level iterator intended for use in special
//! cases. The average user is expected to use reader abstractions such as
//! [`mesh::Reader`].
//!
//! [`Iter`]: struct.Iter.html
//! [`mesh::Reader`]: ../mesh/struct.Reader.html
//!
//! ```
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # use gltf::accessor::{AttributeType, ComponentType, Iter};
//! let (gltf, buffers, _) = gltf::import("examples/Box.gltf")?;
//! let get_buffer_data = |buffer: gltf::Index<gltf::Buffer>| buffers.get(buffer.value()).map(|b| b.0.as_slice());
//! for (index, accessor) in gltf.accessors.iter().enumerate() {
//!     match (accessor.component_type, accessor.attribute_type) {
//!         (ComponentType::F32, AttributeType::Vec3) => {
//!             if let Some(iter) = Iter::<[f32; 3]>::new(&gltf, gltf::Index::new(index as u32), get_buffer_data) {
//!                 for item in iter {
//!                     println!("{item:?}");
//!                 }
//!             }
//!         }
//!         _ => {},
//!     }
//! }
//! # Ok(())
//! # }
//! # fn main() {
//! #    let _ = run().expect("runtime error");
//! # }
//! ```

/// Utility functions.
#[cfg(feature = "utils")]
#[cfg_attr(docsrs, doc(cfg(feature = "utils")))]
pub mod util;

#[cfg(feature = "utils")]
#[doc(inline)]
pub use self::util::{Item, Iter};
use crate::validation::{Error, USize64, Validate};
use crate::{buffer, Extras, Index, Path, Root, Stub, UnrecognizedExtensions};
use serde_json::Value;

/// The component data type.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde_repr::Deserialize_repr, serde_repr::Serialize_repr,
)]
#[repr(u32)]
pub enum ComponentType {
    /// Corresponds to `GL_BYTE`.
    I8 = 5120,
    /// Corresponds to `GL_UNSIGNED_BYTE`.
    U8 = 5121,
    /// Corresponds to `GL_SHORT`.
    I16 = 5122,
    /// Corresponds to `GL_UNSIGNED_SHORT`.
    U16 = 5123,
    /// Corresponds to `GL_UNSIGNED_INT`.
    U32 = 5125,
    /// Corresponds to `GL_FLOAT`.
    F32 = 5126,
}

impl Validate for ComponentType {}

impl From<sparse::IndexType> for ComponentType {
    fn from(value: sparse::IndexType) -> Self {
        match value {
            sparse::IndexType::U8 => ComponentType::U8,
            sparse::IndexType::U16 => ComponentType::U16,
            sparse::IndexType::U32 => ComponentType::U32,
        }
    }
}

impl Stub for ComponentType {
    fn stub() -> Self {
        Self::I8
    }
}

/// Specifies whether an attribute, vector, or matrix.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde_derive::Deserialize, serde_derive::Serialize)]
pub enum AttributeType {
    /// Scalar quantity.
    #[serde(rename = "SCALAR")]
    Scalar = 1,
    /// 2D vector.
    #[serde(rename = "VEC2")]
    Vec2,
    /// 3D vector.
    #[serde(rename = "VEC3")]
    Vec3,
    /// 4D vector.
    #[serde(rename = "VEC4")]
    Vec4,
    /// 2x2 matrix.
    #[serde(rename = "MAT2")]
    Mat2,
    /// 3x3 matrix.
    #[serde(rename = "MAT3")]
    Mat3,
    /// 4x4 matrix.
    #[serde(rename = "MAT4")]
    Mat4,
}

impl Validate for AttributeType {}

impl Stub for AttributeType {
    fn stub() -> Self {
        Self::Scalar
    }
}

/// Contains data structures for sparse storage.
pub mod sparse {
    use crate::validation::{USize64, Validate};
    use crate::{buffer, Extras, Index, Stub, UnrecognizedExtensions};

    /// Data type specific to sparse indices.
    #[derive(
        Clone, Copy, Debug, serde_repr::Deserialize_repr, Eq, PartialEq, serde_repr::Serialize_repr,
    )]
    #[repr(u32)]
    pub enum IndexType {
        /// Corresponds to `GL_UNSIGNED_BYTE`.
        U8 = super::ComponentType::U8 as u32,
        /// Corresponds to `GL_UNSIGNED_SHORT`.
        U16 = super::ComponentType::U16 as u32,
        /// Corresponds to `GL_UNSIGNED_INT`.
        U32 = super::ComponentType::U32 as u32,
    }

    impl Validate for IndexType {}

    impl Stub for IndexType {
        fn stub() -> Self {
            Self::U8
        }
    }

    impl IndexType {
        /// Returns the number of bytes this value represents.
        pub fn size(self) -> usize {
            super::ComponentType::from(self).size()
        }

        /// Returns the corresponding `GLenum`.
        pub fn as_gl_enum(self) -> u32 {
            super::ComponentType::from(self).as_gl_enum()
        }
    }

    /// Indices of those attributes that deviate from their initialization value.
    #[derive(
        Clone,
        Debug,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Stub,
        gltf_derive::Validate,
    )]
    pub struct Indices {
        /// The parent buffer view containing the sparse indices.
        ///
        /// The referenced buffer view must not have `ARRAY_BUFFER` nor
        /// `ELEMENT_ARRAY_BUFFER` as its target.
        pub buffer_view: Index<buffer::View>,

        /// The offset relative to the start of the parent `BufferView` in bytes.
        #[serde(default)]
        pub byte_offset: USize64,

        /// The data type of each index.
        #[serde(rename = "componentType")]
        pub index_type: IndexType,

        /// Unrecognized extension data.
        pub unrecognized_extensions: UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<Extras>,
    }

    /// Sparse storage of attributes that deviate from their initialization value.
    #[derive(
        Clone,
        Debug,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Stub,
        gltf_derive::Validate,
    )]
    pub struct Sparse {
        /// The number of attributes encoded in this sparse accessor.
        pub count: USize64,

        /// Index array of size `count` that points to those accessor attributes
        /// that deviate from their initialization value.
        ///
        /// Indices must strictly increase.
        pub indices: Indices,

        /// Array of size `count * number_of_components` storing the displaced
        /// accessor attributes pointed by `indices`.
        ///
        /// Substituted values must have the same `component_type` and number of
        /// components as the base `Accessor`.
        pub values: Values,

        /// Unrecognized extension data.
        pub unrecognized_extensions: UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<Extras>,
    }

    /// Array of size `count * number_of_components` storing the displaced
    /// accessor attributes pointed by `accessor::sparse::Indices`.
    #[derive(
        Clone,
        Debug,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Stub,
        gltf_derive::Validate,
    )]
    pub struct Values {
        /// The parent buffer view containing the sparse indices.
        ///
        /// The referenced buffer view must not have `ARRAY_BUFFER` nor
        /// `ELEMENT_ARRAY_BUFFER` as its target.
        pub buffer_view: Index<buffer::View>,

        /// The offset relative to the start of the parent buffer view in bytes.
        #[serde(default)]
        pub byte_offset: USize64,

        /// Unrecognized extension data.
        pub unrecognized_extensions: UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<Extras>,
    }
}

/// A typed view into a buffer view.
#[derive(
    Clone,
    Debug,
    gltf_derive::Deserialize,
    gltf_derive::Serialize,
    gltf_derive::Stub,
    gltf_derive::Validate,
)]
#[gltf(validate = "validate_accessor")]
pub struct Accessor {
    /// Specifies if the attribute is a scalar, vector, or matrix.
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,

    /// The parent buffer view this accessor reads from.
    ///
    /// This field can be omitted in sparse accessors.
    pub buffer_view: Option<Index<buffer::View>>,

    /// The offset relative to the start of the parent `BufferView` in bytes.
    ///
    /// This field can be omitted in sparse accessors.
    pub byte_offset: Option<USize64>,

    /// The number of components within the buffer view - not to be confused
    /// with the number of bytes in the buffer view.
    pub count: USize64,

    /// The data type of components in the attribute.
    pub component_type: ComponentType,

    /// Minimum value of each component in this attribute.
    pub min: Option<Value>,

    /// Maximum value of each component in this attribute.
    pub max: Option<Value>,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// Specifies whether integer data values should be normalized.
    pub normalized: bool,

    /// Sparse storage of attributes that deviate from their initialization
    /// value.
    pub sparse: Option<sparse::Sparse>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

fn validate_accessor<P, R>(accessor: &Accessor, _root: &Root, path: P, report: &mut R)
where
    P: Fn() -> Path,
    R: FnMut(&dyn Fn() -> Path, Error),
{
    if accessor.sparse.is_none() && accessor.buffer_view.is_none() {
        // If sparse is missing, then bufferView must be present. Report that bufferView is
        // missing since it is the more common one to require.
        report(&|| path().field("bufferView"), Error::Missing);
    }
}

impl Accessor {
    /// Provides the size of each component that this accessor describes.
    pub fn size(&self) -> usize {
        self.component_type.size() * self.attribute_type.multiplicity()
    }
}

impl ComponentType {
    /// Returns the number of bytes this value represents.
    pub fn size(self) -> usize {
        match self {
            Self::I8 | Self::U8 => 1,
            Self::I16 | Self::U16 => 2,
            Self::F32 | Self::U32 => 4,
        }
    }

    /// Returns the corresponding `GLenum`.
    pub fn as_gl_enum(self) -> u32 {
        self as u32
    }
}

impl AttributeType {
    /// Returns the equivalent number of scalar quantities this type represents.
    pub fn multiplicity(&self) -> usize {
        match *self {
            Self::Scalar => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec4 | Self::Mat2 => 4,
            Self::Mat3 => 9,
            Self::Mat4 => 16,
        }
    }
}
