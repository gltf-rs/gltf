
<h1 align="center">
    gltf
</h1>
<p align="center">
   <a href="https://travis-ci.org/gltf-rs/gltf">
      <img src="https://travis-ci.org/gltf-rs/gltf.svg?branch=master" alt="travis">
   </a>
   <a href="https://crates.io/crates/gltf">
      <img src="https://img.shields.io/crates/v/gltf.svg" alt="crates.io">
   </a>
   <a href="https://docs.rs/gltf">
      <img src="https://docs.rs/gltf/badge.svg" alt="docs.rs">
   </a>
</p>
<hr>

This crate is intended to load [glTF 2.0](https://www.khronos.org/gltf), a file format designed for the efficient transmission of 3D assets.

`rustc` version 1.32 or above is required.

### Reference infographic

![infographic](https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/figures/gltfOverview-2.0.0b.png)

<p align="center">From <a href="https://github.com/javagl/gltfOverview">javagl/gltfOverview.</a></p>

### Usage

See the [crate documentation](https://docs.rs/gltf) for example usage.

### Features

#### Extras and names

By default, `gltf` ignores all `extras` and `names` included with glTF assets. You can negate this by enabling the `extras` and `names` features, respectively.

```toml
[dependencies.gltf]
version = "0.16"
features = ["extras", "names"]
```

#### glTF extensions

The following glTF extensions are supported by the crate:

* `KHR_lights_punctual`
* `KHR_materials_pbrSpecularGlossiness`
* `KHR_materials_unlit`
* `KHR_texture_transform`

To use an extension, list its name in the `features` section.

```toml
[dependencies.gltf]
features = ["KHR_materials_unlit"]
```

### Examples

#### gltf-display

Demonstrates how the glTF JSON is deserialized.

```sh
cargo run --example gltf-display path/to/asset.gltf
```
#### gltf-export

Demonstrates how glTF JSON can be built and exported using the `gltf-json` crate.

```sh
cargo run --example gltf-export
```

#### gltf-roundtrip

Deserializes and serializes the JSON part of a glTF asset.

```sh
cargo run --example gltf-roundtrip path/to/asset.gltf
```

#### gltf-tree

Visualises the scene heirarchy of a glTF asset, which is a strict tree of nodes.

```sh
cargo run --example gltf-tree path/to/asset.gltf
```

