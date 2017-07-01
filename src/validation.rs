
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_json;
use std;
use std::fmt;

use json::*;
use std::collections::HashMap;
use std::hash::Hash;

/// Trait for validating glTF JSON data against the 2.0 specification.
pub trait Validate {
    /// Validates the data against the glTF 2.0 specification.
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
        where
            P: Fn() -> JsonPath,
            R: FnMut(&Fn() -> JsonPath, Error);
}

/// Specifies what kind of error occured during validation.
#[derive(Clone, Debug)]
pub enum Error {
    /// An index was found to be out of bounds.
    IndexOutOfBounds,

    /// An invalid value was identified.
    Invalid,

    /// Some required data has been omitted.
    Missing,
}

/// Specifies a type that has been pre-validated during deserialization or otherwise.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Checked<T> {
    /// The item is valid.
    Valid(T),

    /// The item is invalid.
    Invalid,
}

/// An immutable JSON source path.
#[derive(Clone, Debug)]
pub struct JsonPath(String);

impl<T> Checked<T> {
    pub fn unwrap(self) -> T {
        match self {
            Checked::Valid(item) => item,
            Checked::Invalid => panic!("attempted to unwrap an invalid item"),
        }
    }
}

impl<T: Clone> Clone for Checked<T> {
    fn clone(&self) -> Self {
        match self {
            &Checked::Valid(ref item) => Checked::Valid(item.clone()),
            &Checked::Invalid => Checked::Invalid,
        }
    }
}

impl<T: Copy> Copy for Checked<T> {}

impl<T: Default> Default for Checked<T> {
    fn default() -> Self {
        Checked::Valid(T::default())
    }
}

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

impl fmt::Display for JsonPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Validate for Checked<T> {
    fn validate<P, R>(&self, _root: &Root, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        match self {
            &Checked::Valid(_) => {},
            &Checked::Invalid => report(&path, Error::Invalid),
        }
    }
}

impl<K: Eq + Hash + ToString + Validate, V: Validate> Validate for HashMap<K, V> {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        for (key, value) in self.iter() {
            key.validate(root, || path().key(&key.to_string()), report);
            value.validate(root, || path().key(&key.to_string()), report);
        }
    }
}

impl<T: Validate> Validate for Option<T> {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        if let Some(value) = self.as_ref() {
            value.validate(root, path, report);
        }
    }
}

impl<T: Validate> Validate for Vec<T> {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        for (index, value) in self.iter().enumerate() {
            value.validate(root, || path().index(index), report);
        }
    }
}

impl Validate for bool {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        // nop
    }
}

impl Validate for u32 {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        // nop
    }
}

impl Validate for i32 {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        // nop
    }
}

impl Validate for f32 {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        // nop
    }
}

impl Validate for [f32; 3] {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        // nop
    }
}

impl Validate for [f32; 4] {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        // nop
    }
}

impl Validate for [f32; 16] {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        // nop
    }
}

impl Validate for () {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        // nop
    }
}

impl Validate for String {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        // nop
    }
}

impl Validate for serde_json::Value {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
        where P: Fn() -> JsonPath, R: FnMut(&Fn() -> JsonPath, Error)
    {
        // nop
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::IndexOutOfBounds => "Index out of bounds",
            &Error::Invalid => "Invalid value",
            &Error::Missing => "Missing data",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
}
