
#[cfg(feature = "serde_codegen")]
fn main() {
    extern crate serde_codegen;

    use std::path::Path;

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dst = Path::new(&out_dir).join("definitions.rs");
    let src = Path::new("src/lib.in.rs");

    serde_codegen::expand(&src, &dst)
        .expect("Serde code generation error");
}

#[cfg(not(feature = "serde_codegen"))]
fn main() {
    // noop
}

