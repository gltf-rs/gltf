# gltf

This crate is intended to load [glTF 2.0](https://www.khronos.org/gltf), a file format designed for the efficient transmission of 3D assets.

`rustc` version 1.15 or above is required.

[![Build Status](https://travis-ci.org/alteous/gltf.svg?branch=master)](https://travis-ci.org/alteous/gltf)
[![crates.io](https://img.shields.io/crates/v/gltf.svg)](https://crates.io/crates/gltf)
[![docs.rs](https://docs.rs/gltf/badge.svg)](https://docs.rs/gltf)

### [Documentation](https://docs.rs/gltf)

### Example Usage

#### Importing some glTF 2.0

```rust
extern crate gltf;

fn main() {
    match gltf::import("Example.gltf") {
        Ok(root) => println!("glTF 2.0: {:#?}", root),
        Err(err) => println!("{:?}", err),
    }
}
```

### Extras

By default, `gltf` ignores all `extras` included with glTF assets. You can negate this by enabling the `extras` feature.

```toml
[dependencies]
gltf = { version = "0.6", features = ["extras"] }
```

### Examples

#### gltf_display

If you want to see how the structure of the glTF file is deserialized, you can
use the example here to poke at it.

```sh
cargo run --example gltf_display glTF-Sample-Models/2.0/Box/glTF/Box.gltf
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.