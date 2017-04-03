
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use v2::{accessor, material, Extras, Index, Root};

enum_number! {
    Mode {
        Points = 0,
        Lines = 1,
        LineLoop = 2,
        LineStrip = 3,
        Triangles = 4,
        TriangleStrip = 5,
        TriangleFan = 6,
    }
}

/// Extension specific data for `Mesh`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MeshExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mesh<E: Extras> {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: MeshExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Mesh,
    
    /// Optional user-defined name for this object.
    pub name: Option<String>,
    
    /// Defines the geometry to be renderered with a material.
    pub primitives: Vec<Primitive<E>>,

    /// Defines the weights to be applied to the morph targets.
    #[serde(default)]
    pub weights: Vec<f32>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Primitive<E: Extras> {
    /// Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    #[serde(default)]
    pub attributes: HashMap<String, Index<accessor::Accessor<E>>>,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: PrimitiveExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: <E as Extras>::MeshPrimitive,
    
    /// The `Accessor` that contains the indices.
    pub indices: Option<Index<accessor::Accessor<E>>>,
    
    /// The material to apply to this primitive when rendering.
    pub material: Index<material::Material<E>>,
    
    /// The type of primitives to render.
    #[serde(default)]
    pub mode: Mode,
    
    /// Maps attribute names (only `"POSITION"` and `"NORMAL"`) to their
    /// deviations in the morph target.
    // TODO: Confirm that this the correct implementation
    #[serde(default)]
    pub targets: Vec<HashMap<String, Index<accessor::Accessor<E>>>>,
}

/// Extension specific data for `Primitive`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PrimitiveExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl<E: Extras> Mesh<E> {
    #[doc(hidden)]
    pub fn range_check(&self, root: &Root<E>) -> Result<(), ()> {
        for primitive in &self.primitives {
            for accessor in primitive.attributes.values() {
                let _ = root.try_get(accessor)?;
            }
            if let Some(ref indices) = primitive.indices {
                let _ = root.try_get(indices)?;
            }
            let _ = root.try_get(&primitive.material)?;
            for map in &primitive.targets {
                for accessor in map.values() {
                    let _ = root.try_get(accessor)?;
                }
            }
        }
        Ok(())
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Triangles
    }
}
