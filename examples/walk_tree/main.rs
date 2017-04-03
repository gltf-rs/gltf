
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf;

type Extras = gltf::v2::extras::None;

fn walk_nodes(node: &gltf::v2::tree::Node<Extras>, level: u32) {
    println!("Node");
    for child in node.walk_child_nodes() {
        walk_nodes(&child, level + 1);
    }
}

fn walk_tree(root: &gltf::v2::tree::Root<Extras>) {
    for scene in root.walk_scenes() {
        if let Some(name) = scene.data().name.as_ref() {
            println!("Scene \"{}\":", name);
        } else {
            println!("Unnamed scene");
        };
        for node in scene.walk_nodes() {
            walk_nodes(&node, 1);
        }
    }
}

fn main() {
    let path = "glTF-Sample-Models/2.0/Lantern/glTF/Lantern.gltf";
    let root = gltf::v2::import::<_, Extras>(path).unwrap();
    let tree = root.tree();
    walk_tree(&tree);
}

