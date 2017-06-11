
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_json;
use std;
use std::collections::HashMap;
use v2::json::*;
use v2::json::root::TryGet;

pub use self::error::Error;

/// Trait for validating glTF JSON data against the 2.0 specification.
pub trait Validate {
    /// Validates the data against the glTF 2.0 specification.
    fn validate<F>(&self, root: &Root, path: JsonPath, report: &mut F)
        where F: FnMut(Error);
}

/// Contains `Error` and other related data structures.
pub mod error {
    use serde_json;
    use std;
    use super::JsonPath;

    /// Error encountered when validating glTF 2.0 JSON data.
    #[derive(Clone, Debug)]
    pub struct Error {
        /// JSON source path of the offending data.
        path: JsonPath,

        /// Error kind.
        kind: Kind,
    }

    /// Specifies what kind of error occured during validation.
    #[derive(Clone, Debug)]
    pub enum Kind {
        /// An index was found to be out of bounds.
        IndexOutOfBounds,

        /// An invalid value was identified.
        InvalidValue(serde_json::Value),
    }

    impl Error {
        /// Returns an `IndexOutOfBounds` error.
        pub fn index_out_of_bounds(path: JsonPath) -> Error {
            Error {
                kind: Kind::IndexOutOfBounds,
                path: path,
            }
        }

        /// Returns an `InvalidValue` error.
        pub fn invalid_value<V>(path: JsonPath, value: V) -> Error
            where V: Into<serde_json::Value>
        {
            Error {
                kind: Kind::InvalidValue(value.into()),
                path: path,
            }
        }
    }

    impl std::error::Error for Error {
        fn description(&self) -> &str {
            match &self.kind {
                &Kind::IndexOutOfBounds => "Index out of bounds",
                &Kind::InvalidValue(_) => "Invalid value",
            }
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            use std::error::Error;
            match &self.kind {
                &Kind::IndexOutOfBounds => {
                    write!(f, "{} ({})", self.path, self.description())
                },
                &Kind::InvalidValue(ref value) => {
                    write!(f, "{}: {} ({})", self.path, value, self.description())
                }
            }
        }
    }
}
    
/// An immutable JSON source path.
#[derive(Clone, Debug)]
pub struct JsonPath(String);

impl JsonPath {
    /// Creates an empty JSON source path.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use gltf::v2::validation::JsonPath;
    /// let path = JsonPath::new();
    /// assert_eq!("", path.as_str());
    /// ```
    pub fn new() -> Self {
        JsonPath(String::new())
    }

    /// Returns a new path ending with the given field.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use gltf::v2::validation::JsonPath;
    /// let path = JsonPath::new().field("foo");
    /// assert_eq!("foo", path.as_str());
    /// assert_eq!("foo.bar", path.field("bar").as_str());
    /// ```
    pub fn field(&self, name: &str) -> Self {
        if self.0.is_empty() {
            JsonPath(name.to_string())
        } else {
            JsonPath(format!("{}.{}", self.0, name))
        }
    }

    /// Returns a new path ending with the given array index.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use gltf::v2::validation::JsonPath;
    /// let path = JsonPath::new().field("foo");
    /// assert_eq!("foo[123]", path.index(123).as_str());
    /// ```
    pub fn index(&self, index: usize) -> Self {
        JsonPath(format!("{}[{}]", self.0, index))
    }

    /// Returns a new path ending with the given object key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use gltf::v2::validation::JsonPath;
    /// let path = JsonPath::new().field("foo");
    /// assert_eq!("foo[\"bar\"]", path.key("bar").as_str());
    /// ```
    pub fn key(&self, key: &str) -> Self {
        JsonPath(format!("{}[\"{}\"]", self.0, key))
    }

    /// Returns a view into the internal representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for JsonPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Validate> Validate for HashMap<String, T> {
    fn validate<F>(&self, root: &Root, path: JsonPath, report: &mut F)
        where F: FnMut(Error)
    {
        for (key, value) in self.iter() {
            value.validate(root, path.key(key), report);
        }
    }
}

impl<T: Validate> Validate for Index<T>
    where Root: TryGet<T>
{
    fn validate<F>(&self, root: &Root, path: JsonPath, mut report: &mut F)
        where F: FnMut(Error)
    {
        if root.try_get(self).is_err() {
            report(Error::index_out_of_bounds(path));
        }
    }
}

impl<T: Validate> Validate for Option<T> {
    fn validate<F>(&self, root: &Root, path: JsonPath, report: &mut F)
        where F: FnMut(Error)
    {
        if let Some(value) = self.as_ref() {
            value.validate(root, path, report);
        }
    }
}

impl<T: Validate> Validate for Vec<T> {
    fn validate<F>(&self, root: &Root, path: JsonPath, report: &mut F)
        where F: FnMut(Error)
    {
        for (index, value) in self.iter().enumerate() {
            value.validate(root, path.index(index), report);
        }
    }
}

impl Validate for bool {
    fn validate<F>(&self, _root: &Root, _path: JsonPath, _report: &mut F)
        where F: FnMut(Error)
    {
        // nop
    }
}

impl Validate for u32 {
    fn validate<F>(&self, _root: &Root, _path: JsonPath, _report: &mut F)
        where F: FnMut(Error)
    {
        // nop
    }
}

impl Validate for i32 {
    fn validate<F>(&self, _root: &Root, _path: JsonPath, _report: &mut F)
        where F: FnMut(Error)
    {
        // nop
    }
}

impl Validate for f32 {
    fn validate<F>(&self, _root: &Root, _path: JsonPath, _report: &mut F)
        where F: FnMut(Error)
    {
        // nop
    }
}

impl Validate for [f32; 3] {
    fn validate<F>(&self, _root: &Root, _path: JsonPath, _report: &mut F)
        where F: FnMut(Error)
    {
        // nop
    }
}

impl Validate for [f32; 4] {
    fn validate<F>(&self, _root: &Root, _path: JsonPath, _report: &mut F)
        where F: FnMut(Error)
    {
        // nop
    }
}

impl Validate for [f32; 16] {
    fn validate<F>(&self, _root: &Root, _path: JsonPath, _report: &mut F)
        where F: FnMut(Error)
    {
        // nop
    }
}

impl Validate for () {
    fn validate<F>(&self, _root: &Root, _path: JsonPath, _report: &mut F)
        where F: FnMut(Error)
    {
        // nop
    }
}

impl Validate for String {
    fn validate<F>(&self, _root: &Root, _path: JsonPath, _report: &mut F)
        where F: FnMut(Error)
    {
        // nop
    }
}

impl Validate for serde_json::Value {
    fn validate<F>(&self, _root: &Root, _path: JsonPath, _report: &mut F)
        where F: FnMut(Error)
    {
        // nop
    }
}

