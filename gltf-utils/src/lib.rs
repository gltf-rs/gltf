#![allow(unknown_lints)]
#![allow(cast_lossless)]

extern crate byteorder;
extern crate gltf;

use std::{fmt, marker};
use std::mem::size_of;

use byteorder::{LE, ByteOrder};

use gltf::accessor::{DataType, Dimensions};

/// Helper trait for denormalizing integer types.
///
/// # Examples
///
/// Denormalize a single `u16`.
///
/// ```rust
/// use gltf_utils::Denormalize;
/// let x: u16 = 65535;
/// assert_eq!(1.0, x.denormalize());
/// ```
///
/// Denormalize an array of integers.
///
/// ```rust
/// use gltf_utils::Denormalize;
/// let rgb: [u8; 3] = [0, 120, 255];
/// assert_eq!([0.0, 120.0 / 255.0, 1.0], rgb.denormalize());
/// ```
pub trait Denormalize {
    /// The denormalized version of this type.
    type Denormalized;

    /// Returns the denormalized equivalent of the value.
    fn denormalize(&self) -> Self::Denormalized;
}

/// Represents sources of buffer data.
///
/// See the `Buffers` type in the `gltf-importer` crate for the reference
/// implementation.
pub trait Source: fmt::Debug {
    /// Return the buffer data referenced by the given `Buffer`.
    ///
    /// This method must not fail.
    fn source_buffer(&self, buffer: &gltf::Buffer) -> &[u8];
}

/// Extra methods for working with `gltf::Primitive`.
pub trait PrimitiveIterators<'a> {
    /// Visits the vertex positions of a primitive.
    fn positions<'s, S: Source>(&'a self, source: &'s S) -> Option<Positions<'s>>;

    /// Visits the vertex normals of a primitive.
    fn normals<'s, S: Source>(&'a self, source: &'s S) -> Option<Normals<'s>>;

    /// Visits the vertex tangents of a primitive.
    fn tangents<'s, S: Source>(&'a self, source: &'s S) -> Option<Tangents<'s>>;

    /// Visits the vertex texture co-ordinates of a primitive.
    fn tex_coords_f32<'s, S: Source>(
        &'a self,
        set: u32,
        source: &'s S,
    ) -> Option<TexCoordsF32<'s>>;

    /// Visits the vertex colors of a primitive.
    fn colors_rgba_f32<'s, S: Source>(
        &'a self,
        set: u32,
        default_alpha: f32,
        source: &'s S,
    ) -> Option<ColorsRgbaF32<'s>>;

    /// Visits the vertex draw sequence of a primitive.
    fn indices_u32<'s, S: Source>(&'a self, source: &'s S) -> Option<IndicesU32<'s>>;

    /// Visits the joint indices of the primitive.
    fn joints_u16<'s, S: Source>(
        &'a self,
        set: u32,
        source: &'s S
    ) -> Option<JointsU16<'s>>;

    /// Visits the joint weights of the primitive.
    fn weights_f32<'s, S: Source>(
        &'a self,
        set: u32,
        source: &'s S
    ) -> Option<WeightsF32<'s>>;
}

