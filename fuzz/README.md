# Fuzzing gltf

This crate uses `cargo fuzz` with `libFuzzer` for fuzzing. The only target fuzzes creating the `Gltf` struct from a slice.

To start fuzzing, first install `cargo-fuzz` and a nightly compiler:

```bash
cargo install cargo-fuzz
rustup instally nightly
```

Then inside `/fuzz`:

```bash
# create the corpus (seed inputs for fuzzing)
mkdir -p corpus/gltf_from_slice
# seed the corpus with valid gltf binary data
cp ../examples/Box* ../examples/Lantern.gltf ./corpus/gltf_from_slice
```

Then start fuzzing!

```bash
cargo +nightly fuzz run gltf_from_slice
```

This will start fuzzing with one thread until a crash or slowdown is found. The [cargo-fuzz book](https://rust-fuzz.github.io/book/cargo-fuzz.html) has some more resources and tips.
