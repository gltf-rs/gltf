
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use v2::json::*;

pub use self::error::Error;

use self::error::oob;

/// Trait for validating glTF JSON data against the 2.0 specification.
pub trait Validate {
    /// Validates the data against the glTF 2.0 specification.
    fn validate<E: FnMut(Error)>(&self, root: &Root, err: E);
}

/// Contains `Error` and other related data structures.
pub mod error {
    use std::borrow::Cow;
    
    #[derive(Clone, Copy, Debug)]
    /// Specifies what kind of error occured during validation.
    pub enum Kind {
        /// An index was found to be out of bounds.
        IndexOutOfBounds,
    }

    /// Error type encountered when validating glTF JSON data.
    #[derive(Clone, Debug)]
    pub struct Error {
        /// What kind of error occured during validation.
        kind: Kind,

        /// The source of the error, i.e. the path of the offending JSON data.
        source: Cow<'static, str>,
    }

    /// Returns an `IndexOutOfBounds` error.
    pub fn oob(source: Cow<'static, str>) -> Error {
        Error {
            kind: Kind::IndexOutOfBounds,
            source: source,
        }
    }

    impl Error {
        /// Propagates an error up in the JSON heirarchy.
        pub fn propagate(self, from_source: Cow<'static, str>) -> Self {
            let src = format!("{}.{}", from_source.as_ref(), self.source.as_ref());
            Self {
                kind: self.kind,
                source: Cow::from(src),
            }
        }
    }

    impl ::std::error::Error for Error {
        fn description(&self) -> &str {
            match self.kind {
                Kind::IndexOutOfBounds => "index out of range",
            }
        }
    }

    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            use ::std::error::Error;
            write!(f, "{}: {}", self.source.as_ref(), self.description())
        }
    }
}

impl Validate for mesh::Primitive {
    fn validate<E: FnMut(Error)>(&self, root: &Root, mut err: E) {
        for (attribute, accessor) in self.attributes.iter() {
            if let Err(_) = root.try_get(accessor) {
                let src = Cow::from(format!("attributes[{}]", attribute));
                err(oob(src));
            }
        }
        if let Some(ref indices) = self.indices {
            if let Err(_) = root.try_get(indices) {
                let src = Cow::from("indices");
                err(oob(src));
            }
        }
        if let Some(ref material) = self.material {
            if let Err(_) = root.try_get(&material) {
                let src = Cow::from("material");
                err(oob(src));
            }
        }
        if let Some(ref targets) = self.targets {
            for (index, map) in targets.iter().enumerate() {
                for (attribute, accessor) in map.iter() {
                    if let Err(_) = root.try_get(accessor) {
                        let src = format!("targets[{}][{}]", index, attribute);
                        err(oob(Cow::from(src)));
                    }
                }
            }
        }
    }
}

impl Validate for mesh::Mesh {
    fn validate<E: FnMut(Error)>(&self, root: &Root, mut err: E) {
        for (index, primitive) in self.primitives.iter().enumerate() {
            primitive.validate(root, |e| {
                let src = Cow::from(format!("primitive[{}]", index));
                err(e.propagate(src));
            });
        }
    }
}

impl Validate for accessor::Accessor {
    fn validate<E: FnMut(Error)>(&self, root: &Root, mut err: E) {
        if let Some(ref sparse) = self.sparse {
            if let Err(_) = root.try_get(&sparse.indices.buffer_view) {
                let src = Cow::from("sparse.indices.buffer_view");
                err(oob(src));
            }
            if let Err(_) = root.try_get(&sparse.values.buffer_view) {
                let src = Cow::from("sparse.values.buffer_view");
                err(oob(src));
            }
        }
        if let Err(_) = root.try_get(&self.buffer_view) {
            let src = Cow::from("buffer_view");
            err(oob(src));
        }
    }
}

impl Validate for material::Material {
    fn validate<E: FnMut(Error)>(&self, root: &Root, mut err: E) {
        if let Some(ref texture) = self.normal_texture {
            if let Err(_) = root.try_get(&texture.index) {
                let src = Cow::from("normal_texture.index");
                err(oob(src));
            }
        }
        if let Some(ref texture) = self.occlusion_texture {
            if let Err(_) = root.try_get(&texture.index) {
                let src = Cow::from("occlusion_texture.index");
                err(oob(src));
            }
        }
        if let Some(ref texture) = self.emissive_texture {
            if let Err(_) = root.try_get(&texture.index) {
                let src = Cow::from("emissive_texture.index");
                err(oob(src));
            }
        }
        if let Some(ref bct) = self.pbr_metallic_roughness.base_color_texture {
            if let Err(_) = root.try_get(&bct.index) {
                let src = Cow::from("pbr_metallic_roughness.base_color_texture.index");
                err(oob(src));
            }
        }
        if let Some(ref mrt) = self.pbr_metallic_roughness.metallic_roughness_texture {
            if let Err(_) = root.try_get(&mrt.index) {
                let src = Cow::from("pbr_metallic_roughness.metallic_roughness_texture.index");
                err(oob(src));
            }
        }
    }
}

