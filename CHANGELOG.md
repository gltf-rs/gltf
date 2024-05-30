# Changelog

Notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).

The top-level `gltf` crate adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html). Historically, semantic versioning has not applied to `gltf-json`. This is planned to change from version 2—see issue #409.

## Unreleased

## [1.4.1] - 2024-05-09

### Added

- New functions `animation::Channel::index` and `animation::Sampler::index`.
- New feature flag `allow_empty_texture` to avoid required extension checks.

### Fixed

- Fix `attempt to to subtract with overflow`-panic in `size_hint()` of sparse accessor when collecting items.
- Fix incorrect values returned from `size_hint()` in sparse accessor.
- Add support to read items from sparse accessor without base buffer view.

### Changed

- Update `image` to `0.25.0`.
- Validation will now fail if a glTF document requires extensions that are not supported by the crate.

### Removed

- Feature `image_jpeg_rayon` no longer needed, as `image 0.25.0` now uses `zune-jpeg` for jpeg decoding.

## [1.4.0] - 2023-12-17

### Added

- New API for reading arbitary extension data.
- Interval improvements to prevent panics on 32 bit systems when loading large glTF files.

### Changed

- Offsets and sizes in the `gltf-json` crate have been widened to 64 bits.
- Loading glTF on a 32 bit system containing offsets or sizes larger than `u32` will
  result in a validation error.

### Fixed

- `Gltf::from_reader` no longer winds the reader back to offset zero.
- Broken link in `Material::unlit` documentation.

## [1.3.0] - 2023-08-21

### Added

- Support for the `KHR_materials_emissive_strength` extension.

### Fixed

- `Accessor::byte_offset` is now optional for sparse accessors.

## [1.2.0] - 2023-06-08

### Added

- New functions `import_buffers` and `import_images` for advanced import use cases.

### Changed

- `Primitive::attributes` now uses `BTreeMap` instead of `HashMap` deterministic serialization.

## [1.1.0] - 2023-02-01

### Fixed

- Fix relative file path imports with url encoded characters.
- Fix bounds calculation in export example.

## [1.0.0] - 2022-01-29

### Added

- Support for the `KHR_materials_specular` extension.
- Support for the `KHR_materials_variants` extension.
- Support for the `KHR_materials_volume` extension.
- `ExactSizeIterator` implementation for `Joints` iterator.

### Changed

- The `mesh.primitives` property is now always serialized.

### Fixed

- Incorrect implementation of `Normalize<u16>` and `Normalize<f32>` for `u16`.

## [0.16.0] - 2021-05-13

### Added

- Support for the `KHR_texture_transform` extension.
- Support for the `KHR_materials_transmission_ior` extension.

### Changed

- `Material::alpha_cutoff` is now optional.

### Fixed

- URIs with embedded data failing to import when using `import_slice`.
- Serialization of empty primitives object being skipped.

## [0.15.2] - 2020-03-29

### Changed

