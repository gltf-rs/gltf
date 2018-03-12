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
//!     Ok((gltf, _)) => println!("{:#?}", doc.as_json()),
//!     Err(err) => println!("error: {:?}", err),
//! }
//! ```

use base64;
use json::{self, validation};
use std::{fmt, fs, io, path};

use buffer;
use {Error, Gltf, Buffer, Unvalidated};
use util::Source;
use std::error::Error as StdError;
use std::path::Path;
use std::io::Read;

/// Contains parameters for import configuration.
pub mod config;

pub use self::config::Config;
pub use self::config::ValidationStrategy;

/// Buffer data returned from `import`.
#[derive(Clone, Debug)]
pub struct Buffers(Vec<Vec<u8>>);

impl Source for Buffers {
    fn source_buffer(&self, buffer: &Buffer) -> &[u8] {
        &self.0[buffer.index()]
    }
}

impl Buffers {
    /// Obtain the contents of a loaded buffer.
    pub fn buffer(&self, buffer: &Buffer) -> Option<&[u8]> {
        self.0.get(buffer.index()).map(Vec::as_slice)
    }

    /// Obtain the contents of a loaded buffer view.
    pub fn view(&self, view: &buffer::View) -> Option<&[u8]> {
        self.buffer(&view.buffer()).map(|data| {
            let begin = view.offset();
            let end = begin + view.length();
            &data[begin..end]
        })
    }

    /// Take the loaded buffer data.
    pub fn take(self) -> Vec<Vec<u8>> {
        self.0
    }
}

fn import_impl(path: &Path, config: &Config) -> Result<(Gltf, Buffers), Error> {
    let data = read_to_end(path)?;
    import_data_slice(&data, path, config)
}

/// Imports glTF 2.0 from a slice with custom configuration.
pub fn import_data_slice<'a, P>(
    data: &'a [u8],
    path: P,
    config: &Config,
) -> Result<(Gltf, Buffers), Error>
where
    P: AsRef<Path>,
{
    if data.starts_with(b"glTF") {
        import_binary(&data, config, path.as_ref())
    } else {
        import_standard(&data, config, path.as_ref())
    }
}

/// Imports glTF 2.0 with custom configuration.
pub fn import_with_config<P>(path: P, config: &Config) -> Result<(Gltf, Buffers), Error>
where
    P: AsRef<Path>,
{
    import_impl(path.as_ref(), config)
}

/// Imports glTF 2.0 with default configuration.
pub fn import<P>(path: P) -> Result<(Gltf, Buffers), Error>
where
    P: AsRef<Path>,
{
    import_impl(path.as_ref(), &Default::default())
}

fn read_to_end_impl(path: &Path) -> Result<Vec<u8>, Error> {
    if path.exists() {
        let file = fs::File::open(path)?;
        let mut reader = io::BufReader::new(file);
        let mut buffer = vec![];
        let _ = reader.read_to_end(&mut buffer)?;
        Ok(buffer)
    } else {
        Err(Error::FileNotFound(path.to_path_buf()))
    }
}

fn read_to_end<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    read_to_end_impl(path.as_ref())
}

fn parse_data_uri(uri: &str) -> Result<Vec<u8>, Error> {
    let encoded = uri.split(',').nth(1).unwrap();
    let decoded = base64::decode(&encoded)?;
    Ok(decoded)
}

fn load_external_buffers(
    base_path: &Path,
    doc: &Document,
    mut bin: Option<Vec<u8>>,
) -> Result<Vec<Vec<u8>>, Error> {
    let mut buffers = vec![];
    for (index, buffer) in doc.buffers().enumerate() {
        let uri = buffer.uri();
        let data = if uri == "#bin" {
            Ok(bin.take().unwrap())
        } else if uri.starts_with("data:") {
            Ok(parse_data_uri(uri)?)
        } else {
            let path = base_path
                .parent()
                .unwrap_or_else(|| Path::new("./"))
                .join(uri);
            read_to_end(&path)
        }?;
        if data.len() < buffer.length() {
            let path = json::Path::new().field("buffers").index(index);
            return Err(Error::BufferLength(path));
        }
        buffers.push(data);
    }
    Ok(buffers)
}

fn validate_standard(unvalidated: Unvalidated, config: &Config) -> Result<Gltf, Error> {
    use self::config::ValidationStrategy;
    Ok(match config.validation_strategy {
        ValidationStrategy::Skip => unvalidated.skip_validation(),
        ValidationStrategy::Minimal => unvalidated.validate_minimally()?,
        ValidationStrategy::Complete => unvalidated.validate_completely()?,
    })
}

fn validate_binary(
    unvalidated: Unvalidated,
    config: &Config,
    has_bin: bool,
) -> Result<Gltf, Error> {
    use self::config::ValidationStrategy;
    use json::validation::Error as Reason;

    if config.validation_strategy == ValidationStrategy::Skip {
        return Ok(unvalidated.skip_validation());
    }

    let mut errs = vec![];
    {
        let json = unvalidated.as_json();
        for (index, buffer) in json.buffers.iter().enumerate() {
            let path = || json::Path::new().field("buffers").index(index).field("uri");
            match index {
                0 if has_bin => if buffer.uri.is_some() {
                    errs.push((path(), Reason::Missing));
                },
                _ if buffer.uri.is_none() => {
                    errs.push((path(), Reason::Missing));
                }
                _ => {}
            }
        }
    }

    if errs.is_empty() {
        Ok(match config.validation_strategy {
            ValidationStrategy::Minimal => unvalidated.validate_minimally()?,
            ValidationStrategy::Complete => unvalidated.validate_completely()?,
            ValidationStrategy::Skip => unreachable!(),
        })
    } else {
        Err(Error::Validation(errs))
    }
}

fn import_standard<'a>(
    data: &'a [u8],
    config: &Config,
    base_path: &Path,
) -> Result<(Gltf, Buffers), Error> {
    let unvalidated = Gltf::from_slice(data)?;
    let gltf = validate_standard(unvalidated, config)?;
    let bin = None;
    let mut buffers = Buffers(vec![]);
    for buffer in load_external_buffers(base_path, &gltf, bin)? {
        buffers.0.push(buffer);
    }
    Ok((gltf, buffers))
}

fn import_binary<'a>(
    data: &'a [u8],
    config: &Config,
    base_path: &Path,
) -> Result<(Gltf, Buffers), Error> {
    let gltf::Glb { json, bin, .. } = Binary::from_slice(data)?;
    let unvalidated = Gltf::from_slice(&json)?;
    let bin = bin.map(|x| x.into_owned());
    let gltf = validate_binary(unvalidated, config, bin.is_some())?;
    let mut buffers = Buffers(vec![]);
    for buffer in load_external_buffers(base_path, &gltf, bin)? {
        buffers.0.push(buffer);
    }
    Ok((gltf, buffers))
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Error {
        Error::Base64Decoding(err)
    }
}
