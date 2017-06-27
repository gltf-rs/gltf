
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gltf::{BufferData, ImageData};
use import::{Error, Source};
use json;
use root::Root;
use std::io::Read;
use Gltf;

/// The contents of a .glb file.
#[derive(Clone, Debug)]
struct Glb(Vec<u8>);

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

/// Imports binary glTF (GLB).
#[derive(Clone, Debug)]
pub struct Importer {
    /// The loaded contents of a .glb file.
    glb: Glb,
}

/// The header, JSON, and BIN sections of a .glb file, respectively.
type Split = (Header, Slice, Option<Slice>);

impl Glb {
    /// Splits loaded GLB into its three chunks.
    ///
    /// * Mandatory GLB header.
    /// * Mandatory JSON chunk.
    /// * Optional BIN chunk.
    fn split<S: Source>(&self) -> Result<Split, Error<S>> {
        use std::mem::{size_of, transmute};
        let err = |reason: &str| Err(Error::MalformedGlb(reason.to_string()));
        let glb = self.0.as_slice();
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
        let json_chunk = unsafe {
            let x: *const ChunkInfo = transmute(glb_ptr.offset(offset));
            offset += size_of::<ChunkInfo>() as isize;
            if (*x).length as usize > (glb.len() as isize - offset) as usize {
                return err("JSON chunkLength exceeds length of data");
            }
            if &(*x).kind != b"JSON" {
                return err("JSON chunkType is not `JSON`");
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
                let x: *const ChunkInfo = transmute(glb_ptr.offset(offset));
                offset += size_of::<ChunkInfo>() as isize;
                if (*x).length as usize > (glb.len() as isize - offset) as usize {
                    return err("BIN chunkLength exceeds length of data");
                }
                if &(*x).kind != b"BIN\0" {
                    return err("BIN chunkType is not `BIN\0`");
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
            glb: Glb(vec![]),
        }
    }
    
    /// Clears any data held by the importer.
    /// Must be called at the beginning of each import call.
    fn clear(&mut self) {
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

        // Read .glb file.
        let _ = reader.read_to_end(&mut self.glb.0)?;
        debug_assert!(self.glb.0.starts_with(b"glTF"));

        // Split the GLB into its three chunks.
        let (_header, json_chunk, blob_chunk) = self.glb.split()?;
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

        // Preload the glTF buffer data.
        let mut buffers = vec![];
        {
            // When provided, the internal GLB buffer data is the first in the array.
            let mut buffer_iter = root.buffers.iter();
            if let Some(chunk) = blob_chunk {
                let _ = buffer_iter.next();
                let slice = self.slice(chunk);
                // TODO: Avoid making a copy of this data.
                //
                // This could be achieved by reading the BIN chunk of a .glb file
                // into its own `Vec`.
                let data = slice.to_vec();
                buffers.push(BufferData::Owned(data.into_boxed_slice()));
            }

            // Read any other external GLB buffers.
            for entry in buffer_iter {
                let uri = entry.uri.as_ref().unwrap(); 
                let mut data = vec![];
                let _ = source
                    .read_external_data(uri)
                    .map_err(Error::Source)?
                .read_to_end(&mut data)?;
                buffers.push(BufferData::Owned(data.into_boxed_slice()));
            }
        }

        // Preload the glTF image data.
        let mut images = vec![];
        for entry in &root.images {
            images.push(if let Some(index) = entry.buffer_view.as_ref() {
                ImageData::Borrowed(index.value())
            } else {
                // Read the external GLB image data.
                let mut data = vec![];
                let _ = source
                    .read_external_data(entry.uri.as_ref().unwrap())
                    .map_err(Error::Source)?
                    .read_to_end(&mut data)?;
                ImageData::Owned(data.into_boxed_slice())
            });
        }

        Ok(Gltf::new(Root::new(root), buffers, images))
    }
}
