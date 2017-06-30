
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export]
macro_rules! try_validate {
    ($field:expr, $root:expr, $path:expr, $report:expr) => {
        if $field.validate($root, $path, $report) == ::validation::Action::Stop {
            return ::validation::Action::Stop;
        }
    }
}
