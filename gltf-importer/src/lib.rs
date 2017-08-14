
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The reference loader implementation for the `gltf` crate.
//!
//! # Examples 
//!
//! ### Importing some `glTF` 2.0
//!
//! ```rust
//! use gltf_importer::import;
//! # #[allow(unused_variables)]
//! let path = "path/to/asset.gltf";
//! # let path = "../examples/Box.gltf";
//! match import(path) {
//!     Ok((gltf, _)) => println!("{:#?}", gltf.as_json()),
//!     Err(err) => println!("error: {:?}", err),
//! }
//! ```

extern crate gltf;
extern crate gltf_utils;

use gltf::json::{self, validation};
use std::{fmt, fs, io};

use gltf::Gltf;
use gltf_utils::Source;
use std::error::Error as StdError;
use std::path::Path;

/// Contains parameters for import configuration.
pub mod config;

pub use self::config::Config;

/// Error encountered when importing a glTF 2.0 asset.
#[derive(Debug)]
pub enum Error {
    /// A loaded glTF buffer is not of the required length.
    BufferLength(json::Path),

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

/// Buffer data returned from `import`.
#[derive(Clone, Debug)]
pub struct Buffers(Vec<Vec<u8>>);

impl Source for Buffers {
    fn source_buffer(&self, buffer: &gltf::Buffer) -> &[u8] {
        &self.0[buffer.index()]
    }
}

impl Buffers {
    /// Take the loaded buffer data.
    pub fn take(self) -> Vec<Vec<u8>> {
        self.0
    }
}

fn import_impl(path: &Path, config: Config) -> Result<(Gltf, Buffers), Error> {
    let data = read_to_end(path)?;
    if data.starts_with(b"glTF") {
        import_binary(&data, &config, path)
    } else {
        import_standard(&data, &config, path)
    }
}   

/// Imports glTF 2.0
pub fn import<P>(path: P) -> Result<(Gltf, Buffers), Error>
    where P: AsRef<Path>
{
    import_impl(path.as_ref(), Default::default())
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
    gltf: &Gltf,
    has_blob: bool,
) -> Result<Vec<Vec<u8>>, Error> {
    let mut iter = gltf.as_json().buffers.iter().enumerate();
    if has_blob {
        let _ = iter.next();
    }
    iter
        .map(|(index, buffer)| {
            let uri = buffer.uri.as_ref().unwrap();
            let path = base_path.parent().unwrap_or(Path::new("./")).join(uri);
            let data = read_to_end(&path)?;
            if data.len() != buffer.byte_length as usize {
                let path = json::Path::new().field("buffers").index(index);
                return Err(Error::BufferLength(path));
            }
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
) -> Result<(Gltf, Buffers), Error> {
    let json: gltf::json::Root = gltf::json::from_slice(data)?;
    let _ = validate_standard(&json, &config);
    let gltf = Gltf::from_json(json);
    let has_blob = false;
    let mut buffers = Buffers(vec![]);
    for buffer in load_external_buffers(base_path, &gltf, has_blob)? {
        buffers.0.push(buffer);
    }
    Ok((gltf, buffers))
}

fn import_binary<'a>(
    data: &'a [u8],
    config: &Config,
    base_path: &Path,
) -> Result<(Gltf, Buffers), Error> {
    let glb = Glb(data);
    let (_, json_chunk, blob_chunk) = glb.split()?;
    let begin = json_chunk.offset;
    let end = begin + json_chunk.length;
    let json: gltf::json::Root = gltf::json::from_slice(&glb.0[begin..end])?;
    let blob = blob_chunk.map(|chunk| glb.slice(chunk).to_vec());
    let has_blob = blob.is_some();
    let _ = validate_binary(&json, &config, has_blob)?;
    let gltf = Gltf::from_json(json);
    let mut buffers = Buffers(vec![]);
    if let Some(buffer) = blob {
        buffers.0.push(buffer);
    }
    for buffer in load_external_buffers(base_path, &gltf, has_blob)? {
        buffers.0.push(buffer);
    }
    Ok((gltf, buffers))
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
        match *self {
            BufferLength(_) => "Loaded buffer does not match required length",
            ExtensionDisabled(_) => "Asset requires a disabled extension",
            ExtensionUnsupported(_) => "Assets requires an unsupported extension",
            IncompatibleVersion(_) => "Asset is not glTF version 2.0",
            Io(_) => "I/O error",
            MalformedGlb(_) => "Malformed .glb file",
            MalformedJson(_) => "Malformed .gltf / .glb JSON",
            Validation(_) => "Asset failed validation tests",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        use self::Error::*;
        match *self {
            MalformedJson(ref err) => Some(err),
            Io(ref err) => Some(err),
            _ => None,
        }
    }
}
