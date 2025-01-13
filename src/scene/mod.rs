#[cfg(feature = "extensions")]
use serde_json::{Map, Value};

use crate::math::*;
use crate::{Camera, Document, Mesh, Skin};

/// Iterators.
pub mod iter;

/// The transform for a `Node`.
#[derive(Clone, Debug)]
pub enum Transform {
    /// 4x4 transformation matrix in column-major order.
    Matrix {
        /// 4x4 matrix.
        matrix: [[f32; 4]; 4],
    },

    /// Decomposed TRS properties.
    Decomposed {
        /// `[x, y, z]` vector.
        translation: [f32; 3],

        /// `[x, y, z, w]` quaternion, where `w` is the scalar.
        rotation: [f32; 4],

        /// `[x, y, z]` vector.
        scale: [f32; 3],
    },
}

impl Transform {
    /// Returns the matrix representation of this transform.
    ///
    /// If the transform is `Decomposed`, then the matrix is generated with the
    /// equation `matrix = translation * rotation * scale`.
    pub fn matrix(self) -> [[f32; 4]; 4] {
        match self {
            Transform::Matrix { matrix } => matrix,
            Transform::Decomposed {
                translation: t,
                rotation: r,
                scale: s,
            } => {
                let t = Matrix4::from_translation(Vector3::new(t[0], t[1], t[2]));
                let r = Matrix4::from_quaternion(Quaternion::new(r[3], r[0], r[1], r[2]));
                let s = Matrix4::from_nonuniform_scale(s[0], s[1], s[2]);
                (t * r * s).as_array()
            }
        }
    }

    /// Returns a decomposed representation of this transform.
    ///
    /// If the transform is `Matrix`, then the decomposition is extracted from the
    /// matrix.
    pub fn decomposed(self) -> ([f32; 3], [f32; 4], [f32; 3]) {
        match self {
            Transform::Matrix { matrix: m } => {
                let translation = [m[3][0], m[3][1], m[3][2]];
                #[rustfmt::skip]
                let mut i = Matrix3::new(
                    m[0][0], m[0][1], m[0][2],
                    m[1][0], m[1][1], m[1][2],
                    m[2][0], m[2][1], m[2][2],
                );
                let sx = i.x.magnitude();
                let sy = i.y.magnitude();
                let sz = i.determinant().signum() * i.z.magnitude();
                let scale = [sx, sy, sz];
                i.x.multiply(1.0 / sx);
                i.y.multiply(1.0 / sy);
                i.z.multiply(1.0 / sz);
                let r = Quaternion::from_matrix(i);
                let rotation = [r.v.x, r.v.y, r.v.z, r.s];
                (translation, rotation, scale)
            }
            Transform::Decomposed {
                translation,
                rotation,
                scale,
            } => (translation, rotation, scale),
        }
    }
}

/// A node in the node hierarchy.
///
/// When a node contains a skin, all its meshes contain `JOINTS_0` and `WEIGHTS_0`
/// attributes.
#[derive(Clone, Debug)]
pub struct Node<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::scene::Node,
}

/// The root nodes of a scene.
#[derive(Clone, Debug)]
pub struct Scene<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::scene::Scene,
}

impl<'a> Node<'a> {
    /// Constructs a `Node`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a json::scene::Node) -> Self {
        Self {
            document,
            index,
            json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the camera referenced by this node.
    pub fn camera(&self) -> Option<Camera<'a>> {
        self.json
            .camera
            .as_ref()
            .map(|index| self.document.cameras().nth(index.value()).unwrap())
    }

