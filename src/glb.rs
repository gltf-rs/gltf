
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use std::error::Error as StdError;

/// Binary `glTF` parsing error.
#[derive(Clone, Debug)]
pub struct ParseError(String);

/// The contents of a .glb file.
#[derive(Clone, Debug)]
pub struct Glb<'a> {
    /// The header section of the `.glb` file.
    pub header: Header,

    /// The JSON section of the `.glb` file.
    pub json: &'a [u8],

    /// The optional BIN section of the `.glb` file.
    pub bin: Option<&'a [u8]>,
}

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
struct ChunkInfo {
    /// The length of the chunk data in byte excluding the header.
    length: u32,

    /// Either `b"JSON"` or `b"BIN\0"`.
    kind: [u8; 4],

    // Data follows... 
}

impl<'a> Glb<'a> {
    /// Splits loaded GLB into its three chunks.
    ///
    /// * Mandatory GLB header.
    /// * Mandatory JSON chunk.
    /// * Optional BIN chunk.
    fn from_slice(data: &'a [u8]) -> Result<Self, ParseError> {
        use std::mem::{size_of, transmute};
        let err = |reason: &str| Err(ParseError(reason.to_string()));
        let ptr = data.as_ptr();
        if data.len() < size_of::<Header>() + size_of::<ChunkInfo>() {
            return err("GLB missing header");
        }

        // Offset in bytes into `glb`.
        let mut offset = 0isize;

        let header = unsafe {
            let x: *const Header = transmute(ptr.offset(offset));
            if &(*x).magic != b"glTF" {
                return err("GLB does not start with `glTF`");
            }
            if (*x).length as usize != data.len() {
                return err("length does not match length of data");
            }
            if (*x).version != 2 {
                return err("Not GLB version 2.0");
            }
            *x
        };

        offset += size_of::<Header>() as isize;
        let (json_chunk, json_chunk_length) = unsafe {
            let x: *const ChunkInfo = transmute(ptr.offset(offset));
            offset += size_of::<ChunkInfo>() as isize;
            if (*x).length as usize > (data.len() as isize - offset) as usize {
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
        let bin_chunk = if data.len() as isize - offset == 0 {
            None
        } else {
            unsafe {
                let x: *const ChunkInfo = transmute(ptr.offset(offset));
                offset += size_of::<ChunkInfo>() as isize;
                if (*x).length as usize > (data.len() as isize - offset) as usize {
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

        let json = &data[json_chunk];
        let bin = bin_chunk.map(|range| &data[range]);
        Ok(Glb {
            header,
            json,
            bin,
        })
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for ParseError {
    fn description(&self) -> &str {
        &self.0
    }
}
