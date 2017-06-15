
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
use json::*;

pub use self::error::Error;

/// Trait for validating glTF JSON data against the 2.0 specification.
pub trait Validate {
    /// Validates the data against the glTF 2.0 specification.
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
        where
            P: Fn() -> JsonPath,
            R: FnMut(Error);
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

        /// An invalid enumeration constant was identified.
        InvalidEnum(serde_json::Value),

        /// An invalid semantic name was identified.
        InvalidSemanticName(String),

        /// An invalid value was identified.
        InvalidValue(serde_json::Value),

        /// Some required data has been omitted.
        MissingData(String),
    }

    impl Error {
        /// Returns an `IndexOutOfBounds` error.
        pub fn index_out_of_bounds(path: JsonPath) -> Error {
            Error {
                kind: Kind::IndexOutOfBounds,
                path: path,
            }
        }

        /// Returns an `InvalidEnum` error.
        pub fn invalid_enum<V>(path: JsonPath, value: V) -> Error
            where V: Into<serde_json::Value>
        {
            Error {
                kind: Kind::InvalidEnum(value.into()),
                path: path,
            }
        }

        /// Returns an `InvalidSemanticName` error.
        pub fn invalid_semantic_name(path: JsonPath, name: String) -> Error {
            Error {
                kind: Kind::InvalidSemanticName(name),
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

        /// Returns a `MissingData` error.
        pub fn missing_data(path: JsonPath, reason: String) -> Error {
            Error {
                kind: Kind::MissingData(reason),
                path: path,
            }
        }
    }

    impl std::error::Error for Error {
        fn description(&self) -> &str {
            match &self.kind {
                &Kind::IndexOutOfBounds => "Index out of bounds",
                &Kind::InvalidEnum(_) => "Invalid enumeration constant",
                &Kind::InvalidSemanticName(_) => "Invalid semantic name",
                &Kind::InvalidValue(_) => "Invalid value",
                &Kind::MissingData(_) => "Missing data",
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
                &Kind::InvalidEnum(ref value) => {
                    write!(f, "{}: {} ({})", self.path, value, self.description())
                },
                &Kind::InvalidSemanticName(ref name) => {
                    write!(f, "{}: \"{}\" ({})", self.path, name, self.description())
                },
                &Kind::InvalidValue(ref value) => {
                    write!(f, "{}: {} ({})", self.path, value, self.description())
                },
                &Kind::MissingData(ref reason) => {
                    write!(f, "{}: {} ({})", self.path, self.description(), reason)
                },
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
    /// # use gltf::validation::JsonPath;
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
    /// # use gltf::validation::JsonPath;
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
    /// # use gltf::validation::JsonPath;
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
    /// # use gltf::validation::JsonPath;
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
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        for (key, value) in self.iter() {
            value.validate(root, || path().key(key.as_ref()), report);
        }
    }
}

impl<T: Validate> Validate for Option<T> {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        if let Some(value) = self.as_ref() {
            value.validate(root, path, report);
        }
    }
}

impl<T: Validate> Validate for Vec<T> {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        for (index, value) in self.iter().enumerate() {
            value.validate(root, || path().index(index), report);
        }
    }
}

impl Validate for bool {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        // nop
    }
}

impl Validate for u32 {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        // nop
    }
}

impl Validate for i32 {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        // nop
    }
}

impl Validate for f32 {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        // nop
    }
}

impl Validate for [f32; 3] {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        // nop
    }
}

impl Validate for [f32; 4] {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        // nop
    }
}

impl Validate for [f32; 16] {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        // nop
    }
}

impl Validate for () {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        // nop
    }
}

impl Validate for String {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        // nop
    }
}

impl Validate for serde_json::Value {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(Error)
    {
        // nop
    }
}

