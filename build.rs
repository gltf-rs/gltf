
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    #[cfg(test)]
    let should_download_sample_models = !std::path::Path::new("glTF-Sample-Models").exists();
    #[cfg(not(test))]
    let should_download_sample_models = false;
    if should_download_sample_models {
        println!("curl \"https://codeload.github.com/KhronosGroup/glTF-Sample-Models/zip/master\" -o master.zip");
        println!("unzip -q master.zip");
        println!("mv glTF-Sample-Models-master glTF-Sample-Models");
    }
}