impl<'a> PrimitiveIterators<'a> for gltf::Primitive<'a> {
    fn positions<'s, S: Source>(&self, source: &'s S) -> Option<Positions<'s>> {
        self.get(&gltf::Semantic::Positions)
            .map(|accessor| Positions(AccessorIter::new(accessor, source)))
    }

    fn normals<'s, S: Source>(&self, source: &'s S) -> Option<Normals<'s>> {
        self.get(&gltf::Semantic::Normals)
            .map(|accessor| Normals(AccessorIter::new(accessor, source)))
    }

    fn tangents<'s, S: Source>(&self, source: &'s S) -> Option<Tangents<'s>> {
        self.get(&gltf::Semantic::Tangents)
            .map(|accessor| Tangents(AccessorIter::new(accessor, source)))
    }

    fn tex_coords_f32<'s, S: Source>(&self, set: u32, source: &'s S) -> Option<TexCoordsF32<'s>> {
        self.get(&gltf::Semantic::TexCoords(set))
            .map(|accessor| TexCoordsF32(TexCoords::new(accessor, source)))
    }

    fn colors_rgba_f32<'s, S: Source>(
        &self,
        set: u32,
        default_alpha: f32,
        source: &'s S,
    ) -> Option<ColorsRgbaF32<'s>> {
        self.get(&gltf::Semantic::Colors(set))
            .map(|accessor| {
                ColorsRgbaF32 {
                    iter: Colors::new(accessor, source),
                    default_alpha,
                }
            })
    }

    fn indices_u32<'s, S: Source>(&self, source: &'s S) -> Option<IndicesU32<'s>> {
        self.indices().map(|accessor| IndicesU32(Indices::new(accessor, source)))
    }

    fn joints_u16<'s, S: Source>(&self, set: u32, source: &'s S) -> Option<JointsU16<'s>> {
        self.get(&gltf::Semantic::Joints(set))
            .map(|accessor| JointsU16(Joints::new(accessor, source)))
    }

    fn weights_f32<'s, S: Source>(&self, set: u32, source: &'s S) -> Option<WeightsF32<'s>> {
        self.get(&gltf::Semantic::Weights(set))
            .map(|accessor| WeightsF32(Weights::new(accessor, source)))
    }
}

/// Extra methods for working with `gltf::Skin`.
pub trait SkinIterators<'a> {
    /// Visits the `inverseBindMatrices` of the skin.
    fn ibms<S: Source>(&'a self, source: &'a S) -> Option<InverseBindMatrices<'a>>;
}

impl<'a> SkinIterators<'a> for gltf::Skin<'a> {
    fn ibms<S: Source>(&'a self, source: &'a S) -> Option<InverseBindMatrices<'a>> {
        self.inverse_bind_matrices().map(|accessor| InverseBindMatrices(AccessorIter::new(accessor, source)))
    }
}

/// Extra methods for working with `gltf::animation::Channel`.
pub trait ChannelIterators<'a> {
    /// Visits the input samples of a channel.
    fn inputs<S: Source>(&'a self, source: &'a S) -> Inputs<'a>;

    /// Visits the output samples of a channel.
    fn outputs<S: Source>(&'a self, source: &'a S) -> Outputs<'a>;
}

impl<'a> ChannelIterators<'a> for gltf::animation::Channel<'a> {
    fn inputs<S: Source>(&'a self, source: &'a S) -> Inputs<'a> {
        Inputs(AccessorIter::new(self.sampler().input(), source))
    }

    fn outputs<S: Source>(&'a self, source: &'a S) -> Outputs<'a> {
        match self.target().path() {
            gltf::animation::TrsProperty::Translation => {
                Outputs::Translations(AccessorIter::new(self.sampler().output(), source))
            },
            gltf::animation::TrsProperty::Scale => {
                Outputs::Scales(AccessorIter::new(self.sampler().output(), source))
            },
            gltf::animation::TrsProperty::Rotation => {
                Outputs::Rotations(RotationsF32(Rotations::new(self.sampler().output(), source)))
            },
            gltf::animation::TrsProperty::Weights => {
                Outputs::Weights(MorphWeightsF32(MorphWeights::new(self.sampler().output(), source)))
            }
        }
    }
}

/// Visits the items in an `Accessor`.
#[derive(Clone, Debug)]
pub struct AccessorIter<'a, T> {
    /// The number of bytes between each item.
    stride: usize,

    /// The data we're iterating over.
    data: &'a [u8],
    
    /// Consumes the data type we're returning at each iteration.
    _phantom: marker::PhantomData<T>,
}

