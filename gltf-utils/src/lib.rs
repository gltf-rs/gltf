//! This crate provides utility methods in addition to core gltf, such as
//! accessor iterators and easy conversions between different representations of
//! accessor items.

#![allow(unknown_lints)]
#![allow(cast_lossless)]
#![warn(missing_docs)]
#![warn(trivial_casts)]

extern crate byteorder;
extern crate gltf;

use std::fmt;
use std::marker::PhantomData;
use std::mem::size_of;

use byteorder::{LE, ByteOrder};
use byteorder::ReadBytesExt;

use gltf::accessor::{DataType, Dimensions};

use casts::*;

pub mod casts;

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
pub trait PrimitiveIterators<'a>: private::Sealed {
    /// Visits the vertex positions of a primitive.
    fn positions<'s, S: Source>(&'a self, source: &'s S) -> Option<Positions<'s>>;

    /// Visits the vertex normals of a primitive.
    fn normals<'s, S: Source>(&'a self, source: &'s S) -> Option<Normals<'s>>;

    /// Visits the vertex tangents of a primitive.
    fn tangents<'s, S: Source>(&'a self, source: &'s S) -> Option<Tangents<'s>>;

    /// Visits the vertex colors of a primitive.
    fn colors<'s, S>(&'a self, set: u32, source: &'s S) -> Option<Colors<'s>>
        where S: Source;

    /// Visits the vertex draw sequence of a primitive.
    fn indices<'s, S>(&'a self, source: &'s S) -> Option<Indices<'s>>
        where S: Source;

    /// Visits the joint indices of the primitive.
    fn joints<'s, S>(&'a self, set: u32, source: &'s S) -> Option<Joints<'s>>
        where S: Source;

    /// Visits the vertex texture co-ordinates of a primitive.
    fn tex_coords<'s, S>(&'a self, set: u32, source: &'s S) -> Option<TexCoords<'s>>
        where S: Source;

    /// Visits the joint weights of the primitive.
    fn weights<'s, S>(&'a self, set: u32, source: &'s S) -> Option<Weights<'s>>
        where S: Source;
}

impl<'a> PrimitiveIterators<'a> for gltf::Primitive<'a> {
    fn positions<'s, S: Source>(&'a self, source: &'s S) -> Option<Positions<'s>> {
        self.get(&gltf::Semantic::Positions)
            .map(|accessor| AccessorIter::new(accessor, source))
    }

    fn normals<'s, S: Source>(&'a self, source: &'s S) -> Option<Normals<'s>> {
        self.get(&gltf::Semantic::Normals)
            .map(|accessor| AccessorIter::new(accessor, source))
    }

    fn tangents<'s, S: Source>(&'a self, source: &'s S) -> Option<Tangents<'s>> {
        self.get(&gltf::Semantic::Tangents)
            .map(|accessor| AccessorIter::new(accessor, source))
    }

    fn colors<'s, S>(&'a self, set: u32, source: &'s S) -> Option<Colors<'s>>
        where S: Source
    {
        use DataType::{U8, U16, F32};
        use Dimensions::{Vec3, Vec4};

        self.get(&gltf::Semantic::Colors(set))
            .map(|accessor| match (accessor.data_type(), accessor.dimensions()) {
                (U8, Vec3)  => Colors::RgbU8(AccessorIter::new(accessor, source)),
                (U16, Vec3) => Colors::RgbU16(AccessorIter::new(accessor, source)),
                (F32, Vec3) => Colors::RgbF32(AccessorIter::new(accessor, source)),
                (U8, Vec4)  => Colors::RgbaU8(AccessorIter::new(accessor, source)),
                (U16, Vec4) => Colors::RgbaU16(AccessorIter::new(accessor, source)),
                (F32, Vec4) => Colors::RgbaF32(AccessorIter::new(accessor, source)),
                _ => unreachable!(),
            })
    }

    fn indices<'s, S>(&'a self, source: &'s S) -> Option<Indices<'s>>
        where S: Source
    {
        self.indices()
            .map(|accessor| match accessor.data_type() {
                DataType::U8  => Indices::U8(AccessorIter::new(accessor, source)),
                DataType::U16 => Indices::U16(AccessorIter::new(accessor, source)),
                DataType::U32 => Indices::U32(AccessorIter::new(accessor, source)),
                _ => unreachable!(),
            })
    }

    fn joints<'s, S>(&'a self, set: u32, source: &'s S) -> Option<Joints<'s>>
        where S: Source
    {
        self.get(&gltf::Semantic::Joints(set))
            .map(|accessor| match accessor.data_type() {
                DataType::U8  => Joints::U8(AccessorIter::new(accessor, source)),
                DataType::U16 => Joints::U16(AccessorIter::new(accessor, source)),
                _ => unreachable!(),
            })
    }

    fn tex_coords<'s, S>(&'a self, set: u32, source: &'s S) -> Option<TexCoords<'s>>
        where S: Source
    {
        self.get(&gltf::Semantic::TexCoords(set))
            .map(|accessor| match accessor.data_type() {
                DataType::U8  => TexCoords::U8(AccessorIter::new(accessor, source)),
                DataType::U16 => TexCoords::U16(AccessorIter::new(accessor, source)),
                DataType::F32 => TexCoords::F32(AccessorIter::new(accessor, source)),
                _ => unreachable!(),
            })
    }

    fn weights<'s, S>(&'a self, set: u32, source: &'s S) -> Option<Weights<'s>>
        where S: Source
    {
        self.get(&gltf::Semantic::Weights(set))
            .map(|accessor| match accessor.data_type() {
                DataType::U8  => Weights::U8(AccessorIter::new(accessor, source)),
                DataType::U16 => Weights::U16(AccessorIter::new(accessor, source)),
                DataType::F32 => Weights::F32(AccessorIter::new(accessor, source)),
                _ => unreachable!(),
            })
    }
}

