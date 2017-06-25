
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

/// Imports standard glTF.
#[derive(Clone)]
pub struct Importer {
    /// The glTF JSON data.
    gltf: Vec<u8>,

    /// The imported glTF buffers.
    buffers: Vec<Vec<u8>>,

    /// The imported glTF images.
    images: Vec<Image>,
}

/// Describes image data required to render a single glTF asset.
#[derive(Clone, Debug)]
enum Image {
    /// The image data is borrowed from the indexed buffer view.
    Borrowed(usize),

    /// The image data is owned.
    Owned(Vec<u8>),
}

impl Importer {
    /// Constructs an `Importer`.
    pub fn new() -> Self {
        Self {
            buffers: vec![],
            images: vec![],
            gltf: vec![],
        }
    }
    
    /// Clears any data held by the importer.
    /// Must be called at the beginning of each import call.
    fn clear(&mut self) {
        self.buffers.clear();
        self.images.clear();
        self.gltf.clear();
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
        use self::Error::*;
        use std::io::Read;
        use validation::{Error as Oops, JsonPath, Validate};

        // Cleanup from the last import call.
        self.clear();

        // Read .gltf / .glb file.
        let _ = reader.read_to_end(&mut self.gltf)?;
        if self.gltf.starts_with(b"glTF") {
            unreachable!()
        }

        // Parse and validate the .gltf JSON data
        let root: json::Root = json::from_slice(&self.gltf)?;
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

        // Read the glTF buffer data
        for entry in &root.buffers {
            let mut buffer = vec![];
            let _ = source
                .read_external_data(entry.uri.as_ref().unwrap().as_ref())
                .map_err(Source)?
                .read_to_end(&mut buffer)?;
            self.buffers.push(buffer);
        }

        // Read the glTF image data
        for entry in &root.images {
            let image = if let Some(index) = entry.buffer_view.as_ref() {
                Image::Borrowed(index.value())
            } else if let Some(uri) = entry.uri.as_ref() {
                let mut buffer = vec![];
                let _ = source
                    .read_external_data(uri)
                    .map_err(Source)?
                    .read_to_end(&mut buffer)?;
                Image::Owned(buffer)
            } else {
                unreachable!()
            };
            self.images.push(image);
        }

        let buffer_data: Vec<_>  = self.buffers.iter().map(Vec::as_slice).collect();
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
