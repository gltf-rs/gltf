
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//!
//! glTF JSON, buffers, and images may come from a range of external sources, so
//! customization is an important design goal of the import module. The `Source`
//! trait is provided to facilitate customization of the data loading process.
//!
//! For convenience, the library contains one implementation of the `Source` trait,
//! namely `FromPath`, which allows loading from file system and also from embedded
//! base64 encoded data. This implementation may be used as reference for other
//! schemes such as `http`.

extern crate gltf;

use gltf::json::{self, validation};
use std::{fmt, fs, io};

use gltf::{Gltf, Loaded};
use std::error::Error as StdError;
use std::path::{Path, PathBuf};

/// Contains data structures for import configuration.
pub mod config;

pub use self::config::Config;

/// Error encountered when importing a glTF 2.0 asset.
#[derive(Debug)]
pub enum Error {
    /// A glTF extension required by the asset has not been enabled by the user.
    ExtensionDisabled(String),

    /// A glTF extension required by the asset is not supported by the library.
    ExtensionUnsupported(String),

    /// The glTF version of the asset is incompatible with the importer.
    IncompatibleVersion(String),

    /// Standard I/O error.
    Io(std::io::Error),

    /// Failure when parsing a .glb file.
    MalformedGlb(String),

    /// Failure when deserializing .gltf or .glb JSON.
    MalformedJson(json::Error),

    /// The .gltf data is invalid.
    Validation(Vec<(json::Path, validation::Error)>),
}

/// A `Future` that drives the importation of glTF.
#[derive(Debug)]
pub struct Importer {
    path: PathBuf,
    data: Vec<u8>,
    gltf: Option<Gltf>,
    buffers: Vec<Vec<u8>>,
    config: Config,
}

impl gltf::Source for Importer {
    fn source_buffer(&self, buffer: &gltf::Buffer) -> &[u8] {
        &self.buffers[buffer.index()]
    }
}

/// The contents of a .glb file.
#[derive(Clone, Debug)]
struct Glb<'a>(&'a [u8]);

/// The header section of a .glb file.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Header {
    /// Must be `b"glTF"`.
    magic: [u8; 4],

    /// Must be `2`.
    version: u32,

    /// Must match the length of the parent .glb file.
    length: u32,
}

/// The header of the JSON or BIN section of a .glb file.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct ChunkInfo {
    /// The length of the chunk data in byte excluding the header.
    length: u32,

    /// Either `b"JSON"` or `b"BIN\0"`.
    kind: [u8; 4],

    // Data follows... 
}

/// Represents a subset of `Importer::glb`.
#[derive(Copy, Clone, Debug, Default)]
struct Slice {
    /// Offset into `glb` in bytes.
    offset: usize,

    /// Length of the slice in bytes.
    length: usize,
}

/// The header, JSON, and BIN sections of a .glb file, respectively.
type Split = (Header, Slice, Option<Slice>);

impl<'a> Glb<'a> {
    /// Obtains a slice of the GLB data.
    fn slice(&self, slice: Slice) -> &[u8] {
        let begin = slice.offset;
        let end = begin + slice.length;
        &self.0[begin..end]
    }

    /// Splits loaded GLB into its three chunks.
    ///
    /// * Mandatory GLB header.
    /// * Mandatory JSON chunk.
    /// * Optional BIN chunk.
    fn split(&self) -> Result<Split, Error> {
        use std::mem::{size_of, transmute};
        let err = |reason: &str| Err(Error::MalformedGlb(reason.to_string()));
        let glb = &self.0;
        let glb_ptr = glb.as_ptr();
        if glb.len() < size_of::<Header>() + size_of::<ChunkInfo>() {
            return err("GLB missing header");
        }

        // Offset in bytes into `glb`.
        let mut offset = 0isize;

        let header = unsafe {
            let x: *const Header = transmute(glb_ptr.offset(offset));
            if &(*x).magic != b"glTF" {
                return err("GLB does not start with `glTF`");
            }
            if (*x).length as usize != glb.len() {
                return err("length does not match length of data");
            }
            if (*x).version != 2 {
                return err("Not GLB version 2.0");
            }
            *x
        };

        offset += size_of::<Header>() as isize;
        let json_chunk = unsafe {
            let x: *const ChunkInfo = transmute(glb_ptr.offset(offset));
            offset += size_of::<ChunkInfo>() as isize;
            if (*x).length as usize > (glb.len() as isize - offset) as usize {
                return err("JSON chunkLength exceeds length of data");
            }
            if &(*x).kind != b"JSON" {
                return err("JSON chunkType is not `JSON`");
            }
            Slice {
                offset: offset as usize,
                length: (*x).length as usize,
            }
        };

        offset += json_chunk.length as isize;
        let blob_chunk = if glb.len() as isize - offset == 0 {
            None
        } else {
            unsafe {
                let x: *const ChunkInfo = transmute(glb_ptr.offset(offset));
                offset += size_of::<ChunkInfo>() as isize;
                if (*x).length as usize > (glb.len() as isize - offset) as usize {
                    return err("BIN chunkLength exceeds length of data");
                }
                if &(*x).kind != b"BIN\0" {
                    return err("BIN chunkType is not `BIN\0`");
                }
                Some(Slice {
                    offset: offset as usize,
                    length: (*x).length as usize,
                })
            }
        };

        Ok((header, json_chunk, blob_chunk))
    }
}

fn read_to_end_impl(path: &Path) -> Result<Vec<u8>, Error> {
    use io::Read;
    let file = fs::File::open(path)?;
    let mut reader = io::BufReader::new(file);
    let mut buffer = vec![];
    let _ = reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn read_to_end<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    read_to_end_impl(path.as_ref())
}

