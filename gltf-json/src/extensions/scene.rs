use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "extensions")]
use serde_json::{Map, Value};

/// A node in the node hierarchy.  When the node contains `skin`, all
/// `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes.
/// A node can have either a `matrix` or any combination of
/// `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted
/// to matrices and postmultiplied in the `T * R * S` order to compose the
/// transformation matrix; first the scale is applied to the vertices, then the
/// rotation, and then the translation. If none are provided, the transform is the
/// identity. When a node is targeted for animation (referenced by an
/// animation.channel.target), only TRS properties may be present; `matrix` will not
/// be present.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Node {
    #[cfg(feature = "KHR_lights_punctual")]
    #[serde(
        default,
        rename = "KHR_lights_punctual",
        skip_serializing_if = "Option::is_none"
    )]
    pub khr_lights_punctual: Option<khr_lights_punctual::KhrLightsPunctual>,

    #[cfg(feature = "extensions")]
    #[serde(default, flatten)]
    pub others: Map<String, Value>,
}

#[cfg(feature = "KHR_lights_punctual")]
pub mod khr_lights_punctual {
    use crate::validation::{Checked, Error};
    use crate::{Extras, Index, Path, Root};
    use gltf_derive::Validate;
    use serde::{de, ser};
    use serde_derive::{Deserialize, Serialize};
    use std::fmt;

    /// All valid light types.
    pub const VALID_TYPES: &[&str] = &["directional", "point", "spot"];

    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    pub struct KhrLightsPunctual {
        pub light: Index<Light>,
    }

    /// Specifies the light type.
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Type {
        /// Directional lights act as though they are infinitely far away and emit light in
        /// the direction of the local -z axis. This light type inherits the orientation of
        /// the node that it belongs to; position and scale are ignored except for their
        /// effect on the inherited node orientation. Because it is at an infinite distance,
        /// the light is not attenuated. Its intensity is defined in lumens per metre squared,
        /// or lux (lm/m^2).
        Directional = 1,

        /// Point lights emit light in all directions from their position in space; rotation
        /// and scale are ignored except for their effect on the inherited node position. The
        /// brightness of the light attenuates in a physically correct manner as distance
        /// increases from the light's position (i.e. brightness goes like the inverse square
        /// of the distance). Point light intensity is defined in candela, which is lumens per
        /// square radian (lm/sr)."
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
        Spot,
    }

    #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
    #[gltf(validate_hook = "light_validate_hook")]
    pub struct Light {
        /// Color of the light source.
        #[serde(default = "color_default")]
        pub color: [f32; 3],

        /// Extension specific data.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub extensions: Option<std::boxed::Box<serde_json::value::RawValue>>,

        /// Optional application specific data.
        #[serde(default)]
        #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
        #[cfg_attr(not(feature = "extras"), serde(skip_serializing))]
        pub extras: Extras,

        /// Intensity of the light source. `point` and `spot` lights use luminous intensity
        /// in candela (lm/sr) while `directional` lights use illuminance in lux (lm/m^2).
        #[serde(default = "intensity_default")]
        pub intensity: f32,

        /// Optional user-defined name for this object.
        #[cfg(feature = "names")]
        #[cfg_attr(feature = "names", serde(skip_serializing_if = "Option::is_none"))]
        pub name: Option<String>,

        /// A distance cutoff at which the light's intensity may be considered to have reached
        /// zero.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub range: Option<f32>,

        /// Spot light parameters.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub spot: Option<Spot>,

        /// Specifies the light type.
        #[serde(rename = "type")]
        pub type_: Checked<Type>,
    }

    fn light_validate_hook<P, R>(light: &Light, _root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        if let Checked::Valid(ty) = light.type_.as_ref() {
            if *ty == Type::Spot && light.spot.is_none() {
                report(&|| path().field("spot"), Error::Missing);
            }
        }
    }

    fn color_default() -> [f32; 3] {
        [1.0, 1.0, 1.0]
    }

    fn intensity_default() -> f32 {
        1.0
    }

    /// Spot light parameters.
    #[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Spot {
        /// Angle in radians from centre of spotlight where falloff begins.
        #[serde(default)]
        pub inner_cone_angle: f32,

        /// Angle in radians from centre of spotlight where falloff ends.
        #[serde(default = "outer_cone_angle_default")]
        pub outer_cone_angle: f32,
    }

    fn outer_cone_angle_default() -> f32 {
        std::f32::consts::FRAC_PI_4
    }

    impl<'de> de::Deserialize<'de> for Checked<Type> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct Visitor;
            impl<'de> de::Visitor<'de> for Visitor {
                type Value = Checked<Type>;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "any of: {:?}", VALID_TYPES)
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    use self::Type::*;
                    use crate::validation::Checked::*;
                    Ok(match value {
                        "directional" => Valid(Directional),
                        "point" => Valid(Point),
                        "spot" => Valid(Spot),
                        _ => Invalid,
                    })
                }
            }
            deserializer.deserialize_str(Visitor)
        }
    }

    impl ser::Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_str(match *self {
                Type::Directional => "directional",
                Type::Point => "point",
                Type::Spot => "spot",
            })
        }
    }
}

#[cfg(feature = "KHR_materials_variants")]
pub mod khr_materials_variants {
    use crate::validation::{Error, Validate};
    use crate::{Path, Root};
    use serde_derive::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Variant {
        pub name: String,
    }

    impl Validate for Variant {
        fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
        where
            P: Fn() -> Path,
            R: FnMut(&dyn Fn() -> Path, Error),
        {
            self.name.validate(root, || path().field("name"), report);
        }
    }
}

/// The root `Node`s of a scene.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Scene {
    #[cfg(feature = "extensions")]
    #[serde(default, flatten)]
    pub others: Map<String, Value>,
}