- All features are now exposed in the [online documentation](http://docs.rs/gltf).
- Primary iterators now implement `Iterator::nth` explicitly for improved performance.

### Fixed

- Compiler warnings regarding deprecation of `std::error::Error::description`.

## [0.15.1] - 2020-03-15

### Added

- New feature `guess_mime_type` which, as the name suggests, attempts to guess
  the MIME type of an image if it doesn't exactly match the standard.

### Changed

- `base64` updated to `0.11`.
- `byteorder` updated to `1.3`.
- `image` updated to `0.23.0`.
- `Format` has additional variants for 16-bit pixel formats.

### Fixed

- Off-by-one error when reading whole files incurring a gratuitous reallocation.

## [0.15.0] - 2020-01-18

### Added

- Support for the `KHR_materials_unlit` extension, which adds an `unlit` field
  to `Material`.
- `ExactSizeIterator` implementations for accessor iterators.

### Fixed

- Some lifetimes for accessing fields (e.g. `Node::children`) have been relaxed.
- `Accessor::buffer_view` is now optional for the benefit of sparse accessors.

### Removed

- `animation::Interpolation::CatmullRomSpline`

## [0.14.0] - 2019-10-06

### Added

- `import_slice` function.
- `Clone` and `Debug` implementations for `Material`.
- `accessor::util::SparseIter` and `accessor::util::SparseIndicesIter` which
  iterate over items and indices in sparse accessors respectively.

### Changed

- `accessor::util::Iter` is now an enum; sparse accessors are now read correctly.
- The original version of `accessor::util::Iter` has been renamed as `ItemIter`.

## [0.13.0] - 2019-07-27

### Added

- Support for the `KHR_lights_punctual` extension.
- `gltf-roundtrip` example.

### Changed

- Renamed `validate_minimally` as `validate`.
- Removed dependency on `cgmath`.

### Fixed

- Incorrect version in README instructions.
- Compiler warnings.

### Removed

- Trait function `Validate::validate_completely` and its implementations.

## [0.12.0] - 2019-04-21

### Added

- New image format variants `B8G8R8` and `B8G8R8A8`.
- New export example.

### Changed

- The crate now builds with Rust 2018 edition.
- `Extras` are now exposed as `RawValue`.
- `Index` now implements `Copy`.
- Meshes will no longer report a zero byte stride.
- Updated the following dependendies:
  - approx
  - base64
  - cgmath
  - image
  - lazy_static
  - proc_macro2
  - quote
  - syn

### Fixed

- Removed an unused field in `Accessor` which was a cause of poor performance.
- Borrow checker complaint regarding `Glb::from_reader`.

## [0.11.3] - 2019-02-21

### Added

- New function `gltf_json::Index::new` for export.
- Support for the `KHR_materials_pbrSpecularGlossiness` extension.
- Extra `Eq` and `PartialEq` implementations.

### Fixed

- `Accessor::min` is no longer serialized if `None`.
- `Extras::_allow_unknown_fields` is no longer serialized.
- `bufferView.byteOffset` is now optional.
- Node translation/rotation/scale values are now optional.
- Removed serialization of some default values.
- `extras::Void` is no longer serialized.

## [0.11.2] - 2018-07-08

### Fixed

- JSON chunk padding is now padded with spaces (0x20) rather than zeros.
- `None` is no longer serialized.
- `Vec` is no longer serialized when empty.

## [0.11.1] - 2018-06-10

### Added

- `payload` field in `Gltf` in order to handle binary glTF directly.
- `Error::Io` enum variant.
- `Gltf::from_reader_without_validation` and `Gltf::from_slice_without_validation`.
- All functionality from the `gltf-utils` crate, feature gated with the new
  `utils` feature.
- Most functionality from the `gltf-importer` crate, feature gated with the
  new `import` feature.
- `enum Uri` to represent uniform resource locators.

### Changed

- `fn Gltf::from_*` now imports binary glTF as well as standard glTF.
- `fn Gltf::from_reader` now requires `reader` to implement `std::io::Seek`.
- `Buffer::uri` now returns `None` in the case of binary glTF payload instead
  of the magic string `"#bin"`.
- The `POSITION` attribute is now required by all mesh primitives.
- Several renames:
  - `glb` → `binary`.
  - `Error::Glb` → `Error::Binary`.
  - `TrsProperty` → `Property`.
  - `InterpolationAlgorithm` → `Interpolation`.
  - `Target::path` → `Target::property`.
  - `Primitive::position_bounds` → `Primitive::bounding_box`.
- The `names` feature is now enabled by default, along with `utils` and
  `import`. Rationale: Pareto principle.

### Fixed

- Data structures in `gltf_json` now implement `Serialize`.

### Removed

- `fn Gltf::from_str` -- use `fn Gltf::from_slice` instead.
- `fn Gltf::from_value` -- no longer supported.
- `fn gltf::is_binary` -- use `slice.starts_with("glTF")` instead.
- `struct Unvalidated` -- replaced with `enum Validation`.
- `crate gltf-importer` -- no longer supported.
- `Node::matrix` -- use `transform().matrix()` instead.
- `Node::rotation` -- use `transform().decomposed()` instead.
- All hidden `as_json` functions -- no longer supported.

## [0.11.0] - 2018-06-10

### Yanked

## [0.10.1] - 2018-03-05

### Fixed

- `gltf_utils::AccessorIter::new` is marked `pub` again.

## [0.10.0] - 2018-03-04

### Added

- `ChannelIterators`.

### Changed

- Rework of GLB parser and `AccessorIter` in order to reduce `unsafe` code and
  to allow loading of binary glTF data on non-little-endian machines.
- More descriptive errors in the `gltf` and `gltf_importer` crates.
- `Gltf::skip_validation` is no longer marked `unsafe`.

## [0.9.3] - 2017-10-28

### Fixed

- `gltf_utils::PrimitiveIterators::joints_u16` implementation.

## [0.9.2] - 2017-09-10

### Fixed

- Incorrect implementation of Transform::decomposed (issue #99)

## [0.9.1] - 2017-09-05

### Added

- `scene::Transform` type, returned by `Node::transform`.
- `MorphTarget` type and a corresponding `MorphTargets` iterator.
- `Gltf::from_glb` function.
- `Gltf::default_scene` function.

### Changed

- `as_json` functions are now hidden from the documentation.
- `scene::Children` is bound to the lifetime of `Gltf` instead its parent `Node`
- Documentation has been improved across the crate.

### Removed

- `root` module.
- Default constructors for `Material` and `texture::Sampler`.

### Deprecated

- `Node::matrix/translation/rotation/scale` in favour of `Node::transform`.
- `json` module.

## [0.9.0] - 2017-08-27

### Added

- New `gltf-utils` crate, which removes the need for the `Loaded` wrapper struct.
- New `Attributes` iterator, which visits all vertex attributes of a `Primitive`.
- New `glb` module, containing functions for parsing .glb (binary glTF) files.
- New `is_binary` function, which tests for the glTF magic string.
- New 'explicit' validation strategy with the `gltf::Unvalidated` type.
- New `Bounds` type describing the minimum and maximum values of accessors.
- New `Accessor::position_bounds` convenience function.
- `Info`, `NormalTexture`, and `OcclusionTexture` now implement `AsRef<Texture>`.

### Removed

- Removed the `Loaded` struct in favour of new traits in the `gltf-utils` crate.

## [0.8.6] - 2017-08-26

### Added

- Implemented `ExactSizeIterator` for primitive iterators.

## [0.8.5] - 2017-08-11

### Changed

- Fixed incorrect buffer view slicing from accessors.
- Wrapper struct constructors now use `pub(crate)` instead of `#[doc(hidden)]`.
- Hence the crate now requires `rustc` version 1.18 or above to build.

## [0.8.4] - 2017-08-10

### Added

- Added the `gltf-tree` example which displays the node heirarchies of scenes.

## [0.8.3] - 2017-08-10

### Changed

- Fixed mismatched `names` and `extras` feature flags.

## [0.8.2] - 2017-08-09

### Removed

- Removed broken link to `gltf-importer` in the crate documentation.

## [0.8.1] - 2017-08-09

### Changed

- No concrete changes (unsuccessful update to documentation).

## [0.8.0]- 2017-08-09

### Added

- New `Loaded` type, which provides `glTF` objects with buffer data.
- New `gltf-json` crate which replaces the `json` module. This is intended to
  improve build times.
- Exported `names` and `extras` features from the new `gltf-json` crate.

### Changed

- Redesign of the `Source` trait so that it no longer performs I/O.

## [0.7.0]- 2017-07-28

### Added

- `Image::data` now returns a `DynamicImage`, allowing for post-processing.

## [0.6.1]- 2017-07-15

## Changed

- `Send` and `Sync` are no longer required by the `Source` trait.

## [0.6.0]- 2017-07-15

### Added

- New wrapper interface.
- `Source` trait, which allows for customizing the import process.
- Reference `Source` trait implementation, namely `FromPath`, that can
  read from the file system and decode embedded base64 data URIs.
- Support for binary glTF.
- `Validate` trait, which validates glTF JSON metadata.
- `Import` struct which drives the asynchronous loading of glTF data.
- "Poor man's zero-copy deserialization".

### Changed

- Moved all extension data structures into a new `extensions` module.
- Made the `Get` trait behave the same as the `TryGet` trait.

### Removed

- Removed the `v1` module, as the crate no longer supports glTF 1.0.
- Removed the `TryGet` trait in favour of the redesigned `Get` trait.

## [0.5.0]- 2017-06-10

### Added

- New `v2` module, containing all glTF 2.0 data structures.
- Initial implementation of the glTF 2.0 data structures.

## [0.4.1]- 2017-05-06

### Changed

- Internal improvements to decrease the crate build time.

## [0.4.0] - 2017-04-24

### Added

- New `v1` module, containing all glTF 1.0 data structures.
- glTF objects are now categorised in submodules.
- Complete implementation of the glTF 1.0 data structures.

### Removed

- Removed `gl` crate dependency.

## [0.3.1] - 2017-03-17

### Changed

- Updated links to the official glTF documentation.

## [0.3.0] - 2017-02-16

### Changed

- Allowed the crate to build on the latest stable `rustc` (1.15)
  using the new `serde` frontend, i.e. with the serde `proc_macro`.

## [0.2.1] - 2016-11-17

### Changed

- Allowed the crate to build on the latest stable `rustc` (1.14)
  using the `serde_codegen` crate.

## [0.2.0] - 2016-11-15

### Added

- Added `Technique` data structure for glTF 1.0.

## [0.1.1] - 2016-11-13

### Added

- New documentation for glTF 1.0.

## [0.1.0] - 2016-11-13

### Added

- Initial (incomplete) glTF 1.0 implementation.
