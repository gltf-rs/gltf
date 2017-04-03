# gltf

This library is intended to load [glTF](https://www.khronos.org/gltf), a file format designed for the efficient transmission of 3D assets.

This library requires rustc version 1.15 or above in order to compile.

[![Build Status](https://travis-ci.org/alteous/gltf.svg?branch=master)](https://travis-ci.org/alteous/gltf)
[![crates.io](https://img.shields.io/crates/v/gltf.svg)](https://crates.io/crates/gltf)
[![docs.rs](https://docs.rs/gltf/badge.svg)](https://docs.rs/gltf)

### [Documentation](https://docs.rs/gltf)

### Usage

#### All glTF versions

Add `gltf` to the dependencies section of `Cargo.toml`.

```toml
[dependencies]
gltf = "0.5"
```
#### glTF 1.0

```rust
extern crate gltf;

// This loads all available "extra" data
type Extras = gltf::v1::extras::Any;

fn main() {
    // Import some glTF 1.0 and print the data to the console
    let path = "path/to/asset.gltf";
    let root = gltf::v1::import::<_, Extras>(path).unwrap();
    println!("{:#?}", root);
}
```

#### glTF 2.0

```rust
extern crate gltf;

// This ignores all "extra" data
type Extras = gltf::v2::extras::None;

fn main() {
    // Import some glTF 2.0 and walk the node hierarchy of its scenes
    let path = "path/to/asset.gltf";
    let gltf = gltf::v2::import::<_, Extras>(path).unwrap();
    for scene in gltf.tree().walk_scenes() {
        for node in scene.walk_nodes() {
            visit_node(&node);
        }
    }
}

fn visit_node(node: &gltf::v2::tree::Node<Extras>) {
    for child in node.walk_child_nodes() {
        visit_node(&child);
    }
}
```
### Extensions

All glTF extensions are opt-in and are enabled by specifying [features](http://doc.crates.io/specifying-dependencies.html#choosing-features) in the Cargo.toml manifest file. For example, the below demonstrates enabling the `KHR_binary_glTF` extension:

```toml
[dependencies]
gltf = { version = "0.5", features = ["KHR_binary_glTF"] }
```

Currently, only the `KHR_binary_glTF` extension for glTF 1.0 is supported by the library.

### Examples

#### gltf_display

If you want to see how the structure of the glTF file is deserialized, you can
use the example here to poke at it.

```sh
cargo run --example gltf_display path/to/gltf_file
```

#### walk_tree

If you want to view the node hierarchy of a glTF 2.0 asset, try this example.

```sh
cargo run --example walk_tree path/to/gltf_file
```
