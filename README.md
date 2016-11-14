# gltf

This library is intended to load [.gltf files](https://www.khronos.org/gltf), a file format designed for the efficient transmission of 3D models. It is in its early stages of development, hence it is not fully-featured and future releases are not guaranteed to be backward compatible.

[![Build Status](https://travis-ci.org/Alteous/gltf.svg?branch=master)](https://travis-ci.org/Alteous/gltf)
[![Crates.io](https://img.shields.io/crates/v/gltf.svg?maxAge=2592000)]()
<a href="https://gitter.im/alteous/gltf">
    <img src="https://img.shields.io/badge/GITTER-join%20chat-green.svg?style=flat-square" alt="Gitter Chat">
</a>

[Documentation](https://docs.rs/gltf)

### Usage

Currently `gltf` requires the latest nightly compiler in order to build successfully. An up-to-date compiler may be obtained using [rustup](https://www.rustup.rs/) or from the [official downloads page](https://www.rust-lang.org/en-US/downloads.html). A stable version of the library is planned to be available soon.

Add `gltf` to the dependencies section of `Cargo.toml`:

```toml
[dependencies]
gltf = "0.1"
```

Import the crate in your library or executable:

```rust
extern crate gltf;

use gltf::Gltf;
```

Load a glTF file:

```rust
fn main() {
    let gltf = Gltf::new("Foo.gltf").unwrap();
}
```

### Future Goals

 * Ability to be compilied with the latest stable toolchain
 * Full conformance to the [specification](https://github.com/KhronosGroup/glTF/blob/master/specification/README.md#techniques)
 * Replace untyped `GLenum` identifiers with equivalent type-safe constants

