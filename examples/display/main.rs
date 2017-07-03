// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate futures;
extern crate gltf;

use futures::executor::spawn;
use std::error::Error;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    match spawn(gltf::import::from_path(&path)).wait_future() {
        Ok(gltf) => println!("{:#?}", gltf),
        Err(err) => println!("Invalid glTF ({})", err.description()),
    }
}
