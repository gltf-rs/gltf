
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use import;
use std::ops;

use futures::{BoxFuture, Future, Poll};
use std::boxed::Box;
use std::sync::Arc;

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
    /// A `Future` that resolves to either a `SharedItem<Box<[u8]>>` or else an
    /// `AsyncError`.
    future: BoxFuture<Box<[u8]>, import::Error<S>>,

    /// The subset the data that is required once available.
    region: Region,
}

/// Concrete and thread-safe glTF data.
///
/// May represent `Buffer`, `View`, or `Image` data.
#[derive(Clone, Debug)]
pub struct Data {
    /// The resolved data.
    item: Arc<Box<[u8]>>,

    /// The byte region the data reads from.
    region: Region,
}

impl<S: import::Source> Async<S> {
    /// Constructs `AsyncData` that uses all data from the given future. 
    pub fn full<F>(future: F) -> Self
        where F: Future<Item = Box<[u8]>, Error = S::Error> + Send + 'static
    {
        Async {
            future: future.map_err(import::Error::Source).boxed(),
            region: Region::Full,
        }
    }

    /// Constructs `AsyncData` that uses a subset of the data from the given future.
    pub fn view<F>(future: F, offset: usize, len: usize) -> Self
    where F: Future<Item = Box<[u8]>, Error = S::Error> + Send + 'static
    {
        Async {
            future: future.map_err(import::Error::Source).boxed(),
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
                        Region::Full => {
                            Data::full(Arc::new(item))
                        },
                        Region::View { offset, len } => {
                            Data::view(Arc::new(item), offset, len)
                        },
                    }
                })
            })
    }
}

impl Data {
    /// Constructs concrete and thread-safe glTF data.
    ///
    /// # Notes
    ///
    /// This method is unstable and hence subject to change.
    pub fn full(item: Arc<Box<[u8]>>) -> Self {
        Data {
            item: item,
            region: Region::Full,
        }
    }

    /// Constructs a concrete and thread-safe subset of glTF data.
    ///
    /// # Notes
    ///
    /// This method is unstable and hence subject to change.
    pub fn view(item: Arc<Box<[u8]>>, offset: usize, len: usize) -> Self {
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
