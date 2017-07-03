
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use futures::future;
use json;

use futures::future::Shared;
use futures::{BoxFuture, Future};
use image_crate::{load_from_memory, load_from_memory_with_format};
use image_crate::ImageFormat::{JPEG as Jpeg, PNG as Png};
use import::{Error, Source};
use root::Root;
use std::boxed::Box;
use std::io::Cursor;
use validation::Validate;

use Gltf;

fn source_buffers(
    json: &json::Root,
    source: &Source,
) -> Vec<Shared<BoxFuture<Box<[u8]>, Error>>> {
    json.buffers
        .iter()
        .map(|entry| {
            let uri = entry.uri.as_ref().unwrap();
            source
                .source_external_data(uri)
                .boxed()
                .shared()
        })
        .collect()
}

fn source_images(
    json: &json::Root,
    buffers: &[Shared<BoxFuture<Box<[u8]>, Error>>],
    source: &Source,
) -> Vec<Shared<BoxFuture<Box<[u8]>, Error>>> {
    enum Type<'a> {
        Borrowed {
            buffer_index: usize,
            begin: usize,
            end: usize,
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
            let begin = buffer_view.byte_offset as usize;
            let end = begin + buffer_view.byte_length as usize;
            Type::Borrowed {
                buffer_index: buffer_view.buffer.value(),
                begin: begin,
                end: end
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
                                    future::err(Error::Decode(err))
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
                                    future::err(Error::Decode(err))
                                },
                            }
                        }
                    })
                    .boxed()
                    .shared()
            },
            Type::Borrowed {
                buffer_index,
                begin,
                end,
            } => {
                buffers[buffer_index]
                    .clone()
                    .map_err(Error::LazyLoading)
                    .and_then(move |data| {
                        let slice = &data[begin..end];
                        match load_from_memory_with_format(slice, format.unwrap()) {
                            Ok(image) => {
                                let pixels = image
                                    .raw_pixels()
                                    .into_boxed_slice();
                                future::ok(pixels)
                            },
                            Err(err) => {
                                future::err(Error::Decode(err))
                            },
                        }
                    })
                    .boxed()
                    .shared()
            },
        };
        images.push(future);
    }
    images
}

fn validate(json: json::Root) -> BoxFuture<json::Root, Error> {
    let future = future::lazy(move || {
        let mut errs = vec![];
        // TODO: Allow this to be configurable
        json.validate_completely(&json, || json::Path::new(), &mut |path, err| {
            errs.push((path(), err));
        });
        if errs.is_empty() {
            future::ok(json)
        } else {
            future::err(Error::Validation(errs))
        }
    });
    Box::new(future)
}

pub fn import<S>(data: Box<[u8]>, source: S) -> BoxFuture<Gltf, Error>
    where S: Source
{
    let gltf = future::lazy(move || {
        json::from_reader(Cursor::new(data)).map_err(Error::MalformedJson)
    })
        .and_then(|json| validate(json))
        .and_then(move |json| {
            let source = source;
            let buffers = source_buffers(&json, &source);
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
