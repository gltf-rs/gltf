//! # Basic usage
//!
//! Visiting the accessors of a glTF asset.
//!
//! ```
//! # fn run() -> Result<(), Box<std::error::Error>> {
//! # let gltf = gltf::Gltf::open("examples/Box.gltf")?;
//! for accessor in gltf.accessors() {
//!     println!("Accessor #{}", accessor.index());
//!     println!("offset: {}", accessor.offset());
//!     println!("count: {}", accessor.count());
//!     println!("data_type: {:?}", accessor.data_type());
//!     println!("dimensions: {:?}", accessor.dimensions());
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
//! # fn run() -> Result<(), Box<std::error::Error>> {
//! # use gltf::accessor::{DataType, Dimensions, Iter};
//! let (gltf, buffers, _) = gltf::import("examples/Box.gltf")?;
//! for accessor in gltf.accessors() {
//!     match (accessor.data_type(), accessor.dimensions()) {
//!         (DataType::F32, Dimensions::Vec3) => {
//!             let buffer_index = accessor.view().buffer().index();
//!             let buffer_data = buffers[buffer_index].0.as_slice();
//!             let iter = Iter::<[f32; 3]>::new(accessor, buffer_data);
//!             for item in iter {
//!                 println!("{:?}", item);
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

use {buffer, json};

use Document;

pub use json::accessor::ComponentType as DataType;
pub use json::accessor::Type as Dimensions;

/// Utility functions.
#[cfg(feature = "utils")]
pub mod util;

/// Contains data structures for sparse storage.
pub mod sparse;

#[cfg(feature = "utils")]
#[doc(inline)]
pub use self::util::{Item, Iter};

/// A typed view into a buffer view.
#[derive(Clone, Debug)]
pub struct Accessor<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::accessor::Accessor,

    /// The buffer view this accessor reads from.
    view: buffer::View<'a>,
}

impl<'a> Accessor<'a> {
    /// Constructs an `Accessor`.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::accessor::Accessor,
    ) -> Self {
        let view = document.views().nth(json.buffer_view.value()).unwrap();
        Self {
            document,
            index,
            json,
            view,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the size of each component that this accessor describes.
    pub fn size(&self) -> usize {
        self.data_type().size() * self.dimensions().multiplicity()
    }

    /// Returns the buffer view this accessor reads from.
    pub fn view(&self) -> buffer::View<'a> {
        self.document.views().nth(self.json.buffer_view.value()).unwrap()
    }

    /// Returns the offset relative to the start of the parent buffer view in bytes.
    pub fn offset(&self) -> usize {
        self.json.byte_offset as usize
    }

    /// Returns the number of components within the buffer view - not to be confused
    /// with the number of bytes in the buffer view.
    pub fn count(&self) -> usize {
        self.json.count as usize
    }

    /// Returns the data type of components in the attribute.
    pub fn data_type(&self) -> DataType {
        self.json.component_type.unwrap().0
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Specifies if the attribute is a scalar, vector, or matrix.
    pub fn dimensions(&self) -> Dimensions {
        self.json.type_.unwrap()
    }

    /// Returns the minimum value of each component in this attribute.
    pub fn min(&self) -> Option<json::Value> {
        self.json.min.clone()
    }

    /// Returns the maximum value of each component in this attribute.
    pub fn max(&self) -> Option<json::Value> {
        self.json.max.clone()
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Specifies whether integer data values should be normalized.
    pub fn normalized(&self) -> bool {
        self.json.normalized
    }

    /// Returns sparse storage of attributes that deviate from their initialization
    /// value.
    pub fn sparse(&self) -> Option<sparse::Sparse> {
        self.json.sparse.as_ref().map(|json| {
            sparse::Sparse::new(self.document, json)
        })
    }
}
