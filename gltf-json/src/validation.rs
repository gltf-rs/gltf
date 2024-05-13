use serde::{ser, Serialize, Serializer};
use std::collections::BTreeMap;
use std::hash::Hash;

use crate::{Path, Root};

/// Trait for validating glTF JSON data so that the library can function without panicking.
pub trait Validate {
    /// Validates the invariants required for the library to function safely.
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        // nop
    }
}

/// Specifies what kind of error occured during validation.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// An index was found to be out of bounds.
    IndexOutOfBounds,

    /// An invalid value was identified.
    Invalid,

    /// Some required data has been omitted.
    Missing,

    /// A memory size or offset exceeds the system limits.
    Oversize,

    /// One of more required extensions is not supported by this crate version.
    Unsupported,
}

/// Specifies a type that has been pre-validated during deserialization or otherwise.
#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Checked<T> {
    /// The item is valid.
    Valid(T),

    /// The item is invalid.
    Invalid,
}

impl<T> Checked<T> {
    /// Converts from `Checked<T>` to `Checked<&T>`.
    pub fn as_ref(&self) -> Checked<&T> {
        match *self {
            Checked::Valid(ref item) => Checked::Valid(item),
            Checked::Invalid => Checked::Invalid,
        }
    }

    /// Takes ownership of the contained item if it is `Valid`.
    ///
    /// # Panics
    ///
    /// Panics if called on an `Invalid` item.
    pub fn unwrap(self) -> T {
        match self {
            Checked::Valid(item) => item,
            Checked::Invalid => panic!("attempted to unwrap an invalid item"),
        }
    }
}

impl<T: Serialize> Serialize for Checked<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Checked::Valid(ref item) => item.serialize(serializer),
            Checked::Invalid => Err(ser::Error::custom("invalid item")),
        }
    }
}

impl<T: Clone> Clone for Checked<T> {
    fn clone(&self) -> Self {
        match *self {
            Checked::Valid(ref item) => Checked::Valid(item.clone()),
            Checked::Invalid => Checked::Invalid,
        }
    }
}

impl<T: Copy> Copy for Checked<T> {}

impl<T: Default> Default for Checked<T> {
    fn default() -> Self {
        Checked::Valid(T::default())
    }
}

impl<T> Validate for Checked<T> {
    fn validate<P, R>(&self, _root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        match *self {
            Checked::Valid(_) => {}
            Checked::Invalid => report(&path, Error::Invalid),
        }
    }
}

/// Validates the suitability of 64-bit byte offsets/sizes on 32-bit systems.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
    serde_derive::Deserialize,
    serde_derive::Serialize,
)]
pub struct USize64(pub u64);

impl From<u64> for USize64 {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<usize> for USize64 {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl Validate for USize64 {
    fn validate<P, R>(&self, _root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        if usize::try_from(self.0).is_err() {
            report(&path, Error::Oversize);
        }
    }
}

impl<K: ToString + Validate, V: Validate> Validate for BTreeMap<K, V> {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        for (key, value) in self.iter() {
            key.validate(root, || path().key(&key.to_string()), report);
            value.validate(root, || path().key(&key.to_string()), report);
        }
    }
}

impl Validate for serde_json::Map<String, serde_json::Value> {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        for (key, value) in self.iter() {
            key.validate(root, || path().key(&key.to_string()), report);
            value.validate(root, || path().key(&key.to_string()), report);
        }
    }
}

impl<T: Validate> Validate for Option<T> {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        if let Some(value) = self.as_ref() {
            value.validate(root, path, report);
        }
    }
}

impl<T: Validate> Validate for Vec<T> {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        for (index, value) in self.iter().enumerate() {
            value.validate(root, || path().index(index), report);
        }
    }
}

impl Validate for std::boxed::Box<serde_json::value::RawValue> {
    fn validate<P, R>(&self, _: &Root, _: P, _: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        // nop
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Error::IndexOutOfBounds => "Index out of bounds",
                Error::Invalid => "Invalid value",
                Error::Missing => "Missing data",
                Error::Oversize => "Size exceeds system limits",
                Error::Unsupported => "Unsupported extension",
            }
        )
    }
}

// These types are assumed to be always valid.
impl Validate for bool {}
impl Validate for u32 {}
impl Validate for i32 {}
impl Validate for f32 {}
impl Validate for [f32; 3] {}
impl Validate for [f32; 4] {}
impl Validate for [f32; 16] {}
impl Validate for () {}
impl Validate for String {}
impl Validate for serde_json::Value {}