    /// Returns an `Iterator` that visits the node's children.
    pub fn children(&self) -> iter::Children<'a> {
        iter::Children {
            document: self.document,
            iter: self.json.children.as_ref().map_or([].iter(), |x| x.iter()),
        }
    }

    /// Returns extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        let ext = self.json.extensions.as_ref()?;
        Some(&ext.others)
    }

    /// Queries extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extension_value(&self, ext_name: &str) -> Option<&Value> {
        let ext = self.json.extensions.as_ref()?;
        ext.others.get(ext_name)
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }

    /// Returns the light at this node as defined by the `KHR_lights_punctual` extension.
    #[cfg(feature = "KHR_lights_punctual")]
    #[cfg_attr(docsrs, doc(cfg(feature = "KHR_lights_punctual")))]
    pub fn light(&self) -> Option<crate::khr_lights_punctual::Light<'a>> {
        if let Some(extensions) = self.json.extensions.as_ref() {
            if let Some(khr_lights_punctual) = extensions.khr_lights_punctual.as_ref() {
                let mut lights = self.document.lights().unwrap();
                Some(lights.nth(khr_lights_punctual.light.value()).unwrap())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns the mesh referenced by this node.
    pub fn mesh(&self) -> Option<Mesh<'a>> {
        self.json
            .mesh
            .as_ref()
            .map(|index| self.document.meshes().nth(index.value()).unwrap())
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns the node's transform.
    pub fn transform(&self) -> Transform {
        if let Some(m) = self.json.matrix {
            Transform::Matrix {
                matrix: [
                    [m[0], m[1], m[2], m[3]],
                    [m[4], m[5], m[6], m[7]],
                    [m[8], m[9], m[10], m[11]],
                    [m[12], m[13], m[14], m[15]],
                ],
            }
        } else {
            Transform::Decomposed {
                translation: self.json.translation.unwrap_or([0.0, 0.0, 0.0]),
                rotation: self.json.rotation.unwrap_or_default().0,
                scale: self.json.scale.unwrap_or([1.0, 1.0, 1.0]),
            }
        }
    }

    /// Returns the skin referenced by this node.
    pub fn skin(&self) -> Option<Skin<'a>> {
        self.json
            .skin
            .as_ref()
            .map(|index| self.document.skins().nth(index.value()).unwrap())
    }

    /// Returns the weights of the instantiated morph target.
    pub fn weights(&self) -> Option<&'a [f32]> {
        self.json.weights.as_deref()
    }
}

impl<'a> Scene<'a> {
    /// Constructs a `Scene`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a json::scene::Scene) -> Self {
        Self {
            document,
            index,
            json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        let ext = self.json.extensions.as_ref()?;
        Some(&ext.others)
    }

