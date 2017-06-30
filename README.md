
<h1 align="center">
    gltf
</h1>
<p align="center">
   <a href="https://travis-ci.org/alteous/gltf">
      <img src="https://travis-ci.org/alteous/gltf.svg?branch=master" alt="travis">
   </a>
   <a href="https://crates.io/crates/gltf">
      <img src="https://img.shields.io/crates/v/gltf.svg" alt="crates.io">
   </a>
   <a href="https://docs.rs/gltf">
      <img src="https://docs.rs/gltf/badge.svg" alt="docs.rs">
   </a>
   <a href="https://gitter.im/alteous/gltf">
      <img src="https://img.shields.io/gitter/room/alteous/gltf.svg" alt="gitter">
   </a>
</p>
<hr>

This crate is intended to load [glTF](https://www.khronos.org/gltf), a file format designed for the efficient transmission of 3D assets.

`rustc` version 1.15 or above is required.

### Usage

Add `gltf` to the dependencies section of `Cargo.toml`.

```toml
[dependencies]
gltf = "0.5"
```
Import some glTF 1.0 / 2.0.

```rust
extern crate gltf;

fn main() {
    // Import some glTF 1.0
    match gltf::v1::import("Example-1.0.gltf") {
        Ok(root) => println!("glTF 1.0: {:#?}", root),
        Err(err) => println!("{:?}", err),
    }

    // Import some glTF 2.0
    match gltf::v2::import("Example-2.0.gltf") {
        Ok(root) => println!("glTF 2.0: {:#?}", root),
        Err(err) => println!("{:?}", err),
    }
}
```

### Extensions

All glTF extensions are opt-in and are enabled by specifying [features](http://doc.crates.io/specifying-dependencies.html#choosing-features) in your crate's Cargo.toml manifest file.

Currently, only the `KHR_binary_glTF` extension for glTF 1.0 is supported by the crate.

#### Examples

Enabling the `KHR_binary_glTF` extension.

```toml
[dependencies]
gltf = { version = "0.5", features = ["KHR_binary_glTF"] }
```

### Extras

By default, `gltf` ignores all `extras` included with glTF assets. You can negate this by enabling the `extras` feature.

```toml
[dependencies]
gltf = { version = "0.5", features = ["extras"] }
```

### Examples

#### gltf_display

If you want to see how the structure of the glTF file is deserialized, you can
use the example here to poke at it.

```sh
cargo run --example gltf_display Example.gltf
```