impl<'a, T> AccessorIter<'a, T> {
    pub fn new<S>(accessor: gltf::Accessor, source: &'a S) -> AccessorIter<'a, T>
        where S: Source
    {
        debug_assert_eq!(size_of::<T>(), accessor.size());
        debug_assert!(size_of::<T>() > 0);
        let view = accessor.view();
        let stride = view.stride().unwrap_or(size_of::<T>());
        debug_assert!(stride >= size_of::<T>());
        let start = view.offset() + accessor.offset();
        let end = start + stride * (accessor.count() - 1) + size_of::<T>();
        let data = &source.source_buffer(&view.buffer())[start .. end];
        AccessorIter { stride, data, _phantom: marker::PhantomData }
    }
}

impl<'a, T: AccessorItem> Iterator for AccessorIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let stride = if self.data.len() >= self.stride {
            Some(self.stride)
        } else if self.data.len() >= size_of::<T>() {
            Some(size_of::<T>())
        } else {
            None
        };
        if let Some(stride) = stride {
            let (val, data) = self.data.split_at(stride);
            let val = T::from_slice(val);
            self.data = data;
            Some(val)
        } else {
            None
        }
    }

    fn nth(&mut self, nth: usize) -> Option<Self::Item> {
        if self.data.len() > 0 {
            let val_data = &self.data[nth * self.stride ..];
            let val = T::from_slice(val_data);
            self.data = &val_data[self.stride.min(val_data.len()) ..];
            Some(val)
        } else {
            None
        }
    }

    fn last(self) -> Option<Self::Item> {
        if self.data.len() > 0 {
            self.data
                .get((self.data.len() - 1) / self.stride * self.stride ..)
                .map(T::from_slice)
        } else {
            None
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.data.len() / self.stride
            + (self.data.len() % self.stride > 0) as usize;
        (hint, Some(hint))
    }
}

impl<'a, T: AccessorItem> ExactSizeIterator for AccessorIter<'a, T> {}

/// Any type that can appear in an Accessor.
pub trait AccessorItem: Sized {
    fn from_slice(buf: &[u8]) -> Self;
}

impl AccessorItem for i8 {
    fn from_slice(buf: &[u8]) -> Self {
        buf[0] as i8
    }
}

impl AccessorItem for i16 {
    fn from_slice(buf: &[u8]) -> Self {
        LE::read_i16(buf)
    }
}

impl AccessorItem for u8 {
    fn from_slice(buf: &[u8]) -> Self {
        buf[0]
    }
}

impl AccessorItem for u16 {
    fn from_slice(buf: &[u8]) -> Self {
        LE::read_u16(buf)
    }
}

impl AccessorItem for u32 {
    fn from_slice(buf: &[u8]) -> Self {
        LE::read_u32(buf)
    }
}

impl AccessorItem for f32 {
    fn from_slice(buf: &[u8]) -> Self {
        LE::read_f32(buf)
    }
}

impl<T: AccessorItem> AccessorItem for [T; 2] {
    fn from_slice(buf: &[u8]) -> Self {
        assert!(buf.len() >= 2 * size_of::<T>());
        [T::from_slice(buf),
         T::from_slice(&buf[size_of::<T>() ..])]
    }
}

impl<T: AccessorItem> AccessorItem for [T; 3] {
    fn from_slice(buf: &[u8]) -> Self {
        assert!(buf.len() >= 3 * size_of::<T>());
        [T::from_slice(buf),
         T::from_slice(&buf[1 * size_of::<T>() ..]),
         T::from_slice(&buf[2 * size_of::<T>() ..])]
    }
}

impl<T: AccessorItem> AccessorItem for [T; 4] {
    fn from_slice(buf: &[u8]) -> Self {
        assert!(buf.len() >= 4 * size_of::<T>());
        [T::from_slice(buf),
         T::from_slice(&buf[1 * size_of::<T>() ..]),
         T::from_slice(&buf[2 * size_of::<T>() ..]),
         T::from_slice(&buf[3 * size_of::<T>() ..])]
    }
}

/// XYZ vertex normals of type `[f32; 3]`.
#[derive(Clone, Debug)]
pub struct Normals<'a>(AccessorIter<'a, [f32; 3]>);

/// XYZ vertex positions of type `[f32; 3]`.
#[derive(Clone, Debug)]
pub struct Positions<'a>(AccessorIter<'a, [f32; 3]>);

/// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
/// sign value (-1 or +1) indicating the handedness of the tangent basis.
#[derive(Clone, Debug)]
pub struct Tangents<'a>(AccessorIter<'a, [f32; 4]>);

/// Inverse Bind Matrices of type [[f32; 4]; 4].
#[derive(Clone, Debug)]
pub struct InverseBindMatrices<'a>(AccessorIter<'a, [[f32; 4]; 4]>);

/// Animation input sampler values of type `f32`.
#[derive(Clone, Debug)]
pub struct Inputs<'a>(AccessorIter<'a, f32>);

/// Animation output sampler values.
pub enum Outputs<'a> {
    /// XYZ translations of type `[f32; 3]`.
    Translations(AccessorIter<'a, [f32; 3]>),

    /// Rotation animations.
    Rotations(RotationsF32<'a>),

    /// XYZ scales of type `[f32; 3]`.
    Scales(AccessorIter<'a, [f32; 3]>),

    /// Morph target animations.
    Weights(MorphWeightsF32<'a>),
}

/// Vertex colors.
#[derive(Clone, Debug)]
enum Colors<'a> {
    /// RGB vertex color of type `[u8; 3]>`.
    RgbU8(AccessorIter<'a, [u8; 3]>),

    /// RGBA vertex color of type `[u8; 4]>`.
    RgbaU8(AccessorIter<'a, [u8; 4]>),

    /// RGB vertex color of type `[u16; 3]>`.
    RgbU16(AccessorIter<'a, [u16; 3]>),

    /// RGBA vertex color of type `[u16; 4]>`.
    RgbaU16(AccessorIter<'a, [u16; 4]>),

    /// RGB vertex color of type `[f32; 3]`.
    RgbF32(AccessorIter<'a, [f32; 3]>),

    /// RGBA vertex color of type `[f32; 4]`.
    RgbaF32(AccessorIter<'a, [f32; 4]>),
}

/// Index data.
#[derive(Clone, Debug)]
enum Indices<'a> {
    /// Index data of type U8
    U8(AccessorIter<'a, u8>),
    /// Index data of type U16
    U16(AccessorIter<'a, u16>),
    /// Index data of type U32
    U32(AccessorIter<'a, u32>),
}

/// Vertex joints.
#[derive(Clone, Debug)]
enum Joints<'a> {
    /// Joints of type `[u8; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U8(AccessorIter<'a, [u8; 4]>),

    /// Joints of type `[u16; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U16(AccessorIter<'a, [u16; 4]>),
}

/// UV texture co-ordinates.
#[derive(Clone, Debug)]
enum TexCoords<'a> {
    /// UV texture co-ordinates of type `[f32; 2]`.
    F32(AccessorIter<'a, [f32; 2]>),

    /// UV texture co-ordinates of type `[u8; 2]>`.
    U8(AccessorIter<'a, [u8; 2]>),

    /// UV texture co-ordinates of type `[u16; 2]>`.
    U16(AccessorIter<'a, [u16; 2]>),
}

/// Weights.
#[derive(Clone, Debug)]
enum Weights<'a> {
    /// Weights of type `[f32; 4]`.
    F32(AccessorIter<'a, [f32; 4]>),

    /// Weights of type `[u8; 4]`.
    U8(AccessorIter<'a, [u8; 4]>),

    /// Weights of type `[u16; 4]`.
    U16(AccessorIter<'a, [u16; 4]>),
}

/// Rotation animations .
#[derive(Clone, Debug)]
enum Rotations<'a> {
    /// Rotations of type `[f32; 4]`.
    F32(AccessorIter<'a, [f32; 4]>),
    
    /// Rotations of type `[u8; 4]`.
    U8(AccessorIter<'a, [u8; 4]>),
    
    /// Rotations of type `[i16; 4]`.
    I16(AccessorIter<'a, [i16; 4]>),
    
    /// Rotations of type `[u16; 4]`.
    U16(AccessorIter<'a, [u16; 4]>),
}

/// Morph-target weight animations.
#[derive(Clone, Debug)]
enum MorphWeights<'a> {
    /// Weights of type `f32`.
    F32(AccessorIter<'a, f32>),
    
    /// Weights of type `u8`.
    U8(AccessorIter<'a, u8>),
    
    /// Weights of type `i16`.
    I16(AccessorIter<'a, i16>), 
    
    /// Weights of type `u16`.
    U16(AccessorIter<'a, u16>),
}

/// Index data coerced into `u32` values.
#[derive(Clone, Debug)]
pub struct IndicesU32<'a>(Indices<'a>);

/// Texture co-ordinates coerced into `[f32; 2]` values.
#[derive(Clone, Debug)]
pub struct TexCoordsF32<'a>(TexCoords<'a>);

/// Joint indices co-coerced into `[u16; 4]` values.
#[derive(Clone, Debug)]
pub struct JointsU16<'a>(Joints<'a>);

/// Joint weights co-coerced into `[f32; 4]` values.
#[derive(Clone, Debug)]
pub struct WeightsF32<'a>(Weights<'a>);

/// Vertex colors coerced into `[f32; 4]` (RGBA) values.
#[derive(Clone, Debug)]
pub struct ColorsRgbaF32<'a> {
    /// Internal iterator type.
    iter: Colors<'a>,

    /// Default alpha value.
    default_alpha: f32,
}

/// XYZW quaternion rotations of type `[f32; 4]`.
#[derive(Clone, Debug)]
pub struct RotationsF32<'a>(Rotations<'a>);

/// Morph-target weights of type `f32`.
#[derive(Clone, Debug)]
pub struct MorphWeightsF32<'a>(MorphWeights<'a>);

impl<'a> Colors<'a> {
    fn new<S: Source>(accessor: gltf::Accessor, source: &'a S) -> Colors<'a> {
        match (accessor.dimensions(), accessor.data_type()) {
            (Dimensions::Vec3, DataType::U8) => {
                Colors::RgbU8(AccessorIter::new(accessor, source))
            },
            (Dimensions::Vec4, DataType::U8) => {
                Colors::RgbaU8(AccessorIter::new(accessor, source))
            },
            (Dimensions::Vec3, DataType::U16) => {
                Colors::RgbU16(AccessorIter::new(accessor, source))
            },
            (Dimensions::Vec4, DataType::U16) => {
                Colors::RgbaU16(AccessorIter::new(accessor, source))
            },
            (Dimensions::Vec3, DataType::F32) => {
                Colors::RgbF32(AccessorIter::new(accessor, source))
            },
            (Dimensions::Vec4, DataType::F32) => {
                Colors::RgbaF32(AccessorIter::new(accessor, source))
            },
            _ => unreachable!(),
        }
    }
}

impl<'a> TexCoords<'a> {
    fn new<S: Source>(accessor: gltf::Accessor, source: &'a S) -> TexCoords<'a> {
        match accessor.data_type() {
            DataType::U8 => TexCoords::U8(AccessorIter::new(accessor, source)),
            DataType::U16 => TexCoords::U16(AccessorIter::new(accessor, source)),
            DataType::F32 => TexCoords::F32(AccessorIter::new(accessor, source)),
            _ => unreachable!(),
        }
    }
}

impl<'a> Indices<'a> {
    fn new<S: Source>(accessor: gltf::Accessor, source: &'a S) -> Indices<'a> {
        match accessor.data_type() {
            DataType::U8 => Indices::U8(AccessorIter::new(accessor, source)),
            DataType::U16 => Indices::U16(AccessorIter::new(accessor, source)),
            DataType::U32 => Indices::U32(AccessorIter::new(accessor, source)),
            _ => unreachable!(),
        }
    }
}

impl<'a> Joints<'a> {
    fn new<S: Source>(accessor: gltf::Accessor, source: &'a S) -> Joints<'a> {
        match accessor.data_type() {
            DataType::U8 => Joints::U8(AccessorIter::new(accessor, source)),
            DataType::U16 => Joints::U16(AccessorIter::new(accessor, source)),
            _ => unreachable!(),
        }
    }
}

impl<'a> Weights<'a> {
    fn new<S: Source>(accessor: gltf::Accessor, source: &'a S) -> Weights<'a> {
        match accessor.data_type() {
            DataType::U8 => Weights::U8(AccessorIter::new(accessor, source)),
            DataType::U16 => Weights::U16(AccessorIter::new(accessor, source)),
            DataType::F32 => Weights::F32(AccessorIter::new(accessor, source)),
            _ => unreachable!(),
        }
    }
}

impl<'a> Rotations<'a> {
    fn new<S: Source>(accessor: gltf::Accessor<'a>, source: &'a S) -> Rotations<'a> {
        match accessor.dimensions() {
            Dimensions::Vec4 => {
                match accessor.data_type() {
                    DataType::F32 => {
                        Rotations::F32(AccessorIter::new(accessor, source))
                    },
                    DataType::U8 => {
                        Rotations::U8(AccessorIter::new(accessor, source))
                    },
                    DataType::I16 => {
                        Rotations::I16(AccessorIter::new(accessor, source))
                    },
                    DataType::U16 => {
                        Rotations::U16(AccessorIter::new(accessor, source))
                    },
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        }
    }
}

impl<'a> MorphWeights<'a> {
    fn new<S: Source>(accessor: gltf::Accessor<'a>, source: &'a S) -> MorphWeights<'a> {
        match accessor.dimensions() {
            Dimensions::Scalar => {
                match accessor.data_type() {
                    DataType::F32 => {
                        MorphWeights::F32(AccessorIter::new(accessor, source))
                    },
                    DataType::U8 => {
                        MorphWeights::U8(AccessorIter::new(accessor, source))
                    },
                    DataType::I16 => {
                        MorphWeights::I16(AccessorIter::new(accessor, source))
                    },
                    DataType::U16 => {
                        MorphWeights::U16(AccessorIter::new(accessor, source))
                    },
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        }
    }
}

impl<'a> ExactSizeIterator for IndicesU32<'a> {}
impl<'a> Iterator for IndicesU32<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Indices::U8(ref mut i) => i.next().map(|x| x as u32),
            Indices::U16(ref mut i) => i.next().map(|x| x as u32),
            Indices::U32(ref mut i) => i.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Indices::U8(ref i) => i.size_hint(),
            Indices::U16(ref i) => i.size_hint(),
            Indices::U32(ref i) => i.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for JointsU16<'a> {}
impl<'a> Iterator for JointsU16<'a> {
    type Item = [u16; 4];
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Joints::U8(ref mut i) => {
                i.next()
                    .map(|x| [x[0] as u16, x[1] as u16, x[2] as u16, x[3] as u16])
            },
            Joints::U16(ref mut i) => i.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Joints::U8(ref i) => i.size_hint(),
            Joints::U16(ref i) => i.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for ColorsRgbaF32<'a> {}
impl<'a> Iterator for ColorsRgbaF32<'a> {
    type Item = [f32; 4];
    fn next(&mut self) -> Option<Self::Item> {
        let default_alpha = self.default_alpha;
        match self.iter {
            Colors::RgbU8(ref mut i) => {
                i.next().map(|x| {
                    let rgb = x.denormalize();
                    [rgb[0], rgb[1], rgb[2], default_alpha]
                })
            },
            Colors::RgbU16(ref mut i) => {
                i.next().map(|x| {
                    let rgb = x.denormalize();
                    [rgb[0], rgb[1], rgb[2], default_alpha]
                })
            },
            Colors::RgbF32(ref mut i) => {
                i.next().map(|rgb| [rgb[0], rgb[1], rgb[2], default_alpha])
            },
            Colors::RgbaU8(ref mut i) => i.next().map(|x| x.denormalize()),
            Colors::RgbaU16(ref mut i) => i.next().map(|x| x.denormalize()),
            Colors::RgbaF32(ref mut i) => i.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.iter {
            Colors::RgbU8(ref i) => i.size_hint(),
            Colors::RgbU16(ref i) => i.size_hint(),
            Colors::RgbF32(ref i) => i.size_hint(),
            Colors::RgbaU8(ref i) => i.size_hint(),
            Colors::RgbaU16(ref i) => i.size_hint(),
            Colors::RgbaF32(ref i) => i.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for TexCoordsF32<'a> {}
impl<'a> Iterator for TexCoordsF32<'a> {
    type Item = [f32; 2];
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            TexCoords::U8(ref mut i) => i.next().map(|x| x.denormalize()),
            TexCoords::U16(ref mut i) => i.next().map(|x| x.denormalize()),
            TexCoords::F32(ref mut i) => i.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            TexCoords::U8(ref i) => i.size_hint(),
            TexCoords::U16(ref i) => i.size_hint(),
            TexCoords::F32(ref i) => i.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for WeightsF32<'a> {}
impl<'a> Iterator for WeightsF32<'a> {
    type Item = [f32; 4];
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Weights::U8(ref mut i) => i.next().map(|x| x.denormalize()),
            Weights::U16(ref mut i) => i.next().map(|x| x.denormalize()),
            Weights::F32(ref mut i) => i.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Weights::U8(ref i) => i.size_hint(),
            Weights::U16(ref i) => i.size_hint(),
            Weights::F32(ref i) => i.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for Positions<'a> {}
impl<'a> Iterator for Positions<'a> {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for Normals<'a> {}
impl<'a> Iterator for Normals<'a> {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for Tangents<'a> {}
impl<'a> Iterator for Tangents<'a> {
    type Item = [f32; 4];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for InverseBindMatrices<'a> {}
impl<'a> Iterator for InverseBindMatrices<'a> {
    type Item = [[f32; 4]; 4];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for Inputs<'a> {}
impl<'a> Iterator for Inputs<'a> {
    type Item = f32;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for RotationsF32<'a> {}
impl<'a> Iterator for RotationsF32<'a> {
    type Item = [f32; 4];
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Rotations::F32(ref mut i) => i.next(),
            Rotations::U8(ref mut i) => i.next().map(|x| x.denormalize()),
            Rotations::I16(ref mut i) => i.next().map(|x| x.denormalize()),
            Rotations::U16(ref mut i) => i.next().map(|x| x.denormalize()),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Rotations::F32(ref i) => i.size_hint(),
            Rotations::U8(ref i) => i.size_hint(),
            Rotations::I16(ref i) => i.size_hint(),
            Rotations::U16(ref i) => i.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for MorphWeightsF32<'a> {}
impl<'a> Iterator for MorphWeightsF32<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            MorphWeights::F32(ref mut i) => i.next(),
            MorphWeights::U8(ref mut i) => i.next().map(|x| x.denormalize()),
            MorphWeights::I16(ref mut i) => i.next().map(|x| x as f32 / 32767.0),
            MorphWeights::U16(ref mut i) => i.next().map(|x| x.denormalize()),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            MorphWeights::F32(ref i) => i.size_hint(),
            MorphWeights::U8(ref i) => i.size_hint(),
            MorphWeights::I16(ref i) => i.size_hint(),
            MorphWeights::U16(ref i) => i.size_hint(),
        }
    }
}

impl Denormalize for u8 {
    type Denormalized = f32;
    fn denormalize(&self) -> Self::Denormalized {
        *self as f32 / Self::max_value() as f32
    }
}

impl Denormalize for i16 {
    type Denormalized = f32;
    fn denormalize(&self) -> Self::Denormalized {
        let num = *self as f32 / Self::max_value() as f32;
        if num < -1.0_f32 { -1.0_f32 } else { num }
    }
}

impl Denormalize for u16 {
    type Denormalized = f32;
    fn denormalize(&self) -> Self::Denormalized {
        *self as f32 / Self::max_value() as f32
    }
}

impl<T: Denormalize> Denormalize for [T; 2] {
    type Denormalized = [T::Denormalized; 2];
    fn denormalize(&self) -> Self::Denormalized {
        [
            self[0].denormalize(),
            self[1].denormalize(),
        ]
    }
}

impl<T: Denormalize> Denormalize for [T; 3] {
    type Denormalized = [T::Denormalized; 3];
    fn denormalize(&self) -> Self::Denormalized {
        [
            self[0].denormalize(),
            self[1].denormalize(),
            self[2].denormalize(),
        ]
    }
}

impl<T: Denormalize> Denormalize for [T; 4] {
    type Denormalized = [T::Denormalized; 4];
    fn denormalize(&self) -> Self::Denormalized {
        [
            self[0].denormalize(),
            self[1].denormalize(),
            self[2].denormalize(),
            self[3].denormalize(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::{AccessorItem, AccessorIter};

    #[test]
    fn accessor_empty() {
        let i: AccessorIter<f32> = AccessorIter {
            stride: 4,
            data: &[],
            _phantom: Default::default(),
        };
        assert_eq!(None, i.clone().next());
        assert_eq!(None, i.clone().nth(0));
        assert_eq!(None, i.clone().last());
        assert_eq!(0, i.clone().count());
    }

    #[test]
    fn accessor_single() {
        let data = [0x00, 0x00, 0x80, 0x3f];
        let i: AccessorIter<f32> = AccessorIter {
            stride: 4,
            data: &data,
            _phantom: Default::default(),
        };
        assert_eq!(Some(1.0), i.clone().next());
        assert_eq!(Some(1.0), i.clone().nth(0));
        assert_eq!(Some(1.0), i.clone().last());
        assert_eq!(1, i.clone().count());
    }

    #[test]
    fn accessor_single_stride() {
        let data = [0x00, 0x00, 0x80, 0x3f, 0xff];
        let i: AccessorIter<f32> = AccessorIter {
            stride: 7,
            data: &data,
            _phantom: Default::default(),
        };
        assert_eq!(Some(1.0), i.clone().next());
        assert_eq!(Some(1.0), i.clone().nth(0));
        assert_eq!(Some(1.0), i.clone().last());
        assert_eq!(1, i.clone().count());
    }

    #[test]
    fn accessor_multi() {
        let data = [0x00, 0x00, 0x80, 0x3f,
                   0xd0, 0x0f, 0x49, 0x40,
                   0x00, 0x00, 0x28, 0x42];
        let i: AccessorIter<f32> = AccessorIter {
            stride: 4,
            data: &data,
            _phantom: Default::default(),
        };
        assert_eq!(Some(1.0),      i.clone().nth(0));
        assert_eq!(Some(3.141590), i.clone().nth(1));
        assert_eq!(Some(42.0),     i.clone().nth(2));
        assert_eq!(Some(42.0),     i.clone().last());
        assert_eq!(3, i.clone().count());
    }

    #[test]
    fn accessor_multi_stride() {
        let data = [0x00, 0x00, 0x80, 0x3f, 0xde, 0xad, 0xbe, 0xef,
                   0xd0, 0x0f, 0x49, 0x40, 0xde, 0xad, 0xbe, 0xef,
                   0x00, 0x00, 0x28, 0x42];
        let i: AccessorIter<f32> = AccessorIter {
            stride: 8,
            data: &data,
            _phantom: Default::default(),
        };
        assert_eq!(Some(1.0),      i.clone().nth(0));
        assert_eq!(Some(3.141590), i.clone().nth(1));
        assert_eq!(Some(42.0),     i.clone().nth(2));
        assert_eq!(Some(42.0),     i.clone().last());
        assert_eq!(3, i.clone().count());
    }

    #[test]
    fn accessor_types() {
        let data = [0x26, 0x84, 0xa1, 0x99];
        let evil = -1.670038415647693561554125748263503574431165787927966448478400707244873046875e-23;
        assert_eq!(0x26,       <i8  as AccessorItem>::from_slice(&data));
        assert_eq!(-31706,     <i16 as AccessorItem>::from_slice(&data));
        assert_eq!(0x26,       <u8  as AccessorItem>::from_slice(&data));
        assert_eq!(0x8426,     <u16 as AccessorItem>::from_slice(&data));
        assert_eq!(0x99a18426, <u32 as AccessorItem>::from_slice(&data));
        assert_eq!(evil,       <f32 as AccessorItem>::from_slice(&data));
    }
}