    /// Queries extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extension_value(&self, ext_name: &str) -> Option<&Value> {
        let ext = self.json.extensions.as_ref()?;
        ext.others.get(ext_name)
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns an `Iterator` that visits each root node of the scene.
    pub fn nodes(&self) -> iter::Nodes<'a> {
        iter::Nodes {
            document: self.document,
            iter: self.json.nodes.iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::*;
    use crate::scene::Transform;
    use std::f32::consts::PI;

    fn rotate(x: f32, y: f32, z: f32, r: f32) -> [f32; 4] {
        let r = Quaternion::from_axis_angle(Vector3::new(x, y, z).normalize(), r);
        [r.v.x, r.v.y, r.v.z, r.s]
    }

    fn test_decompose(translation: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) {
        let matrix = Transform::Decomposed {
            translation,
            rotation,
            scale,
        }
        .matrix();
        let (translation, rotation, scale) = Transform::Matrix { matrix }.decomposed();
        let check = Transform::Decomposed {
            translation,
            rotation,
            scale,
        }
        .matrix();
        assert_relative_eq!(
            Matrix4::from_array(check),
            Matrix4::from_array(matrix),
            epsilon = 0.05
        );
    }

    fn test_decompose_rotation(rotation: [f32; 4]) {
        let translation = [1.0, -2.0, 3.0];
        let scale = [1.0, 1.0, 1.0];
        test_decompose(translation, rotation, scale);
    }

    fn test_decompose_scale(scale: [f32; 3]) {
        let translation = [1.0, 2.0, 3.0];
        let rotation = rotate(1.0, 0.0, 0.0, PI / 2.0);
        test_decompose(translation, rotation, scale);
    }

    fn test_decompose_translation(translation: [f32; 3]) {
        let rotation = [0.0, 0.0, 0.0, 1.0];
        let scale = [1.0, 1.0, 1.0];
        test_decompose(translation, rotation, scale);
    }

    #[test]
    fn decompose_identity() {
        let translation = [0.0, 0.0, 0.0];
        let rotation = [0.0, 0.0, 0.0, 1.0];
        let scale = [1.0, 1.0, 1.0];
        test_decompose(translation, rotation, scale);
    }

    #[test]
    fn decompose_translation_unit_x() {
        let translation = [1.0, 0.0, 0.0];
        test_decompose_translation(translation);
    }

    #[test]
    fn decompose_translation_unit_y() {
        let translation = [0.0, 1.0, 0.0];
        test_decompose_translation(translation);
    }

    #[test]
    fn decompose_translation_unit_z() {
        let translation = [0.0, 0.0, 1.0];
        test_decompose_translation(translation);
    }

    #[test]
    fn decompose_translation_random0() {
        let translation = [1.0, -1.0, 1.0];
        test_decompose_translation(translation);
    }

    #[test]
    fn decompose_translation_random1() {
        let translation = [-1.0, -1.0, -1.0];
        test_decompose_translation(translation);
    }

    #[test]
    fn decompose_translation_random2() {
        let translation = [-10.0, 100000.0, -0.0001];
        test_decompose_translation(translation);
    }

    #[test]
    fn decompose_rotation_xaxis() {
        let rotation = rotate(1.0, 0.0, 0.0, PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_yaxis() {
        let rotation = rotate(0.0, 1.0, 0.0, PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_zaxis() {
        let rotation = rotate(0.0, 0.0, 1.0, PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_negative_xaxis() {
        let rotation = rotate(-1.0, 0.0, 0.0, PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_negative_yaxis() {
        let rotation = rotate(0.0, -1.0, 0.0, PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_negative_zaxis() {
        let rotation = rotate(0.0, 0.0, -1.0, PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_eighth_turn() {
        let rotation = rotate(1.0, 0.0, 0.0, PI / 4.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_negative_quarter_turn() {
        let rotation = rotate(0.0, 1.0, 0.0, -PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_half_turn() {
        let rotation = rotate(0.0, 0.0, 1.0, PI);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_zero_turn_xaxis() {
        let rotation = rotate(1.0, 0.0, 0.0, 0.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_zero_turn_yaxis() {
        let rotation = rotate(0.0, 1.0, 0.0, 0.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_zero_turn_zaxis() {
        let rotation = rotate(0.0, 0.0, 1.0, 0.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_full_turn() {
        let rotation = rotate(1.0, 0.0, 0.0, 2.0 * PI);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_random0() {
        let rotation = rotate(1.0, 1.0, 1.0, PI / 3.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_random1() {
        let rotation = rotate(1.0, -1.0, 1.0, -PI / 6.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_uniform_scale_up() {
        let scale = [100.0, 100.0, 100.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_uniform_scale_down() {
        let scale = [0.01, 0.01, 0.01];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_xscale_up() {
        let scale = [100.0, 1.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_xscale_down() {
        let scale = [0.001, 1.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_yscale_up() {
        let scale = [1.0, 100.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_yscale_down() {
        let scale = [1.0, 0.001, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_zscale_up() {
        let scale = [1.0, 1.0, 100.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_zscale_down() {
        let scale = [1.0, 1.0, 0.001];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_xscale_unit() {
        let scale = [-1.0, 1.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_xscale_up() {
        let scale = [-10.0, 1.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_xscale_down() {
        let scale = [-0.1, 1.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_yscale_unit() {
        let scale = [1.0, -1.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_yscale_up() {
        let scale = [1.0, -10.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_yscale_down() {
        let scale = [1.0, -0.1, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_zscale_unit() {
        let scale = [1.0, 1.0, -1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_zscale_up() {
        let scale = [1.0, 1.0, -10.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_zscale_down() {
        let scale = [1.0, 1.0, -0.1];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_up_sml() {
        let scale = [10.0, 100.0, 1000.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_up_mls() {
        let scale = [100.0, 1000.0, 10.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_up_lsm() {
        let scale = [1000.0, 10.0, 100.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_down_sml() {
        let scale = [0.01, 0.001, 0.0001];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_down_mls() {
        let scale = [0.001, 0.0001, 0.01];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_down_lsm() {
        let scale = [0.0001, 0.01, 0.01];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_unit_ls() {
        let scale = [1.0, 100000.0, 0.000001];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_ms_negative_unit() {
        let scale = [10.0, 0.1, -1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_ms_negative_up() {
        let scale = [10.0, 0.1, -10.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_ms_negative_down() {
        let scale = [10.0, 0.1, -0.1];
        test_decompose_scale(scale);
    }
}
