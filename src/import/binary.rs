
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use import::{Error, Source};
use json;
use root::Root;
use std::io::Read;
use Gltf;

/// Describes buffer data required to render a single glTF asset.
#[derive(Clone, Debug)]
enum Buffer {
    /// The buffer data is borrowed from the GLB.
    Internal(Slice),

    /// The buffer data is from an external source and hence owned.
    External(Vec<u8>),
}

/// Describes image data required to render a single glTF asset.
#[derive(Clone, Debug)]
enum Image {
    /// The image data is borrowed from the indexed buffer view.
    Borrowed(usize),

    /// The image data is owned.
    Owned(Vec<u8>),
}

/// The contents of a .glb file.
#[derive(Clone, Debug)]
struct Glb(Vec<u8>);

/// GLB application binary interface.
mod abi {
    /// The header section of a .glb file.
    #[derive(Copy, Clone, Debug)]
    #[repr(C)]
    pub struct Header {
        /// Must be `b"glTF"`.
        pub magic: [u8; 4],

        /// Must be `2`.
        pub version: u32,

        /// Must match the length of the parent .glb file.
        pub length: u32,
    }

    /// The header of the JSON or BIN section of a .glb file.
    #[derive(Copy, Clone, Debug)]
    #[repr(C)]
    pub struct ChunkInfo {
        /// The length of the chunk data in byte excluding the header.
        pub length: u32,

        /// Either `b"JSON"` or `b"BIN\0"`.
        pub kind: [u8; 4],

        // Data follows... 
    }
}

/// Represents a subset of `Importer::glb`.
#[derive(Copy, Clone, Debug, Default)]
struct Slice {
    /// Offset into `glb` in bytes.
    offset: usize,

    /// Length of the slice in bytes.
    length: usize,
}

/// Imports binary glTF (GLB).
#[derive(Clone, Debug)]
pub struct Importer {
    /// The entire GLB binary blob.
    glb: Glb,

    /// The imported glTF buffers.
    buffers: Vec<Buffer>,

    /// The imported glTF images.
    images: Vec<Image>,
}

/// The header, JSON, and BIN sections of a .glb file, respectively.
type Split = (abi::Header, Slice, Option<Slice>);

impl Glb {
    fn split(&self) -> Result<Split, &'static str> {
        use std::mem::{size_of, transmute};

        let glb = self.0.as_slice();
        let glb_ptr = glb.as_ptr();
        if glb.len() < size_of::<abi::Header>() + size_of::<abi::ChunkInfo>() {
            return Err("GLB missing header");
        }

        // Offset in bytes into `glb`.
        let mut offset = 0isize;

        let header = unsafe {
            let x: *const abi::Header = transmute(glb_ptr.offset(offset));
            if &(*x).magic != b"glTF" {
                return Err("GLB does not start with `glTF`");
            }
            if (*x).length as usize != glb.len() {
                return Err("length does not match length of data");
            }
            if (*x).version != 2 {
                return Err("Not GLB version 2.0");
            }
            *x
        };

        offset += size_of::<abi::Header>() as isize;
        let json_chunk = unsafe {
            let x: *const abi::ChunkInfo = transmute(glb_ptr.offset(offset));
            offset += size_of::<abi::ChunkInfo>() as isize;
            if (*x).length as usize > (glb.len() as isize - offset) as usize {
                return Err("JSON chunkLength exceeds length of data");
            }
            if &(*x).kind != b"JSON" {
                return Err("JSON chunkType is not `JSON`");
            }
            Slice {
                offset: offset as usize,
                length: (*x).length as usize,
            }
        };

        offset += json_chunk.length as isize;
        let blob_chunk = if glb.len() as isize - offset == 0 {
            None
        } else {
            unsafe {
                let x: *const abi::ChunkInfo = transmute(glb_ptr.offset(offset));
                offset += size_of::<abi::ChunkInfo>() as isize;
                if (*x).length as usize > (glb.len() as isize - offset) as usize {
                    return Err("BIN chunkLength exceeds length of data");
                }
                if &(*x).kind != b"BIN\0" {
                    return Err("BIN chunkType is not `BIN\0`");
                }
                Some(Slice {
                    offset: offset as usize,
                    length: (*x).length as usize,
                })
            }
        };

