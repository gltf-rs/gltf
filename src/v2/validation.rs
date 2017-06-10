
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::json::*;

pub trait Validate {
    fn validate(&self, root: &Root) -> Result<(), ()>;
}

impl Validate for mesh::Mesh {
    fn validate(&self, root: &Root) -> Result<(), ()> {
        for primitive in &self.primitives {
            for accessor in primitive.attributes.values() {
                let _ = root.try_get(accessor)?;
            }
            if let Some(ref indices) = primitive.indices {
                let _ = root.try_get(indices)?;
            }
            if let Some(ref material) = primitive.material {
                let _ = root.try_get(&material)?;
            }
            if let Some(ref targets) = primitive.targets {
                for map in targets.iter() {
                    for accessor in map.values() {
                        let _ = root.try_get(accessor)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl Validate for accessor::Accessor {
    fn validate(&self, root: &Root) -> Result<(), ()> {
        if let Some(ref sparse) = self.sparse {
            let _ = root.try_get(&sparse.indices.buffer_view)?;
            let _ = root.try_get(&sparse.values.buffer_view)?;
        }
        let _ = root.try_get(&self.buffer_view)?;
        Ok(())
    }
}

impl Validate for material::Material {
    fn validate(&self, root: &Root) -> Result<(), ()> {
        if let Some(ref texture) = self.normal_texture {
            let _ = root.try_get(&texture.index)?;
        }
        if let Some(ref texture) = self.occlusion_texture {
            let _ = root.try_get(&texture.index)?;
        }
        if let Some(ref texture) = self.emissive_texture {
            let _ = root.try_get(&texture.index)?;
        }
        if let Some(ref bct) = self.pbr_metallic_roughness.base_color_texture {
            let _ = root.try_get(&bct.index)?;
        }
        if let Some(ref mrt) = self.pbr_metallic_roughness.metallic_roughness_texture {
            let _ = root.try_get(&mrt.index)?;
        }
        Ok(())
    }
}

impl Validate for animation::Animation {
    fn validate(&self, root: &Root) -> Result<(), ()> {
        for sampler in &self.samplers {
            let _ = root.try_get(&sampler.input)?;
            let _ = root.try_get(&sampler.output)?;
        }
        for channel in &self.channels {
            let _ = root.try_get(&channel.target.node)?;
            if channel.sampler.value() as usize >= self.samplers.len() {
                return Err(());
            }
        }
        Ok(())
    }
}

impl Validate for buffer::Buffer {
    fn validate(&self, _root: &Root) -> Result<(), ()> {
        Ok(())
    }
}

impl Validate for camera::Camera {
    fn validate(&self, _root: &Root) -> Result<(), ()> {
        Ok(())
    }
}

impl Validate for buffer::View {
    fn validate(&self, root: &Root) -> Result<(), ()> {
        let _ = root.try_get(&self.buffer)?;
        Ok(())
    }
}

impl Validate for image::Image {
    fn validate(&self, root: &Root) -> Result<(), ()> {
        if let Some(ref buffer_view) = self.buffer_view {
            let _ = root.try_get(buffer_view)?;
        }
        Ok(())
    }
}

impl Validate for skin::Skin {
    fn validate(&self, root: &Root) -> Result<(), ()> {
        if let Some(ref accessor) = self.inverse_bind_matrices {
            let _ = root.try_get(accessor)?;
        }
        for joint in &self.joints {
            let _ = root.try_get(joint)?;
        }
        if let Some(ref node) = self.skeleton {
            let _ = root.try_get(node)?;
        }
        Ok(())
    }
}

impl Validate for texture::Sampler {
    fn validate(&self, _root: &Root) -> Result<(), ()> {
        Ok(())
    }
}

impl Validate for texture::Texture {
    fn validate(&self, root: &Root) -> Result<(), ()> {
        if let Some(ref sampler) = self.sampler {
            let _ = root.try_get(sampler)?;
        }
        let _ = root.try_get(&self.source)?;
        Ok(())
    }
}

impl Validate for scene::Node {
    fn validate(&self, root: &Root) -> Result<(), ()> {
        if let Some(ref camera) = self.camera {
            let _ = root.try_get(&camera)?;
            if let Some(ref children) = self.children {
            for node in children.iter() {
                let _ = root.try_get(node)?;
            }
            }
        }
        if let Some(ref mesh) = self.mesh {
            let _ = root.try_get(mesh)?;
        }
        if let Some(ref skin) = self.skin {
            let _ = root.try_get(skin)?;
        }
        Ok(())
    }
}

impl Validate for scene::Scene {
    fn validate(&self, root: &Root) -> Result<(), ()> {
        for node in &self.nodes {
            let _ = root.try_get(node)?;
        }
        Ok(())
    }
}
