# gltf

This library is intended to load [.gltf files](https://www.khronos.org/gltf), a file format designed for the efficient transmission of 3D models.

### Usage

Currently, `gltf` requires the lastest nightly compiler in order to build successfully. An up-to-date compiler may be obtained using [rustup](https://www.rustup.rs/) or from the [official downloads page](https://www.rust-lang.org/en-US/downloads.html). A stable version of the library is planned to be available soon.

Add `gltf` to the dependencies section of `Cargo.toml`:

```toml
[dependencies]
gltf = "0.1.0"
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