/// Extra methods for working with `gltf::Skin`.
pub trait SkinIterators<'a>: private::Sealed {
    /// Visits the `inverseBindMatrices` of the skin.
    fn ibms<'s, S>(&'a self, source: &'s S) -> Option<InverseBindMatrices<'s>>
        where S: Source;
}

impl<'a> SkinIterators<'a> for gltf::Skin<'a> {
    fn ibms<'s, S>(&'a self, source: &'s S) -> Option<InverseBindMatrices<'s>>
        where S: Source
    {
        self.inverse_bind_matrices()
            .map(|accessor| AccessorIter::new(accessor, source))
    }
}

/// Extra methods for working with `gltf::animation::Channel`.
pub trait ChannelIterators<'a>: private::Sealed {
    /// Visits the input samples of a channel.
    fn inputs<'s, S>(&'a self, source: &'s S) -> Inputs<'s>
        where S: Source;

    /// Returns a visitor to either translation, rotation, scaling or morph
    /// target weight samples of a channel.
    fn select<'s, S>(&'a self, source: &'s S) -> Select<'s>
        where S: Source;
}

impl<'a> ChannelIterators<'a> for gltf::animation::Channel<'a> {
    fn inputs<'s, S>(&'a self, source: &'s S) -> Inputs<'s>
        where S: Source
    {
        AccessorIter::new(self.sampler().input(), source)
    }

    fn select<'s, S>(&'a self, source: &'s S) -> Select<'s>
        where S: Source
    {
        use gltf::animation::TrsProperty;

        let output = self.sampler().output();

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
    stride: usize,
    data: &'a [u8],
    _phantom: PhantomData<T>,
}