        Ok((header, json_chunk, blob_chunk))
    }
}

impl Importer {
    /// Constructs an `Importer`.
    pub fn new() -> Self {
        Self {
            buffers: vec![],
            images: vec![],
            glb: Glb(vec![]),
        }
    }
    
    /// Clears any data held by the importer.
    /// Must be called at the beginning of each import call.
    fn clear(&mut self) {
        self.buffers.clear();
        self.images.clear();
        self.glb.0.clear();
    }

    /// Obtains a slice of the GLB blob.
    fn slice(&self, slice: Slice) -> &[u8] {
        &self.glb.0[slice.offset..(slice.offset + slice.length)]
    }
    
    /// Imports some glTF from the given data source.
    pub fn import<'a, R, S>(
        &'a mut self,
        mut reader: R,
        source: S,
    ) -> Result<Gltf<'a>, Error<S>>
    where
        R: Read,
        S: Source,
    {
        use validation::{Error as Oops, JsonPath, Validate};

        // Cleanup from last import call.
        self.clear();

        // Read GLB contents.
        let _ = reader.read_to_end(&mut self.glb.0)?;
        // TODO: Remove unwrap().
        let (_header, json_chunk, blob_chunk) = self.glb.split().unwrap();
        let root: json::Root = json::from_slice(self.slice(json_chunk))?;

        // Validate the JSON data.
        let mut errs = vec![];
        root.validate(&root, || JsonPath::new(), &mut |err| errs.push(err));
        for (index, buffer) in root.buffers.iter().enumerate() {
            let path = || JsonPath::new().field("buffers").index(index).field("uri");
            match index {
                0 if blob_chunk.is_some() => if buffer.uri.is_some() {
                    let reason = format!("must be `undefined` when BIN is provided");
                    let uri = buffer.uri.as_ref().unwrap().as_ref();
                    errs.push(Oops::invalid_value(path(), uri, reason));
                },
                _ if buffer.uri.is_none() => {
                    let reason = format!("must be defined");
                    errs.push(Oops::missing_data(path(), reason));
                },
                _ => {},
            }
        }

        if !errs.is_empty() {
            return Err(Error::Validation(errs));
        }

        // When provided, the internal GLB buffer data is the first in the array.
        {
            let mut buffer_iter = root.buffers.iter();
            if let Some(chunk) = blob_chunk.as_ref() {
                let _ = buffer_iter.next();
                self.buffers.push(Buffer::Internal(*chunk));
            }
            // Read any other external GLB buffers.
            for entry in buffer_iter {
                let uri = entry.uri.as_ref().unwrap(); 
                let mut data = vec![];
                let _ = source
                    .read_external_data(uri)
                    .map_err(Error::Source)?
                    .read_to_end(&mut data)?;
                self.buffers.push(Buffer::External(data));
            }
        }

        // Read the external GLB image data.
        for entry in &root.images {
            let image = if let Some(index) = entry.buffer_view.as_ref() {
                Image::Borrowed(index.value())
            } else {
                let mut data = vec![];
                let _ = source
                    .read_external_data(entry.uri.as_ref().unwrap())
                    .map_err(Error::Source)?
                .read_to_end(&mut data)?;
                Image::Owned(data)
            };
            self.images.push(image);
        }

        // Prepare references for wrapper interface.
        let mut buffer_data = vec![];
        for entry in &self.buffers {
            buffer_data.push(match entry {
                &Buffer::Internal(ref slice) => self.slice(*slice),
                &Buffer::External(ref vec) => vec.as_slice(),
            });
        }
        let mut image_data = vec![];
        for entry in &self.images {
            image_data.push(match entry {
                &Image::Borrowed(index) => {
                    let ref view = root.buffer_views[index];
                    let begin = view.byte_offset as usize;
                    let end = begin + view.byte_length as usize;
                    &buffer_data[view.buffer.value()][begin..end]
                },
                &Image::Owned(ref data) => data.as_slice(),
            });
        }

        Ok(Gltf::new(Root::new(root), buffer_data, image_data))
    }
}
