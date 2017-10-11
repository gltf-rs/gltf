
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Error;
use GlbError;

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
struct ChunkInfo<'a> {
    /// The length of the chunk data in byte excluding the header.
    length: u32,
    /// Either `b"JSON"` or `b"BIN\0"`.
    kind: [u8; 4],
    // Data follows...
    data: &'a [u8],
}

impl<'a> Glb<'a> {
    /// Splits loaded GLB into its three chunks.
    ///
    /// * Mandatory GLB header.
    /// * Mandatory JSON chunk.
    /// * Optional BIN chunk.
    pub fn from_slice(data: &'a [u8]) -> Result<Self, Error> {
        use byteorder::{LE, ReadBytesExt};
        use std::io::{Cursor, Read, Seek, SeekFrom};
        use std::mem::size_of;
        if data.len() < size_of::<Header>() + size_of::<ChunkInfo>() {
            return Err(Error::Glb(GlbError::MissingHeader));
        }

        // TODO: Clean up
        let mut reader = Cursor::new(data);
        let header = {
            let mut magic = [0_u8; 4];
            reader.read_exact(&mut magic).unwrap();
            let version = reader.read_u32::<LE>().unwrap();
            let length  = reader.read_u32::<LE>().unwrap();
            if &magic != b"glTF" {
                return Err(Error::Glb(GlbError::Magic(magic)));
            }
            if version != 2 {
                return Err(Error::Glb(GlbError::Version));
            }
            if length as usize > data.len() {
                return Err(Error::Glb(GlbError::Length));
            }
            Header { magic, version, length }
        };

        let json = {
            let length = reader.read_u32::<LE>().unwrap();
            let mut kind = [0_u8; 4];
            reader.read_exact(&mut kind).unwrap();
            if &kind != b"JSON" {
                return Err(Error::Glb(GlbError::JsonChunkType));
            }
            let start = reader.position() as usize;
            if start + length as usize > data.len() {
                return Err(Error::Glb(GlbError::JsonChunkLength));
            }
            let end = start + length as usize;
            &data[start .. end]
        };

        let bin = if reader.seek(SeekFrom::Current((json.len() as i64 + 3) & !3)).is_ok() {
            let length = reader.read_u32::<LE>().unwrap();
            let mut kind = [0_u8; 4];
            reader.read_exact(&mut kind).unwrap();
            if &kind != b"BIN\0" {
                return Err(Error::Glb(GlbError::BinChunkType));
            }
            let start = reader.position() as usize;
            if start + length as usize > data.len() {
                return Err(Error::Glb(GlbError::BinChunkLength));
            }
            let end = start + length as usize;
            Some(start .. end).map(|range| &data[range])
        } else {
            None
        };

        Ok(Glb { header, json, bin })
    }
}
