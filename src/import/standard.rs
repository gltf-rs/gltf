
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

/// Imports standard glTF using static deserialization.
#[derive(Clone)]
pub struct StaticImporter;

fn make_wrapper<'a, S: Source>(
    root: json::Root<'a>,
    source: S,
) -> Result<Gltf<'a>, Error<S>> {
    use self::Error::*;
    use std::io::Read;
    use validation::{Error as Oops, JsonPath, Validate};

    // Parse and validate the .gltf JSON data
    let mut errs = Vec::new();
    root.validate(&root, || JsonPath::new(), &mut |err| errs.push(err));
    for (index, buffer) in root.buffers.iter().enumerate() {
        if buffer.uri.is_none() {
            let path = JsonPath::new().field("buffers").index(index);
            let reason = format!("uri is `undefined`");
            errs.push(Oops::missing_data(path, reason));
        }
    }
    if !errs.is_empty() {
        return Err(Validation(errs));
    }

    // Preload the external glTF buffer data.
    let mut buffers = vec![];
    for entry in &root.buffers {
        let mut buffer = vec![];
        let _ = source
            .read_external_data(entry.uri.as_ref().unwrap().as_ref())
            .map_err(Source)?
        .read_to_end(&mut buffer)?;
        buffers.push(BufferData::Owned(buffer.into_boxed_slice()));
    }

    // Preload the glTF image data.
    let mut images = vec![];
    for entry in &root.images {
        images.push(if let Some(index) = entry.buffer_view.as_ref() {
            ImageData::Borrowed(index.value())
        } else if let Some(uri) = entry.uri.as_ref() {
            // Read the external glTF image data.
            let mut buffer = vec![];
            let _ = source
                .read_external_data(uri)
                .map_err(Source)?
            .read_to_end(&mut buffer)?;
            ImageData::Owned(buffer.into_boxed_slice())
        } else {
            unreachable!()
        });
    }

    Ok(Gltf::new(Root::new(root), buffers, images))
}   

impl StaticImporter {
    /// Constructs a `StaticImporter`.
    pub fn new() -> Self {
        StaticImporter
    }

    pub fn import<R, S>(
        &self,
        reader: R,
        source: S,
    ) -> Result<Gltf<'static>, Error<S>>
    where
        R: Read,
        S: Source,
    {
        let root = json::from_reader(reader)?;
        let gltf = make_wrapper(root, source)?;
        Ok(gltf)
    }
}
