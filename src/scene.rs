use crate::{camera, mesh, scene, skin, Extras, Index, UnrecognizedExtensions};

/// Support for the `KHR_lights_punctual` extension.
pub mod khr_lights_punctual {
    use crate::validation::{Error, Validate};
    use crate::{Index, Path, Root};

    /// Introduces a light source to a scene node.
    #[derive(
        Clone,
        Debug,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Stub,
        gltf_derive::Validate,
    )]
    pub struct LightInstance {
        /// The index of the light referenced by this node.
        pub light: Index<Light>,

        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }

    /// Specifies the light type.
    #[derive(
        Clone, Copy, Debug, serde_derive::Deserialize, Eq, Hash, PartialEq, serde_derive::Serialize,
    )]
    pub enum Type {
        /// Directional lights act as though they are infinitely far away and emit light in
        /// the direction of the local -z axis. This light type inherits the orientation of
        /// the node that it belongs to; position and scale are ignored except for their
        /// effect on the inherited node orientation. Because it is at an infinite distance,
        /// the light is not attenuated. Its intensity is defined in lumens per metre squared,
        /// or lux (lm/m^2).
        #[serde(rename = "directional")]
        Directional = 1,

        /// Point lights emit light in all directions from their position in space; rotation
        /// and scale are ignored except for their effect on the inherited node position. The
        /// brightness of the light attenuates in a physically correct manner as distance
        /// increases from the light's position (i.e. brightness goes like the inverse square
        /// of the distance). Point light intensity is defined in candela, which is lumens per
        /// square radian (lm/sr)."
        #[serde(rename = "point")]
        Point,

        /// Spot lights emit light in a cone in the direction of the local -z axis. The angle
        /// and falloff of the cone is defined using two numbers, the innerConeAngle and outer
        /// ConeAngle. As with point lights, the brightness also attenuates in a physically
        /// correct manner as distance increases from the light's position (i.e. brightness
        /// goes like the inverse square of the distance). Spot light intensity refers to the
        /// brightness inside the innerConeAngle (and at the location of the light) and is
        /// defined in candela, which is lumens per square radian (lm/sr). Engines that don't
        /// support two angles for spotlights should use outerConeAngle as the spotlight angle
        /// (leaving innerConeAngle to implicitly be 0).
        #[serde(rename = "spot")]
        Spot,
    }

    impl Validate for Type {}

    impl crate::Stub for Type {
        fn stub() -> Self {
            Self::Directional
        }
    }

    /// A directional, point, or spot light placeable within a scene.
    #[derive(Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Stub)]
    pub struct Light {
        /// Color of the light source.
        #[gltf(default = [1.0, 1.0, 1.0])]
        pub color: [f32; 3],
        /// Intensity of the light source. `point` and `spot` lights use luminous intensity
        /// in candela (lm/sr) while `directional` lights use illuminance in lux (lm/m^2).
        #[gltf(default = 1.0)]
        pub intensity: f32,

        /// Optional user-defined name for this object.
        pub name: Option<String>,

        /// A distance cutoff at which the light's intensity may be considered to have reached
        /// zero.
        pub range: Option<f32>,

        /// Spot light parameters.
        pub spot: Option<Spot>,

        /// Specifies the light type.
        #[serde(rename = "type")]
        pub type_: Type,

        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }

    impl Validate for Light {
        fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
        where
            P: Fn() -> Path,
            R: FnMut(&dyn Fn() -> Path, Error),
        {
            if self.type_ == Type::Spot && self.spot.is_none() {
                report(&|| path().field("spot"), Error::Missing);
            }

            self.type_.validate(root, || path().field("type"), report);
        }
    }

    /// Spot light parameters.
    #[derive(
        Clone,
        Debug,
        gltf_derive::Default,
        gltf_derive::Deserialize,
        gltf_derive::Serialize,
        gltf_derive::Validate,
    )]
    pub struct Spot {
        /// Angle in radians from centre of spotlight where falloff begins.
        #[gltf(default)]
        pub inner_cone_angle: f32,

        /// Angle in radians from centre of spotlight where falloff ends.
        #[gltf(default = std::f32::consts::FRAC_PI_4)]
        pub outer_cone_angle: f32,

        /// Unrecognized extension data.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,

        /// Optional application specific data.
        pub extras: Option<crate::Extras>,
    }
}

/// A node in the node hierarchy.
///
/// When the node contains `skin`, all
/// `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes.
/// A node can have either a `matrix` or any combination of
/// `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted
/// to matrices and postmultiplied in the `T * R * S` order to compose the
/// transformation matrix; first the scale is applied to the vertices, then the
/// rotation, and then the translation. If none are provided, the transform is the
/// identity. When a node is targeted for animation (referenced by an
/// animation.channel.target), only TRS properties may be present; `matrix` will not
/// be present.
#[derive(
    Clone, Debug, Default, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Validate,
)]
pub struct Node {
    /// The index of the camera referenced by this node.
    pub camera: Option<Index<camera::Camera>>,

    /// The indices of this node's children.
    pub children: Vec<Index<scene::Node>>,

    /// 4x4 column-major transformation matrix.
    pub matrix: Option<[f32; 16]>,

    /// The index of the mesh in this node.
    pub mesh: Option<Index<mesh::Mesh>>,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The node's unit quaternion rotation in the order `[x, y, z, w]`, where `w` is
    /// the scalar component.
    pub rotation: Option<[f32; 4]>,

    /// The node's non-uniform scale.
    pub scale: Option<[f32; 3]>,

    /// The node's translation.
    pub translation: Option<[f32; 3]>,

    /// The index of the skin referenced by this node.
    pub skin: Option<Index<skin::Skin>>,

    /// The weights of the instantiated Morph Target. Number of elements must match
    /// the number of Morph Targets of used mesh.
    pub weights: Vec<f32>,

    /// Support for the `KHR_lights_punctual` extension.
    #[gltf(extension = "KHR_lights_punctual")]
    pub light: Option<khr_lights_punctual::LightInstance>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

/// The root `Node`s of a scene.
#[derive(
    Clone,
    Debug,
    gltf_derive::Deserialize,
    gltf_derive::Serialize,
    gltf_derive::Stub,
    gltf_derive::Validate,
)]
pub struct Scene {
    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The indices of each root node.
    pub nodes: Vec<Index<Node>>,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}
