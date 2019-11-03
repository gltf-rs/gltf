use gltf_json::Extras;
use crate::Document;

/// A light in the scene.
pub struct Light<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::extensions::scene::khr_lights_punctual::Light,
}

impl<'a> Light<'a> {
    /// Constructs a `Light`.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a json::extensions::scene::khr_lights_punctual::Light) -> Self {
        Self {
            document,
            index,
            json,
        }
    }

    /// Color of the light source.
    pub fn color(&self) -> [f32; 3] {
        self.json.color.clone()
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a Extras {
        &self.json.extras
    }

    /// Intensity of the light source. `point` and `spot` lights use luminous intensity
    /// in candela (lm/sr) while `directional` lights use illuminance in lux (lm/m^2).
    pub fn intensity(&self) -> f32 {
        self.json.intensity
    }

    /// A distance cutoff at which the light's intensity may be considered to have reached
    /// zero.
    pub fn range(&self) -> Option<f32> {
        self.json.range
    }

    /// Specifies the light subcategory.
    pub fn kind(&self) -> Kind {
        use json::extensions::scene::khr_lights_punctual::Type;
        match self.json.type_.unwrap() {
            Type::Directional => Kind::Directional,
            Type::Point => Kind::Point,
            Type::Spot => {
                let args = self.json.spot.as_ref().unwrap();
                Kind::Spot {
                    inner_cone_angle: args.inner_cone_angle,
                    outer_cone_angle: args.outer_cone_angle,
                }
            },
        }
    }
}

/// Light subcategory.
pub enum Kind {
    /// Directional lights are light sources that act as though they are infinitely far away
    /// and emit light in the direction of the local -z axis. This light type inherits the
    /// orientation of the node that it belongs to; position and scale are ignored except for
    /// their effect on the inherited node orientation. Because it is at an infinite distance,
    /// the light is not attenuated. Its intensity is defined in lumens per metre squared, or
    /// lux (lm/m2).
    Directional,

    /// Point lights emit light in all directions from their position in space; rotation and
    /// scale are ignored except for their effect on the inherited node position. The
    /// brightness of the light attenuates in a physically correct manner as distance
    /// increases from the light's position (i.e. brightness goes like the inverse square of
    /// the distance). Point light intensity is defined in candela, which is lumens per square
    /// radian (lm/sr).
    Point,

    /// Spot lights emit light in a cone in the direction of the local -z axis. The angle and
    /// falloff of the cone is defined using two numbers, the `inner_cone_angle` and
    /// `outer_cone_angle`. As with point lights, the brightness also attenuates in a
    /// physically correct manner as distance increases from the light's position (i.e.
    /// brightness goes like the inverse square of the distance). Spot light intensity refers
    /// to the brightness inside the `inner_cone_angle` (and at the location of the light) and
    /// is defined in candela, which is lumens per square radian (lm/sr). Engines that don't
    /// support two angles for spotlights should use `outer_cone_angle` as the spotlight angle
    /// (leaving `inner_cone_angle` to implicitly be 0).
    ///
    /// A spot light's position and orientation are inherited from its node transform.
    /// Inherited scale does not affect cone shape, and is ignored except for its effect on
    /// position and orientation.
    Spot {
        /// Angle in radians from centre of spotlight where falloff begins.
        inner_cone_angle: f32,

        /// Angle in radians from centre of spotlight where falloff ends.
        outer_cone_angle: f32,
    },
}
