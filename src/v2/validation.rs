
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use std::collections::HashMap;
use v2::json::*;
use v2::json::root::TryGet;

pub use self::error::Error;

/// Trait for validating glTF JSON data against the 2.0 specification.
pub trait Validate {
    /// Validates the data against the glTF 2.0 specification.
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error);
}

pub mod error {
    use std;
    use super::JsonPath;
    
    #[derive(Clone, Debug)]
    pub struct Error {
        /// JSON source path of the offending data.
        path: JsonPath,

        /// Error kind.
        kind: Kind,
    }

    #[derive(Clone, Copy, Debug)]
    /// Specifies what kind of error occured during validation.
    pub enum Kind {
        /// An index was found to be out of bounds.
        IndexOutOfBounds,
    }

    impl Error {
        /// Returns an `IndexOutOfBounds` error.
        pub fn index_out_of_bounds(path: JsonPath) -> Error {
            Error {
                kind: Kind::IndexOutOfBounds,
                path: path,
            }
        }
    }

    impl std::error::Error for Error {
        fn description(&self) -> &str {
            match self.kind {
                Kind::IndexOutOfBounds => "Index out of bounds",
            }
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            use std::error::Error;
            write!(f, "{} ({})", self.path, self.description())
        }
    }
}
    
/// JSON source path.
#[derive(Clone, Debug)]
pub struct JsonPath(String);

impl JsonPath {
    pub fn new() -> Self {
        JsonPath(String::new())
    }

    pub fn field(&self, name: &str) -> Self {
        if self.0.is_empty() {
            JsonPath(name.to_string())
        } else {
            JsonPath(format!("{}.{}", self.0, name))
        }
    }

    pub fn index(&self, index: usize) -> Self {
        JsonPath(format!("{}[{}]", self.0, index))
    }

    pub fn key(&self, key: &str) -> Self {
        JsonPath(format!("{}[\"{}\"]", self.0, key))
    }
}

impl std::fmt::Display for JsonPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Validate> Validate for HashMap<String, T> {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        for (key, value) in self.iter() {
            value.validate(root, path.key(key), err);
        }
    }
}

impl<T: Validate> Validate for Index<T>
    where Root: TryGet<T>
{
    fn validate<E>(&self, root: &Root, path: JsonPath, mut err: &mut E)
        where E: FnMut(Error)
    {
        if root.try_get(self).is_err() {
            err(Error::index_out_of_bounds(path));
        }
    }
}

impl<T: Validate> Validate for Option<T> {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        if let Some(value) = self.as_ref() {
            value.validate(root, path, err);
        }
    }
}

impl<T: Validate> Validate for Vec<T> {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        for (index, value) in self.iter().enumerate() {
            value.validate(root, path.index(index), err);
        }
    }
}

impl Validate for mesh::Primitive {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        self.attributes.validate(root, path.field("attributes"), err);
        self.indices.validate(root, path.field("indices"), err);
        self.material.validate(root, path.field("material"), err);
        self.targets.validate(root, path.field("targets"), err);
    }
}

impl Validate for mesh::Mesh {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        self.primitives.validate(root, path.field("primitives"), err);
    }
}

impl Validate for accessor::sparse::Sparse {
    fn validate<E>(&self, _root: &Root, path: JsonPath, _err: &mut E)
        where E: FnMut(Error)
    {
        // TODO
    }
}

impl Validate for accessor::Accessor {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        self.buffer_view.validate(root, path.field("bufferView"), err);
        self.sparse.validate(root, path.field("sparse"), err);
    }
}

impl Validate for texture::Info {
    fn validate<E>(&self, _root: &Root, path: JsonPath, _err: &mut E)
        where E: FnMut(Error)
    {
        // TODO
    }
}

impl Validate for material::NormalTexture {
    fn validate<E>(&self, _root: &Root, path: JsonPath, _err: &mut E)
        where E: FnMut(Error)
    {
        // TODO
    }
}

impl Validate for material::OcclusionTexture {
    fn validate<E>(&self, _root: &Root, path: JsonPath, _err: &mut E)
        where E: FnMut(Error)
    {
        // TODO
    }
}

impl Validate for material::PbrMetallicRoughness {
    fn validate<E>(&self, _root: &Root, path: JsonPath, _err: &mut E)
        where E: FnMut(Error)
    {
        // TODO
    }
}

impl Validate for material::Material {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        self.normal_texture.validate(root, path.field("normalTexture"), err);
        self.occlusion_texture.validate(root, path.field("occlusionTexture{}"), err);
        self.emissive_texture.validate(root, path.field("emissiveTexture{}"), err);
        self.pbr_metallic_roughness.validate(root, path.field("emissiveTexture{}"), err);
    }
}

impl Validate for animation::Sampler {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        self.input.validate(root, path.field("input"), err);
        self.output.validate(root, path.field("output"), err);
    }
}

impl Validate for animation::Channel {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        self.target.node.validate(root, path.field("target.node"), err);
    }
}

impl Validate for animation::Animation {
    fn validate<E>(&self, root: &Root, path: JsonPath, mut err: &mut E)
        where E: FnMut(Error)
    {
        self.samplers.validate(root, path.field("samplers"), err);
        self.channels.validate(root, path.field("channels"), err);
        for (index, channel) in self.channels.iter().enumerate() {
            if channel.sampler.value() as usize >= self.samplers.len() {
                let field = format!("channels[{}].sampler", index);
                err(Error::index_out_of_bounds(path.field(&field)));
            }
        }
    }
}

impl Validate for buffer::Buffer {
    fn validate<E>(&self, _root: &Root, _path: JsonPath, _err: &mut E)
        where E: FnMut(Error)
    {
        // nop
    }
}

impl Validate for camera::Camera {
    fn validate<E>(&self, _root: &Root, _path: JsonPath, _err: &mut E)
        where E: FnMut(Error)
    {
        // nop
    }
}

impl Validate for buffer::View {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        self.buffer.validate(root, path.field("buffer"), err);
    }
}

impl Validate for image::Image {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        self.buffer_view.validate(root, path.field("bufferView"), err);
    }
}

impl Validate for skin::Skin {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        self.inverse_bind_matrices.validate(root, path.field("inverseBindMatrices"), err);
        self.joints.validate(root, path.field("joints"), err);
        self.skeleton.validate(root, path.field("skeleton"), err);
    }
}

impl Validate for texture::Sampler {
    fn validate<E>(&self, _root: &Root, _path: JsonPath, _err: &mut E)
        where E: FnMut(Error)
    {
        // nop
    }
}

impl Validate for texture::Texture {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        self.sampler.validate(root, path.field("sampler"), err);
        self.source.validate(root, path.field("source"), err);
    }
}

impl Validate for scene::Node {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        self.camera.validate(root, path.field("camera"), err);
        self.children.validate(root, path.field("children"), err);
        self.mesh.validate(root, path.field("mesh"), err);
        self.skin.validate(root, path.field("skin"), err);
    }
}

impl Validate for scene::Scene {
    fn validate<E>(&self, root: &Root, path: JsonPath, err: &mut E)
        where E: FnMut(Error)
    {
        self.nodes.validate(root, path.field("nodes"), err);
    }
}
