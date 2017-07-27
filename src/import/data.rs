
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use import;
use std::ops;

use futures::{Future, Poll};
use std::boxed::Box;

/// Represents decoded image data.
#[derive(Clone, Debug)]
pub struct DecodedImage {
    /// The image width.
    width: u32,

    /// The image height.
    height: u32,

    /// The raw pixel data.
    pixels: Data,
}

/// Represents a contiguous subset of either `AsyncData` or concrete `Data`.
#[derive(Clone, Copy, Debug)]
enum Region {
    /// Represents the whole contents of the parent data. 
    Full,

    /// Represents a subset of the contents of the parent data.
    View {
        /// Byte offset where the data region begins.
        offset: usize,

        /// Byte length past the offset where the data region ends.
        len: usize,
    },
}

/// A `Future` that drives the acquisition of glTF data.
pub struct Async<S: import::Source> {
    /// A `Future` that resolves to a `Box<[u8]>`.
    future: Box<Future<Item = Box<[u8]>, Error = import::Error<S>>>,

    /// The subset the data that is required once available.
    region: Region,
}

/// Concrete glTF data.
///
/// May represent `Buffer`, `View`, or `Image` data.
#[derive(Clone, Debug)]
pub struct Data {
    /// The resolved data.
    item: Box<[u8]>,

    /// The byte region the data reads from.
    region: Region,
}

impl<S: import::Source> Async<S> {
    /// Constructs `AsyncData` that uses all data from the given future. 
    pub fn full(future: Box<Future<Item = Box<[u8]>, Error = S::Error>>) -> Self {
        Async {
            future: Box::new(future.map_err(import::Error::Source)),
            region: Region::Full,
        }
    }

    /// Constructs `AsyncData` that uses a subset of the data from the given future.
    pub fn view(
        future: Box<Future<Item = Box<[u8]>, Error = S::Error>>,
        offset: usize,
        len: usize,
    ) -> Self {
        Async {
            future: Box::new(future.map_err(import::Error::Source)),
            region: Region::View { offset, len },
        }
    }

    /// Consumes this `AsyncData`, constructing a subset instead.
    ///
    /// If the data is already a subset then a sub-subset is created, etc.
    pub fn subview(self, offset: usize, len: usize) -> Self {
        Async {
            future: self.future,
            region: self.region.subview(offset, len),
        }
    }
}

impl<S: import::Source> Future for Async<S> {
    type Item = Data;
    type Error = import::Error<S>;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future
            .poll()
            .map(|async| {
                async.map(|item| {
                    match self.region {
                        Region::Full => Data::full(item),
                        Region::View { offset, len } => Data::view(item, offset, len),
                    }
                })
            })
    }
}

impl Data {
    /// Constructs concrete glTF data.
    ///
    /// # Notes
    ///
    /// This method is unstable and hence subject to change.
    pub fn full(item: Box<[u8]>) -> Self {
        Data {
            item: item,
            region: Region::Full,
        }
    }

    /// Constructs a subset of concrete glTF data.
    ///
    /// # Notes
    ///
    /// This method is unstable and hence subject to change.
    pub fn view(item: Box<[u8]>, offset: usize, len: usize) -> Self {
        Data {
            item: item,
            region: Region::View { offset, len },
        }
    }

    /// Consumes this `Data`, constructing a subset instead.
    ///
    /// If the data is already a subset then a sub-subset is created, etc.
    pub fn subview(self, offset: usize, len: usize) -> Self {
        Data {
            item: self.item,
            region: self.region.subview(offset, len),
        }
    }
}

impl ops::Deref for Data {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        match self.region {
            Region::Full => &self.item[..],
            Region::View { offset, len } => &self.item[offset..(offset + len)],
        }
    }
}

impl Region {
    /// Consumes this `Region`, constructing a view instead.
    ///
    /// If the region is already a view then a subview is created, etc.
    pub fn subview(self, offset: usize, len: usize) -> Region {
        match self {
            Region::Full => {
                Region::View {
                    offset: offset,
                    len: len,
                }
            },
            Region::View {
                offset: prev_offset,
                len: _,
            } => {
                Region::View {
                    offset: prev_offset + offset,
                    len: len,
                }
            },
        }
    }
}

impl DecodedImage {
    /// Constructs a `DecodedImage`.
    pub fn new(width: u32, height: u32, pixels: Data) -> Self {
        DecodedImage {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    /// Returns the image width in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the image height in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the raw image pixel data.
    pub fn raw_pixels(&self) -> &[u8] {
        &self.pixels
    }
}
