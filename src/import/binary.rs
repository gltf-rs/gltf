
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use futures::future;
use import::{self, config};
use json;

use futures::{BoxFuture, Future};
use image_crate::{load_from_memory, load_from_memory_with_format};
use image_crate::ImageFormat::{JPEG as Jpeg, PNG as Png};
use import::{Config, Source};
use root::Root;
use {AsyncData, Gltf};

/// The contents of a .glb file.
#[derive(Clone, Debug)]
struct Glb(Box<[u8]>);

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
type Split = (Header, Slice, Option<Slice>);

impl Glb {
    /// Obtains a slice of the GLB data.
    fn slice(&self, slice: Slice) -> &[u8] {
        let begin = slice.offset;
        let end = begin + slice.length;
        &self.0[begin..end]
    }

    /// Splits loaded GLB into its three chunks.
    ///
    /// * Mandatory GLB header.
    /// * Mandatory JSON chunk.
    /// * Optional BIN chunk.
    fn split(&self) -> Result<Split, import::Error> {
        use std::mem::{size_of, transmute};
        let err = |reason: &str| Err(import::Error::MalformedGlb(reason.to_string()));
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

fn source_buffers(
    json: &json::Root,
    blob: Option<Box<[u8]>>,
    source: &Source,
) -> Vec<AsyncData> {
    let mut iter = json.buffers.iter();
    let mut buffers = vec![];
    if let Some(data) = blob {
        let future = future::ok(data).boxed().shared();
        // Skip the first buffer entry.
        let _ = iter.next();
        buffers.push(AsyncData::full(future));
    }
    for entry in iter {
        let uri = entry.uri.as_ref().unwrap();
        let future = source
            .source_external_data(uri)
            .boxed()
            .shared();
        buffers.push(AsyncData::full(future));
    }
    buffers
}

fn source_images(
    json: &json::Root,
    buffers: &[AsyncData],
    source: &Source,
) -> Vec<AsyncData> {
    enum Type<'a> {
        Borrowed {
            buffer_index: usize,
            offset: usize,
            len: usize,
        },
        Owned {
            uri: &'a str,
        },
    }
    let mut images = vec![];
    for entry in &json.images {
        let format = entry.mime_type.as_ref().map(|x| match x.0.as_str() {
            "image/jpeg" => Jpeg,
            "image/png" => Png,
            _ => unreachable!(),
        });
        let ty = if let Some(uri) = entry.uri.as_ref() {
            Type::Owned {
                uri: uri,
            }
        } else if let Some(index) = entry.buffer_view.as_ref() {
            let buffer_view = &json.buffer_views[index.value()];
            Type::Borrowed {
                buffer_index: buffer_view.buffer.value(),
                offset: buffer_view.byte_offset as usize,
                len: buffer_view.byte_length as usize,
            }
        } else {
            unreachable!()
        };
        let future = match ty {
            Type::Owned {
                uri,
            } => {
                source
                    .source_external_data(uri)
                    .and_then(move |data| {
                        if let Some(format) = format {
                            match load_from_memory_with_format(&data, format) {
                                Ok(image) => {
                                    let pixels = image
                                        .raw_pixels()
                                        .into_boxed_slice();
                                    future::ok(pixels)
                                },
                                Err(err) => {
                                    future::err(import::Error::Decode(err))
                                },
                            }
                        } else {
                            match load_from_memory(&data) {
                                Ok(image) => {
                                    let pixels = image
                                        .raw_pixels()
                                        .into_boxed_slice();
                                    future::ok(pixels)
                                },
                                Err(err) => {
                                    future::err(import::Error::Decode(err))
                                },
                            }
                        }
                    })
                    .boxed()
                    .shared()
            },
            Type::Borrowed {
                buffer_index,
                offset,
                len,
            } => {
                buffers[buffer_index]
                    .clone()
                    .map_err(import::Error::LazyLoading)
                    .and_then(move |data| {
                        let slice = &data[offset..(offset + len)];
                        match load_from_memory_with_format(slice, format.unwrap()) {
                            Ok(image) => {
                                let pixels = image
                                    .raw_pixels()
                                    .into_boxed_slice();
                                future::ok(pixels)
                            },
                            Err(err) => {
                                future::err(import::Error::Decode(err))
                            },
                        }
                    })
                    .boxed()
                    .shared()
            },
        };
        images.push(AsyncData::full(future));
    }
    images
}

fn validate(
    json: json::Root,
    has_blob: bool,
    strategy: config::ValidationStrategy, 
) -> Result<json::Root, import::Error> {
    use validation::{Error as Reason, Validate};
    
    let mut errs = vec![];
    match strategy {
        config::ValidationStrategy::Skip => {
            return Ok(json);
        },
        config::ValidationStrategy::Minimal => {
            json.validate_minimally(&json, || json::Path::new(), &mut |path, err| {
                errs.push((path(), err));
            })
        },
        config::ValidationStrategy::Complete => {
            json.validate_completely(&json, || json::Path::new(), &mut |path, err| {
                errs.push((path(), err));
            })
        }
    }

    // Required for the `Minimal` and `Complete` validation strategies.
    for (index, buffer) in json.buffers.iter().enumerate() {
        let path = || json::Path::new().field("buffers").index(index).field("uri");
        match index {
            0 if has_blob => if buffer.uri.is_some() {
                errs.push((path(), Reason::Missing));
            },
            _ if buffer.uri.is_none() => {
                errs.push((path(), Reason::Missing));
            },
            _ => {},
        }
    }

    if errs.is_empty() {
        Ok(json)
    } else {
        Err(import::Error::Validation(errs))
    }
}

/// Imports some glTF from the given data source.
pub fn import<S>(
    data: Box<[u8]>,
    source: S,
    config: Config,
) -> BoxFuture<Gltf, import::Error>
    where S: Source
{
    let gltf = future::lazy(move || {
        let glb = Glb(data);
        let (_, json_chunk, blob_chunk) = glb.split()?;
        let begin = json_chunk.offset;
        let end = begin + json_chunk.length;
        let json: json::Root = json::from_slice(&glb.0[begin..end])?;
        let blob = blob_chunk.map(|chunk| {
            glb.slice(chunk).to_vec().into_boxed_slice()
        });
        Ok((json, blob))
    })
        .and_then(move |(json, blob)| {
            let config = config;
            match validate(json, blob.is_some(), config.validation_strategy) {
                Ok(json) => future::ok((json, blob)).boxed(),
                Err(err) => future::err(err).boxed(),
            }
        })
        .and_then(move |(json, blob)| {
            let source = source;
            let buffers = source_buffers(&json, blob, &source);
            future::ok((json, source, buffers))
        })
        .and_then(|(json, source, buffers)| {
            let images = source_images(&json, &buffers, &source);
            future::ok((json, buffers, images))
        })
        .and_then(|(json, buffers, images)| {
            future::ok(Gltf::new(Root::new(json), buffers, images))
        });
    Box::new(gltf)
}

