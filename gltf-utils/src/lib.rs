
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf;

use std::marker;

use std::ops::Range;

/// Represents sources of buffer data.
pub trait Source: std::fmt::Debug {
    /// Return the buffer data referenced by the given `Buffer`.
    ///
    /// This method must not fail.
    fn source_buffer(&self, buffer: &gltf::Buffer) -> &[u8];
}

/// The contents of a .glb file.
#[derive(Clone, Debug)]
struct Glb<'a>(&'a [u8]);

/// The header section of a .glb file.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Header {
    /// Must be `b"glTF"`.
    magic: [u8; 4],

    /// Must be `2`.
    version: u32,

    /// Must match the length of the parent .glb file.
    length: u32,
}

/// The header of the JSON or BIN section of a .glb file.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct ChunkInfo {
    /// The length of the chunk data in byte excluding the header.
    length: u32,

    /// Either `b"JSON"` or `b"BIN\0"`.
    kind: [u8; 4],

    // Data follows... 
}


/// Represents a subset of `Importer::glb`.
#[derive(Copy, Clone, Debug, Default)]
struct Slice {
    /// Offset into `glb` in bytes.
    offset: usize,

    /// Length of the slice in bytes.
    length: usize,
}

/// The header, JSON, and BIN sections of a .glb file, respectively.
type Split = (Header, Range<usize>, Option<Range<usize>>);

impl<'a> Glb<'a> {
    /// Splits loaded GLB into its three chunks.
    ///
    /// * Mandatory GLB header.
    /// * Mandatory JSON chunk.
    /// * Optional BIN chunk.
    fn split(&self) -> Result<Split, String> {
        use std::mem::{size_of, transmute};
        let err = |reason: &str| Err(reason.to_string());
        let glb = &self.0;
        let glb_ptr = glb.as_ptr();
        if glb.len() < size_of::<Header>() + size_of::<ChunkInfo>() {
            return err("GLB missing header");
        }

        // Offset in bytes into `glb`.
        let mut offset = 0isize;

        let header = unsafe {
            let x: *const Header = transmute(glb_ptr.offset(offset));
            if &(*x).magic != b"glTF" {
                return err("GLB does not start with `glTF`");
            }
            if (*x).length as usize != glb.len() {
                return err("length does not match length of data");
            }
            if (*x).version != 2 {
                return err("Not GLB version 2.0");
            }
            *x
        };

        offset += size_of::<Header>() as isize;
        let (json_chunk, json_chunk_length) = unsafe {
            let x: *const ChunkInfo = transmute(glb_ptr.offset(offset));
            offset += size_of::<ChunkInfo>() as isize;
            if (*x).length as usize > (glb.len() as isize - offset) as usize {
                return err("JSON chunkLength exceeds length of data");
            }
            if &(*x).kind != b"JSON" {
                return err("JSON chunkType is not `JSON`");
            }
            let begin = offset as usize;
            let end = begin + (*x).length as usize;
            (begin..end, (*x).length as usize)
        };

        offset += json_chunk_length as isize;
        let blob_chunk = if glb.len() as isize - offset == 0 {
            None
        } else {
            unsafe {
                let x: *const ChunkInfo = transmute(glb_ptr.offset(offset));
                offset += size_of::<ChunkInfo>() as isize;
                if (*x).length as usize > (glb.len() as isize - offset) as usize {
                    return err("BIN chunkLength exceeds length of data");
                }
                if &(*x).kind != b"BIN\0" {
                    return err("BIN chunkType is not `BIN\0`");
                }
                let begin = offset as usize;
                let end = begin + (*x).length as usize;
                Some(begin..end)
            }
        };

        Ok((header, json_chunk, blob_chunk))
    }
}

/// Visits the items in an `Accessor`.
#[derive(Clone, Debug)]
pub struct Iter<'a, T> {
    /// The total number of iterations left.
    count: usize,

    /// The index of the next iteration.
    index: usize,

    /// The number of bytes between each item.
    stride: usize,

    /// Byte offset into the buffer view where the items begin.
    offset: usize,
    
    /// The accessor we're iterating over.
    accessor: gltf::Accessor<'a>,

    /// Consumes the data type we're returning at each iteration.
    _marker: marker::PhantomData<T>,
}

pub fn normals<'a, S>(primitive: &gltf::Primitive, source: &S) -> Option<Normals<'a>>
    where S: Source
{
    unimplemented!()
}

/// XYZ vertex normals of type `[f32; 3]`.
#[derive(Clone, Debug)]
pub struct Normals<'a>(Iter<'a, [f32; 3]>);

/// XYZ vertex positions of type `[f32; 3]`.
#[derive(Clone, Debug)]
pub struct Positions<'a>(Iter<'a, [f32; 3]>);

/// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
/// sign value (-1 or +1) indicating the handedness of the tangent basis.
#[derive(Clone, Debug)]
pub struct Tangents<'a>(Iter<'a, [f32; 4]>);

/// Vertex colors.
#[derive(Clone, Debug)]
pub enum Colors<'a> {
    /// RGB vertex color of type `[u8; 3]>`.
    RgbU8(Iter<'a, [u8; 3]>),

    /// RGBA vertex color of type `[u8; 4]>`.
    RgbaU8(Iter<'a, [u8; 4]>),

    /// RGB vertex color of type `[u16; 3]>`.
    RgbU16(Iter<'a, [u16; 3]>),

    /// RGBA vertex color of type `[u16; 4]>`.
    RgbaU16(Iter<'a, [u16; 4]>),

    /// RGB vertex color of type `[f32; 3]`.
    RgbF32(Iter<'a, [f32; 3]>),

    /// RGBA vertex color of type `[f32; 4]`.
    RgbaF32(Iter<'a, [f32; 4]>),
}

/// Index data.
#[derive(Clone, Debug)]
pub enum Indices<'a> {
    /// Index data of type U8
    U8(Iter<'a, u8>),
    /// Index data of type U16
    U16(Iter<'a, u16>),
    /// Index data of type U32
    U32(Iter<'a, u32>),
}

/// Index data coerced into `u32` values.
#[derive(Clone, Debug)]
pub struct IndicesU32<'a>(Indices<'a>);

/// Texture co-ordinates coerced into `[f32; 2]` values.
#[derive(Clone, Debug)]
pub struct TexCoordsF32<'a>(TexCoords<'a>);

/// Vertex joints.
/// Refer to the documentation on morph targets and skins for more
/// information.
#[derive(Clone, Debug)]
pub enum Joints<'a> {
    /// Joints of type `[u8; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U8(Iter<'a, [u8; 4]>),
    
    /// Joints of type `[u16; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U16(Iter<'a, [u16; 4]>),
}

/// UV texture co-ordinates.
#[derive(Clone, Debug)]
pub enum TexCoords<'a> {
    /// UV texture co-ordinates of type `[f32; 2]`.
    F32(Iter<'a, [f32; 2]>),

    /// UV texture co-ordinates of type `[u8; 2]>`.
    U8(Iter<'a, [u8; 2]>),

    /// UV texture co-ordinates of type `[u16; 2]>`.
    U16(Iter<'a, [u16; 2]>),
}

/// Weights,
/// Refer to the documentation on morph targets for more information.
#[derive(Clone, Debug)]
pub enum Weights<'a> {
    /// Weights of type `[f32; 4]`.
    F32(Iter<'a, [f32; 4]>),

    /// Weights of type `[u8; 4]`.
    U8(Iter<'a, [u8; 4]>),

    /// Weights of type `[u16; 4]`.
    U16(Iter<'a, [u16; 4]>),
}