fn load_external_buffers(
    base_path: &Path,
    root: &gltf::root::Root,
    is_glb: bool,
) -> Result<Vec<Vec<u8>>, Error> {
    let mut iter = root.as_json().buffers.iter();
    if is_glb {
        let _ = iter.next();
    }
    iter
        .map(|buffer| {
            let uri = buffer.uri.as_ref().unwrap();
            let path = base_path.parent().unwrap_or(Path::new("./")).join(uri);
            let data = read_to_end(&path)?;
            Ok(data)
        })
        .collect()
}

fn validate_standard(json: &gltf::json::Root, config: &Config) -> Result<(), Error> {
    use config::ValidationStrategy;
    use gltf::json::validation::Validate;
    match config.validation_strategy {
        ValidationStrategy::Skip => Ok(()),
        ValidationStrategy::Minimal => {
            let mut errs = vec![];
            json.validate_minimally(
                &json,
                || json::Path::new(),
                &mut |path, err| errs.push((path(), err)),
            );
            if errs.is_empty() {
                Ok(())
            } else {
                Err(Error::Validation(errs))
            }
        },
        ValidationStrategy::Complete => {
            let mut errs = vec![];
            json.validate_completely(
                &json,
                || json::Path::new(),
                &mut |path, err| errs.push((path(), err)),
            );
            if errs.is_empty() {
                Ok(())
            } else {
                Err(Error::Validation(errs))
            }
        },
    }
}

fn validate_binary(
    json: &gltf::json::Root,
    config: &Config,
    has_blob: bool,
) -> Result<(), Error> {
    use self::json::validation::Error as Reason;
    let mut errs = vec![];
    let _ = validate_standard(json, config)?;
    if config.validation_strategy == config::ValidationStrategy::Skip {
        return Ok(());
    }

    // Required for the `Minimal` and `Complete` validation strategies.
    for (index, buffer) in json.buffers.iter().enumerate() {
        let path = || {
            json::Path::new()
                .field("buffers")
                .index(index)
                .field("uri")
        };
        match index {
            0 if has_blob => if buffer.uri.is_some() {
                errs.push((path(), Reason::Missing));
            },
            _ if buffer.uri.is_none() => {
                errs.push((path(), Reason::Missing));
            },
            _ => {},
        }
    }

    if errs.is_empty() {
        Ok(())
    } else {
        Err(Error::Validation(errs))
    }
}

fn import_standard<'a>(
    data: &'a [u8],
    config: &Config,
    base_path: &Path,
) -> Result<(Gltf, Vec<Vec<u8>>), Error> {
    let json: gltf::json::Root = gltf::json::from_slice(data)?;
    let _ = validate_standard(&json, &config);
    let root = gltf::root::Root::new(json);
    let is_glb = false;
    let buffers = load_external_buffers(base_path, &root, is_glb)?;
    Ok((Gltf::new(root), buffers))
}

fn import_binary<'a>(
    data: &'a [u8],
    config: &Config,
    base_path: &Path,
) -> Result<(Gltf, Vec<Vec<u8>>), Error> {
    let glb = Glb(data);
    let (_, json_chunk, blob_chunk) = glb.split()?;
    let begin = json_chunk.offset;
    let end = begin + json_chunk.length;
    let json: gltf::json::Root = gltf::json::from_slice(&glb.0[begin..end])?;
    let blob = blob_chunk.map(|chunk| glb.slice(chunk).to_vec());
    let has_blob = blob.is_some();
    let _ = validate_binary(&json, &config, has_blob)?;
    let root = gltf::root::Root::new(json);
    let mut buffers = vec![];
    if let Some(buffer) = blob {
        buffers.push(buffer);
    }
    let is_glb = true;
    for buffer in load_external_buffers(base_path, &root, is_glb)? {
        buffers.push(buffer);
    }
    Ok((Gltf::new(root), buffers))
}

impl Importer {
    /// Constructs an `Importer`.
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path: path.into(),
            data: vec![],
            gltf: None,
            buffers: vec![],
            config: Default::default(),
        }
    }

    /// Imports the `glTF`.
    pub fn import(&mut self) -> Result<Loaded<&Gltf>, Error> {
        self.data = read_to_end(&self.path)?;
        let (gltf, buffers) = if self.data.starts_with(b"glTF") {
            import_binary(&self.data, &self.config, &self.path)
        } else {
            import_standard(&self.data, &self.config, &self.path)
        }?;
        self.buffers = buffers;
        self.gltf = Some(gltf);
        Ok(self.gltf.as_ref().unwrap().loaded(self))
    }

    /// Returns the path to the glTF.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Error {
        Error::MalformedJson(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<Vec<(json::Path, validation::Error)>> for Error {
    fn from(errs: Vec<(json::Path, validation::Error)>) -> Error {
        Error::Validation(errs)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match self {
            &ExtensionDisabled(_) => "Asset requires a disabled extension",
            &ExtensionUnsupported(_) => "Assets requires an unsupported extension",
            &IncompatibleVersion(_) => "Asset is not glTF version 2.0",
            &Io(_) => "I/O error",
            &MalformedGlb(_) => "Malformed .glb file",
            &MalformedJson(_) => "Malformed .gltf / .glb JSON",
            &Validation(_) => "Asset failed validation tests",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        use self::Error::*;
        match self {
            &MalformedJson(ref err) => Some(err),
            &Io(ref err) => Some(err),
            _ => None,
        }
    }
}
