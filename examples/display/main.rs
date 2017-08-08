
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf;

use gltf::json;
use std::{fs, io};

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let file = fs::File::open(&path).expect("I/O error");
        let buf_reader = io::BufReader::new(file);
        let json: json::Root = json::from_reader(buf_reader).expect("I/O error");
        println!("{:#?}", json);
    } else {
        println!("usage: gltf-display <PATH>");
    }
}
