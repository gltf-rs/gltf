use crate::validation::{Error, Validate};
use crate::{accessor, scene, Extras, Index, Path, Root, Stub, UnrecognizedExtensions};

/// Specifies an interpolation algorithm.
#[derive(
    Clone, Copy, Debug, Default, serde_derive::Deserialize, Eq, PartialEq, serde_derive::Serialize,
)]
pub enum Interpolation {
    /// Linear interpolation.
    ///
    /// The animated values are linearly interpolated between keyframes.
    /// When targeting a rotation, spherical linear interpolation (slerp) should be
    /// used to interpolate quaternions. The number output of elements must equal
    /// the number of input elements.
    #[default]
    #[serde(rename = "LINEAR")]
    Linear = 1,

    /// Step interpolation.
    ///
    /// The animated values remain constant to the output of the first keyframe,
    /// until the next keyframe. The number of output elements must equal the number
    /// of input elements.
    #[serde(rename = "STEP")]
    Step,

    /// Cubic spline interpolation.
    ///
    /// The animation's interpolation is computed using a cubic spline with specified
    /// tangents. The number of output elements must equal three times the number of
    /// input elements. For each input element, the output stores three elements, an
    /// in-tangent, a spline vertex, and an out-tangent. There must be at least two
    /// keyframes when using this interpolation
    #[serde(rename = "CUBICSPLINE")]
    CubicSpline,
}
impl Validate for Interpolation {}

/// Specifies a property to animate.
#[derive(Clone, Copy, Debug, serde_derive::Deserialize, Eq, PartialEq, serde_derive::Serialize)]
pub enum Property {
    /// XYZ translation vector.
    #[serde(rename = "translation")]
    Translation = 1,
    /// XYZW rotation quaternion.
    #[serde(rename = "rotation")]
    Rotation,
    /// XYZ scale vector.
    #[serde(rename = "scale")]
    Scale,
    /// Weights of morph targets.
    #[serde(rename = "weights")]
    MorphTargetWeights,
}

impl Validate for Property {}

impl Stub for Property {
    fn stub() -> Self {
        Self::Translation
    }
}

/// A keyframe animation.
#[derive(Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Stub)]
pub struct Animation {
    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,

    /// An array of channels, each of which targets an animation's sampler at a
    /// node's property.
    ///
    /// Different channels of the same animation must not have equal targets.
    pub channels: Vec<Channel>,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// An array of samplers that combine input and output accessors with an
    /// interpolation algorithm to define a keyframe graph (but not its target).
    pub samplers: Vec<Sampler>,
}

/// Targets an animation's sampler at a node's property.
#[derive(Clone, Debug, gltf_derive::Deserialize, gltf_derive::Serialize, gltf_derive::Stub)]
pub struct Channel {
    /// The index of a sampler in this animation used to compute the value for the
    /// target.
    pub sampler: Index<Sampler>,

    /// The index of the node and TRS property to target.
    pub target: Target,

    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,
}

/// The index of the node and TRS property that an animation channel targets.
#[derive(
    Clone,
    Debug,
    gltf_derive::Deserialize,
    gltf_derive::Serialize,
    gltf_derive::Validate,
    gltf_derive::Stub,
)]
pub struct Target {
    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,

    /// The index of the node to target.
    pub node: Index<scene::Node>,

    /// The name of the node's property to modify or the 'weights' of the
    /// morph targets it instantiates.
    pub path: Property,
}

/// Defines a keyframe graph but not its target.
#[derive(
    Clone,
    Debug,
    gltf_derive::Deserialize,
    gltf_derive::Serialize,
    gltf_derive::Stub,
    gltf_derive::Validate,
)]
pub struct Sampler {
    /// Unrecognized extension data.
    pub unrecognized_extensions: UnrecognizedExtensions,

    /// Optional application specific data.
    pub extras: Option<Extras>,

    /// The index of an accessor containing keyframe input values, e.g., time.
    pub input: Index<accessor::Accessor>,

    /// The interpolation algorithm.
    #[serde(default)]
    pub interpolation: Interpolation,

    /// The index of an accessor containing keyframe output values.
    pub output: Index<accessor::Accessor>,
}

impl Validate for Animation {
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        self.samplers
            .validate(root, || path().field("samplers"), report);
        for (index, channel) in self.channels.iter().enumerate() {
            if channel.sampler.value() >= self.samplers.len() {
                let path = || path().field("channels").index(index).field("sampler");
                report(&path, Error::IndexOutOfBounds);
            }
        }
    }
}