impl Validate for animation::Animation {
    fn validate<E: FnMut(Error)>(&self, root: &Root, mut err: E) {
        for (index, sampler) in self.samplers.iter().enumerate() {
            if let Err(_) = root.try_get(&sampler.input) {
                let src = Cow::from(format!("samplers[{}].input", index));
                err(oob(src));
            }
            if let Err(_) = root.try_get(&sampler.output) {
                let src = Cow::from(format!("samplers[{}].output", index));
                err(oob(src));
            }
        }
        for (index, channel) in self.channels.iter().enumerate() {
            if let Err(_) = root.try_get(&channel.target.node) {
                let src = Cow::from(format!("channels[{}].target.node", index));
                err(oob(src));
            }
            if channel.sampler.value() as usize >= self.samplers.len() {
                let src = Cow::from(format!("channels[{}].sampler", index));
                err(oob(src));
            }
        }
    }
}

impl Validate for buffer::Buffer {
    fn validate<E: FnMut(Error)>(&self, _root: &Root, _err: E) {
        // nop
    }
}

impl Validate for camera::Camera {
    fn validate<E: FnMut(Error)>(&self, _root: &Root, _err: E) {
        // nop
    }
}

impl Validate for buffer::View {
    fn validate<E: FnMut(Error)>(&self, root: &Root, mut err: E) {
        if let Err(_) = root.try_get(&self.buffer) {
            let src = Cow::from("buffer");
            err(oob(src));
        }
    }
}

impl Validate for image::Image {
    fn validate<E: FnMut(Error)>(&self, root: &Root, mut err: E) {
        if let Some(ref buffer_view) = self.buffer_view {
            if let Err(_) = root.try_get(buffer_view) {
                let src = Cow::from("buffer_view");
                err(oob(src));
            }
        }
    }
}

impl Validate for skin::Skin {
    fn validate<E: FnMut(Error)>(&self, root: &Root, mut err: E) {
        if let Some(ref accessor) = self.inverse_bind_matrices {
            if let Err(_) = root.try_get(accessor) {
                let src = Cow::from("inverse_bind_matrices");
                err(oob(src));
            }
        }
        for (index, joint) in self.joints.iter().enumerate() {
            if let Err(_) = root.try_get(joint) {
                let src = Cow::from(format!("joints[{}]", index));
                err(oob(src));
            }
        }
        if let Some(ref node) = self.skeleton {
            if let Err(_) = root.try_get(node) {
                let src = Cow::from("skeleton");
                err(oob(src));
            }
        }
    }
}

impl Validate for texture::Sampler {
    fn validate<E: FnMut(Error)>(&self, _root: &Root, _err: E) {
        // nop
    }
}

impl Validate for texture::Texture {
    fn validate<E: FnMut(Error)>(&self, root: &Root, mut err: E) {
        if let Some(ref sampler) = self.sampler {
            if let Err(_) = root.try_get(sampler) {
                let src = Cow::from("sampler");
                err(oob(src));
            }
        }
        if let Err(_) = root.try_get(&self.source) {
            let src = Cow::from("source");
            err(oob(src));
        }
    }
}

impl Validate for scene::Node {
    fn validate<E: FnMut(Error)>(&self, root: &Root, mut err: E) {
        if let Some(ref camera) = self.camera {
            if let Err(_) = root.try_get(&camera) {
                let src = Cow::from("camera");
                err(oob(src));
            }
            if let Some(ref children) = self.children {
                for (index, node) in children.iter().enumerate() {
                    if let Err(_) = root.try_get(node) {
                        let src = Cow::from(format!("children[{}]", index));
                        err(oob(src));
                    }
                }
            }
        }
        if let Some(ref mesh) = self.mesh {
            if let Err(_) = root.try_get(mesh) {
                let src = Cow::from("mesh");
                err(oob(src));
            }
        }
        if let Some(ref skin) = self.skin {
            if let Err(_) = root.try_get(skin) {
                let src = Cow::from("skin");
                err(oob(src));
            }
        }
    }
}

impl Validate for scene::Scene {
    fn validate<E: FnMut(Error)>(&self, root: &Root, mut err: E) {
        for (index, node) in self.nodes.iter().enumerate() {
            if let Err(_) = root.try_get(node) {
                let src = Cow::from(format!("nodes[{}]", index));
                err(oob(src));
            }
        }
    }
}
