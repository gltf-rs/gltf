
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

/// An immutable JSON source path.
#[derive(Clone, Debug, PartialEq)]
pub struct Path(pub String);

impl Path {
    /// Creates an empty JSON source path.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```rust
    /// # use gltf_json::Path;
    /// let path = Path::new();
    /// assert_eq!("", path.as_str());
    /// ```
    pub fn new() -> Self {
        Path(String::new())
    }

    /// Returns a new path ending with the given field.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```rust
    /// # use gltf_json::Path;
    /// let path = Path::new().field("foo");
    /// assert_eq!("foo", path.as_str());
    /// assert_eq!("foo.bar", path.field("bar").as_str());
    /// ```
    pub fn field(&self, name: &str) -> Self {
        if self.0.is_empty() {
            Path(name.to_string())
        } else {
            Path(format!("{}.{}", self.0, name))
        }
    }

    /// Returns a new path ending with the given array index.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```rust
    /// # use gltf_json::Path;
    /// let path = Path::new().field("foo");
    /// assert_eq!("foo[123]", path.index(123).as_str());
    /// ```
    pub fn index(&self, index: usize) -> Self {
        Path(format!("{}[{}]", self.0, index))
    }

    /// Returns a new path ending with the given object key.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```rust
    /// # use gltf_json::Path;
    /// let path = Path::new().field("foo");
    /// assert_eq!("foo[\"bar\"]", path.key("bar").as_str());
    /// ```
    pub fn key(&self, key: &str) -> Self {
        Path(format!("{}[\"{}\"]", self.0, key))
    }

    /// Returns a view into the internal representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