impl<'a, T> AccessorIter<'a, T> {
    fn new<S>(accessor: gltf::Accessor, source: &'a S) -> AccessorIter<'a, T>
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
        AccessorIter { stride, data, _phantom: PhantomData }
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
pub trait AccessorItem {
    /// Create an object of this type from a byte slice.
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

/// XYZ vertex positions of type `[f32; 3]`.
pub type Positions<'a> = AccessorIter<'a, [f32; 3]>;
/// XYZ vertex normals of type `[f32; 3]`.
pub type Normals<'a> = AccessorIter<'a, [f32; 3]>;
/// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
/// sign value (-1 or +1) indicating the handedness of the tangent basis.
pub type Tangents<'a> = AccessorIter<'a, [f32; 3]>;

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
pub enum Colors<'a> {
    /// RGB vertex color of type `[u8; 3]>`.
    RgbU8(AccessorIter<'a, [u8; 3]>),
    /// RGB vertex color of type `[u16; 3]>`.
    RgbU16(AccessorIter<'a, [u16; 3]>),
    /// RGB vertex color of type `[f32; 3]`.
    RgbF32(AccessorIter<'a, [f32; 3]>),
    /// RGBA vertex color of type `[u8; 4]>`.
    RgbaU8(AccessorIter<'a, [u8; 4]>),
    /// RGBA vertex color of type `[u16; 4]>`.
    RgbaU16(AccessorIter<'a, [u16; 4]>),
    /// RGBA vertex color of type `[f32; 4]`.
    RgbaF32(AccessorIter<'a, [f32; 4]>),
}

/// Index data.
#[derive(Clone, Debug)]
pub enum Indices<'a> {
    /// Index data of type U8
    U8(AccessorIter<'a, u8>),
    /// Index data of type U16
    U16(AccessorIter<'a, u16>),
    /// Index data of type U32
    U32(AccessorIter<'a, u32>),
}

/// Vertex joints.
#[derive(Clone, Debug)]
pub enum Joints<'a> {
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
pub enum TexCoords<'a> {
    /// UV texture co-ordinates of type `[u8; 2]>`.
    U8(AccessorIter<'a, [u8; 2]>),
    /// UV texture co-ordinates of type `[u16; 2]>`.
    U16(AccessorIter<'a, [u16; 2]>),
    /// UV texture co-ordinates of type `[f32; 2]`.
    F32(AccessorIter<'a, [f32; 2]>),
}

/// Weights.
#[derive(Clone, Debug)]
pub enum Weights<'a> {
    /// Weights of type `[u8; 4]`.
    U8(AccessorIter<'a, [u8; 4]>),
    /// Weights of type `[u16; 4]`.
    U16(AccessorIter<'a, [u16; 4]>),
    /// Weights of type `[f32; 4]`.
    F32(AccessorIter<'a, [f32; 4]>),
}

/// Purpose of data in an animation channel,
#[derive(Clone, Debug)]
pub enum Select<'a> {
    /// Channel contains translation data.
    Translations(Translations<'a>),
    /// Channel contains quaternion rotation data.
    Rotations(Rotations<'a>),
    /// Channel contains scaling data.
    Scales(Scales<'a>),
    /// Channel contains morph weights.
    MorphWeights(MorphWeights<'a>),
}

/// Animation XYZ translations of type `[f32; 3]`.
pub type Translations<'a> = AccessorIter<'a, [f32; 3]>;

/// Animation XYZ scales of type `[f32; 3]`.
pub type Scales<'a> = AccessorIter<'a, [f32; 3]>;

/// Rotation animations
#[derive(Clone, Debug)]
pub enum Rotations<'a> {
    /// Rotations of type `[i8; 4]`.
    I8(AccessorIter<'a, [i8; 4]>),
    /// Rotations of type `[u8; 4]`.
    U8(AccessorIter<'a, [u8; 4]>),
    /// Rotations of type `[i16; 4]`.
    I16(AccessorIter<'a, [i16; 4]>),
    /// Rotations of type `[u16; 4]`.
    U16(AccessorIter<'a, [u16; 4]>),
    /// Rotations of type `[f32; 4]`.
    F32(AccessorIter<'a, [f32; 4]>),
}

/// Morph-target weight animations.
#[derive(Clone, Debug)]
pub enum MorphWeights<'a> {
    /// Weights of type `i8`.
    I8(AccessorIter<'a, i8>),
    /// Weights of type `u8`.
    U8(AccessorIter<'a, u8>),
    /// Weights of type `i16`.
    I16(AccessorIter<'a, i16>),
    /// Weights of type `u16`.
    U16(AccessorIter<'a, u16>),
    /// Weights of type `f32`.
    F32(AccessorIter<'a, f32>),
}

impl<'a> Colors<'a> {
    /// Reinterpret colors as RGB u8, discarding alpha, if present.  Lossy if
    /// the underlying iterator yields u16, f32 or any RGBA.
    pub fn into_rgb_u8(self) -> colors::CastingIter<'a, colors::RgbU8> {
        colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGB u16, discarding alpha, if present.  Lossy if
    /// the underlying iterator yields f32 or any RGBA.
    pub fn into_rgb_u16(self) -> colors::CastingIter<'a, colors::RgbU16> {
        colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGB f32, discarding alpha, if present.  Lossy if
    /// the underlying iterator yields u16 or any RGBA.
    pub fn into_rgb_f32(self) -> colors::CastingIter<'a, colors::RgbF32> {
        colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGBA u8, with default alpha 255.  Lossy if the
    /// underlying iterator yields u16 or f32.
    pub fn into_rgba_u8(self) -> colors::CastingIter<'a, colors::RgbaU8> {
        colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGBA u16, with default alpha 65535.  Lossy if the
    /// underlying iterator yields f32.
    pub fn into_rgba_u16(self) -> colors::CastingIter<'a, colors::RgbaU16> {
        colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGBA f32, with default alpha 1.0.  Lossy if the
    /// underlying iterator yields u16.
    pub fn into_rgba_f32(self) -> colors::CastingIter<'a, colors::RgbaF32> {
        colors::CastingIter::new(self)
    }
}

impl<'a> Indices<'a> {
    /// Reinterpret indices as u32, which can fit any possible index.
    pub fn into_u32(self) -> indices::CastingIter<'a, indices::U32> {
        indices::CastingIter::new(self)
    }
}

impl<'a> Joints<'a> {
    /// Reinterpret joints as u16, which can fit any possible joint.
    pub fn into_u16(self) -> joints::CastingIter<'a, joints::U16> {
        joints::CastingIter::new(self)
    }
}

impl<'a> TexCoords<'a> {
    /// Reinterpret texture coordinates as u8.  Lossy if the underlying iterator
    /// yields u16 or f32.
    pub fn into_u8(self) -> tex_coords::CastingIter<'a, tex_coords::U8> {
        tex_coords::CastingIter::new(self)
    }

    /// Reinterpret texture coordinates as u16.  Lossy if the underlying
    /// iterator yields f32.
    pub fn into_u16(self) -> tex_coords::CastingIter<'a, tex_coords::U16> {
        tex_coords::CastingIter::new(self)
    }

    /// Reinterpret texture coordinates as f32.  Lossy if the underlying
    /// iterator yields u16.
    pub fn into_f32(self) -> tex_coords::CastingIter<'a, tex_coords::F32> {
        tex_coords::CastingIter::new(self)
    }
}

impl<'a> Weights<'a> {
    /// Reinterpret weights as u8.  Lossy if the underlying iterator yields u16
    /// or f32.
    pub fn into_u8(self) -> weights::CastingIter<'a, weights::U8> {
        weights::CastingIter::new(self)
    }

    /// Reinterpret weights as u16.  Lossy if the underlying iterator yields
    /// f32.
    pub fn into_u16(self) -> weights::CastingIter<'a, weights::U16> {
        weights::CastingIter::new(self)
    }

    /// Reinterpret weights as f32.  Lossy if the underlying iterator yields
    /// u16.
    pub fn into_f32(self) -> weights::CastingIter<'a, weights::F32> {
        weights::CastingIter::new(self)
    }
}

impl<'a> Rotations<'a> {
    /// Reinterpret rotations as u16.  Lossy if underlying iterator yields u8,
    /// i16, u16 or f32.
    pub fn into_i8(self) -> rotations::CastingIter<'a, rotations::I8> {
        rotations::CastingIter::new(self)
    }

    /// Reinterpret rotations as u16.  Lossy if underlying iterator yields i16,
    /// u16 or f32.
    pub fn into_u8(self) -> rotations::CastingIter<'a, rotations::U8> {
        rotations::CastingIter::new(self)
    }

    /// Reinterpret rotations as u16.  Lossy if underlying iterator yields u16
    /// or f32.
    pub fn into_i16(self) -> rotations::CastingIter<'a, rotations::I16> {
        rotations::CastingIter::new(self)
    }

    /// Reinterpret rotations as u16.  Lossy if underlying iterator yields f32.
    pub fn into_u16(self) -> rotations::CastingIter<'a, rotations::U16> {
        rotations::CastingIter::new(self)
    }

    /// Reinterpret rotations as f32.  Lossy if underlying iterator yields i16
    /// or u16.
    pub fn into_f32(self) -> rotations::CastingIter<'a, rotations::F32> {
        rotations::CastingIter::new(self)
    }
}

impl<'a> MorphWeights<'a> {
    /// Reinterpret morph weights as u16.  Lossy if underlying iterator yields
    /// u8, i16, u16 or f32.
    pub fn into_i8(self) -> morph_weights::CastingIter<'a, morph_weights::I8> {
        morph_weights::CastingIter::new(self)
    }

    /// Reinterpret morph weights as u16.  Lossy if underlying iterator yields
    /// i16, u16 or f32.
    pub fn into_u8(self) -> morph_weights::CastingIter<'a, morph_weights::U8> {
        morph_weights::CastingIter::new(self)
    }

    /// Reinterpret morph weights as u16.  Lossy if underlying iterator yields
    /// u16 or f32.
    pub fn into_i16(self) -> morph_weights::CastingIter<'a, morph_weights::I16> {
        morph_weights::CastingIter::new(self)
    }

    /// Reinterpret morph weights as u16.  Lossy if underlying iterator yields
    /// f32.
    pub fn into_u16(self) -> morph_weights::CastingIter<'a, morph_weights::U16> {
        morph_weights::CastingIter::new(self)
    }

    /// Reinterpret morph weights as f32.  Lossy if underlying iterator yields
    /// i16 or u16.
    pub fn into_f32(self) -> morph_weights::CastingIter<'a, morph_weights::F32> {
        morph_weights::CastingIter::new(self)
    }
}

mod private {
    use gltf;

    pub trait Sealed {}

    impl<'a> Sealed for gltf::Primitive<'a> {}
    impl<'a> Sealed for gltf::animation::Channel<'a> {}
    impl<'a> Sealed for gltf::Skin<'a> {}
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use super::{AccessorItem, AccessorIter};

    #[test]
    fn accessor_empty() {
        let i: AccessorIter<f32> = AccessorIter {
            stride: 4,
            data: &[],
            _phantom: PhantomData,
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
            _phantom: PhantomData,
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
            _phantom: PhantomData,
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
            _phantom: PhantomData,
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
            _phantom: PhantomData,
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
