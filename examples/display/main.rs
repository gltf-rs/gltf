// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf;

use gltf::Generic::{V1, V2};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    match gltf::import(path) {
        Ok(V1(root)) => {
            println!("glTF version 1.0");
            println!("{:#?}", root);
        },
        Ok(V2(root)) => {
            println!("glTF version 2.0");
            println!("{:#?}", root);
        },
        Err(err) => println!("Error: {:?}", err),
    }
}
