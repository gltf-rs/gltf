
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use futures::{self, future};
use import::{self, data};
use json;

use futures::{Future, Poll};
use image_crate::{load_from_memory, load_from_memory_with_format};
use image_crate::GenericImage;
use image_crate::ImageFormat as Format;
use image_crate::ImageResult;
use image_crate::ImageFormat::{JPEG as Jpeg, PNG as Png};
use root::Root;
use std::boxed::Box;
use std::io::Cursor;
use validation::Validate;

use {Data, DecodedImage, Gltf};

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

/// A resolved `AsyncImage`.
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

fn source_buffers<S: import::Source>(
    root: &Root,
    source: &S,
) -> Vec<data::Async<S>> {
    root.as_json().buffers
        .iter()
        .map(|entry| {
            let uri = entry.uri.as_ref().unwrap();
            let future = Box::new(source.source_external_data(uri));
            data::Async::full(future)
        })
        .collect()
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
) -> ImageResult<Vec<DecodedImage>> {
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
            }.map(|decoded_image| {
                let (width, height) = decoded_image.dimensions();
                let raw_pixels = decoded_image.raw_pixels().into_boxed_slice();
                let image_data = Data::full(raw_pixels);
                DecodedImage::new(width as u32, height as u32, image_data)
            })
        })
        .collect()
}

pub fn import<S: import::Source>(
    data: Box<[u8]>,
    source: S,
    config: import::Config,
) -> Box<Future<Item = Gltf, Error = import::Error<S>>> {
    let task = future::lazy(move || {
        let data = data;
        match json::from_reader(Cursor::new(data)) {
            Ok(json) => future::ok(json),
            Err(err) => future::err(import::Error::MalformedJson(err)),
        }
    })
        .and_then(move |json: json::Root| {
            let config = config;
            match config.validation_strategy {
                import::config::ValidationStrategy::Skip => {
                    future::ok(Root::new(json))
                },
                import::config::ValidationStrategy::Minimal => {
                    let mut errs = vec![];
                    json.validate_minimally(
                        &json,
                        || json::Path::new(),
                        &mut |path, err| errs.push((path(), err)),
                    );
                    if errs.is_empty() {
                        future::ok(Root::new(json))
                    } else {
                        future::err(import::Error::Validation(errs))
                    }
                },
                import::config::ValidationStrategy::Complete => {
                    let mut errs = vec![];
                    json.validate_completely(
                        &json,
                        || json::Path::new(),
                        &mut |path, err| errs.push((path(), err)),
                    );
                    if errs.is_empty() {
                        future::ok(Root::new(json))
                    } else {
                        future::err(import::Error::Validation(errs))
                    }
                },
            }
        })
        .and_then(move |root| {
            let source = source;
            let buffers = source_buffers(&root, &source);
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
        });
    Box::new(task)
}
