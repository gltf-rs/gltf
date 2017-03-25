// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v1::texture::Filter;
use v1::texture::Wrap;
use traits::{Extensions, Extras};

#[derive(Debug, Deserialize, Serialize)]
pub struct Sampler<E: Extensions, X: Extras> {
    /// Magnification filter.
    #[serde(rename = "magFilter")]
    #[serde(default = "sample_mag_filter_default")]
    pub mag_filter: Filter,

    /// Minification filter.
    #[serde(rename = "minFilter")]
    #[serde(default = "sample_min_filter_default")]
    pub min_filter: Filter,

    /// s wrapping mode.
    #[serde(rename = "wrapS")]
    #[serde(default = "sample_wrap_s_default")]
    pub wrap_s: Wrap,

    /// t wrapping mode.
    #[serde(rename = "wrapT")]
    #[serde(default = "sample_wrap_t_default")]
    pub wrap_t: Wrap,

    /// The user-defined name of this object.
    pub name: Option<String>,
    
    /// A dictionary object containing extension-specific data.
    #[serde(default)]
    pub extensions: <E as Extensions>::Sampler,

    /// Application-specific data.
    #[serde(default)]
    pub extras: <X as Extras>::Sampler,
}

fn sample_mag_filter_default() -> Filter {
    Filter::Linear
}

fn sample_min_filter_default() -> Filter {
    Filter::NearestMipmapLinear
}

fn sample_wrap_s_default() -> Wrap {
    Wrap::Repeat
}

fn sample_wrap_t_default() -> Wrap {
    Wrap::Repeat
}
