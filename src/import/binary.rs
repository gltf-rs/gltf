
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use futures::{self, future};
use import::{self, config, data};
use json;

use futures::{Future, Poll};
use image_crate::{load_from_memory, load_from_memory_with_format};
use image_crate::ImageFormat as Format;
use image_crate::ImageFormat::{JPEG as Jpeg, PNG as Png};
use image_crate::ImageResult;
use import::{Config, Source};
use root::Root;
use {Data, DynamicImage, Gltf};

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

enum AsyncImage<S: import::Source> {
    /// Image data is borrowed from a buffer.
    Borrowed {
        /// The buffer index.
        index: usize,

        /// Byte offset into the indexed buffer where the image data begins.
        offset: usize,

        /// Length of the image past the offset in bytes.
        len: usize,

        /// The image format.
        format: Format,
    },

    /// Image data is owned.
    Owned {
        /// A `Future` that drives the loading of external image data.
        data: data::Async<S>,

        /// The image format.
        format: Option<Format>,
    },
}

impl<S: import::Source> Future for AsyncImage<S> {
    type Item = EncodedImage;
    type Error = import::Error<S>;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self {
            &mut AsyncImage::Borrowed { index, offset, len, format } => {
                Ok(futures::Async::Ready(EncodedImage::Borrowed {
                    index: index,
                    offset: offset,
                    len: len,
                    format: format,
                }))
            },
            &mut AsyncImage::Owned { ref mut data, format } => {
                data.poll()
                    .map(|async| {
                        async.map(|data| {
                            EncodedImage::Owned {
                                data: data,
                                format: format,
                            }
                        })
                    })
            },
        }
    }
}

enum EncodedImage {
    /// Image data is borrowed from a buffer.
    Borrowed {
        /// The buffer index.
        index: usize,

        /// Byte offset into the indexed buffer where the image data begins.
        offset: usize,

        /// Length of the image past the offset in bytes.
        len: usize,

        /// The image format.
        format: Format,
    },

    /// Image data is owned.
    Owned {
        /// A `Future` that drives the loading of external image data.
        data: Data,

        /// The image format.
        format: Option<Format>,
    },
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
    fn split<S: import::Source>(&self) -> Result<Split, import::Error<S>> {
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

fn source_buffers<S: import::Source>(
    root: &Root,
    source: &S,
    blob: Option<Box<[u8]>>,
) -> Vec<data::Async<S>> {
    let json = root.as_json();
    let mut iter = json.buffers.iter();
    let mut buffers = vec![];
    if let Some(data) = blob {
        let future = Box::new(future::ok(data));
        // Skip the first buffer entry.
        let _ = iter.next();
        buffers.push(data::Async::full(future));
    }
    for entry in iter {
        let uri = entry.uri.as_ref().unwrap();
        let future = Box::new(source.source_external_data(uri));
        buffers.push(data::Async::full(future));
    }
    buffers
}

fn source_images<S: import::Source>(
    root: &Root,
    source: &S,
) -> Vec<AsyncImage<S>> {
    root.as_json().images
        .iter()
        .map(|entry| {
            let format = entry.mime_type.as_ref().map(|x| match x.0.as_str() {
                "image/jpeg" => Jpeg,
                "image/png" => Png,
                _ => unreachable!(),
            });
            if let Some(uri) = entry.uri.as_ref() {
                let future = Box::new(source.source_external_data(uri));
                AsyncImage::Owned {
                    data: data::Async::full(future),
                    format: format,
                }
            } else if let Some(index) = entry.buffer_view.as_ref() {
                let buffer_view = &root.as_json().buffer_views[index.value()];
                AsyncImage::Borrowed {
                    index: buffer_view.buffer.value(),
                    offset: buffer_view.byte_offset as usize,
                    len: buffer_view.byte_length as usize,
                    format: format.unwrap(),
                }
            } else {
                unreachable!()
            }
        })
        .collect()
}

fn decode_images(
    buffers: &[Data],
    images: Vec<EncodedImage>,
) -> ImageResult<Vec<DynamicImage>> {
    images
        .iter()
        .map(|entry| {
            match entry {
                &EncodedImage::Borrowed { index, offset, len, format } => {
                    let data = &buffers[index][offset..(offset + len)];
                    load_from_memory_with_format(data, format)
                },
                &EncodedImage::Owned { ref data, format: Some(format) } => {
                    load_from_memory_with_format(data, format)
                },
                &EncodedImage::Owned { ref data, format: None } => {
                    load_from_memory(data)
                },
            }
        })
        .collect()
}

fn validate<S: Source>(
    json: json::Root,
    strategy: config::ValidationStrategy, 
    has_blob: bool,
) -> Box<Future<Item = Root, Error = import::Error<S>>> {
    use validation::{Error as Reason, Validate};
    Box::new(future::lazy(move || {
        let mut errs = vec![];
        match strategy {
            config::ValidationStrategy::Skip => {
                return Ok(Root::new(json));
            },
            config::ValidationStrategy::Minimal => {
                json.validate_minimally(
                    &json,
                    || json::Path::new(),
                    &mut |path, err| {
                        errs.push((path(), err));
                    },
                )
            },
            config::ValidationStrategy::Complete => {
                json.validate_completely(
                    &json,
                    || json::Path::new(),
                    &mut |path, err| {
                        errs.push((path(), err));
                    },
                )
            }
        }

        // Required for the `Minimal` and `Complete` validation strategies.
        for (index, buffer) in json.buffers.iter().enumerate() {
            let path = || {
                json::Path::new()
                    .field("buffers")
                    .index(index)
                    .field("uri")
            };
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
            Ok(Root::new(json))
        } else {
            Err(import::Error::Validation(errs))
        }
    }))
}

/// Imports some glTF from the given data source.
pub fn import<S: import::Source>(
    data: Box<[u8]>,
    source: S,
    config: Config,
) -> Box<Future<Item = Gltf, Error = import::Error<S>>> {
    Box::new(future::lazy(move || {
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
            let has_blob = blob.is_some();
            validate(json, config.validation_strategy, has_blob)
                .join(future::ok(blob))
        })
        .and_then(move |(root, blob)| {
            let source = source;
            let buffers = source_buffers(&root, &source, blob);
            let images = source_images(&root, &source);
            future::ok(root)
                .join3(
                    future::join_all(buffers),
                    future::join_all(images),
                )
        })
        .and_then(|(root, buffers, images)| {
            let decoded_images = decode_images(&buffers, images)?;
            Ok((root, buffers, decoded_images))
        })
        .and_then(|(root, buffers, images)| {
            Ok(Gltf::new(root, buffers, images))
        }))
}

