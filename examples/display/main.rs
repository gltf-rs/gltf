// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf;

fn main() {
    let mut importer = gltf::ZeroCopyImporter::new();
    let path = std::env::args().nth(1).unwrap();
    println!("{:#?}", importer.import_from_path(path));
}
