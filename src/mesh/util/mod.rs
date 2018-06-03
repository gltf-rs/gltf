/// Casting iterator adapters for colors.
pub mod colors;

/// Casting iterator adapters for vertex indices.
pub mod indices;

/// Casting iterator adapters for joint indices.
pub mod joints;

/// Casting iterator adapters for texture co-ordinates.
pub mod tex_coords;

/// Casting iterator adapters for node weights.
pub mod weights;

use accessor::Iter;

/// XYZ vertex positions of type `[f32; 3]`.
pub type IterPositions<'a> = Iter<'a, [f32; 3]>;

/// XYZ vertex normals of type `[f32; 3]`.
pub type IterNormals<'a> = Iter<'a, [f32; 3]>;

/// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
/// sign value (-1 or +1) indicating the handedness of the tangent basis.
pub type IterTangents<'a> = Iter<'a, [f32; 4]>;

/// Vertex colors.
#[derive(Clone, Debug)]
pub enum IterColors<'a> {
    /// RGB vertex color of type `[u8; 3]>`.
    RgbU8(Iter<'a, [u8; 3]>),
    /// RGB vertex color of type `[u16; 3]>`.
    RgbU16(Iter<'a, [u16; 3]>),
    /// RGB vertex color of type `[f32; 3]`.
    RgbF32(Iter<'a, [f32; 3]>),
    /// RGBA vertex color of type `[u8; 4]>`.
    RgbaU8(Iter<'a, [u8; 4]>),
    /// RGBA vertex color of type `[u16; 4]>`.
    RgbaU16(Iter<'a, [u16; 4]>),
    /// RGBA vertex color of type `[f32; 4]`.
    RgbaF32(Iter<'a, [f32; 4]>),
}

/// Index data.
#[derive(Clone, Debug)]
pub enum IterIndices<'a> {
    /// Index data of type U8
    U8(Iter<'a, u8>),
    /// Index data of type U16
    U16(Iter<'a, u16>),
    /// Index data of type U32
    U32(Iter<'a, u32>),
}

/// Vertex joints.
#[derive(Clone, Debug)]
pub enum IterJoints<'a> {
    /// Joints of type `[u8; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U8(Iter<'a, [u8; 4]>),
    /// Joints of type `[u16; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U16(Iter<'a, [u16; 4]>),
}

/// UV texture co-ordinates.
#[derive(Clone, Debug)]
pub enum IterTexCoords<'a> {
    /// UV texture co-ordinates of type `[u8; 2]>`.
    U8(Iter<'a, [u8; 2]>),
    /// UV texture co-ordinates of type `[u16; 2]>`.
    U16(Iter<'a, [u16; 2]>),
    /// UV texture co-ordinates of type `[f32; 2]`.
    F32(Iter<'a, [f32; 2]>),
}

/// Weights.
#[derive(Clone, Debug)]
pub enum IterWeights<'a> {
    /// Weights of type `[u8; 4]`.
    U8(Iter<'a, [u8; 4]>),
    /// Weights of type `[u16; 4]`.
    U16(Iter<'a, [u16; 4]>),
    /// Weights of type `[f32; 4]`.
    F32(Iter<'a, [f32; 4]>),
}

impl<'a> IterColors<'a> {
    /// Reinterpret colors as RGB u8, discarding alpha, if present.  Lossy if
    /// the underlying iterator yields u16, f32 or any RGBA.
    pub fn into_rgb_u8(self) -> self::colors::CastingIter<'a, self::colors::RgbU8> {
        self::colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGB u16, discarding alpha, if present.  Lossy if
    /// the underlying iterator yields f32 or any RGBA.
    pub fn into_rgb_u16(self) -> self::colors::CastingIter<'a, self::colors::RgbU16> {
        self::colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGB f32, discarding alpha, if present.  Lossy if
    /// the underlying iterator yields u16 or any RGBA.
    pub fn into_rgb_f32(self) -> self::colors::CastingIter<'a, self::colors::RgbF32> {
        self::colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGBA u8, with default alpha 255.  Lossy if the
    /// underlying iterator yields u16 or f32.
    pub fn into_rgba_u8(self) -> self::colors::CastingIter<'a, self::colors::RgbaU8> {
        self::colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGBA u16, with default alpha 65535.  Lossy if the
    /// underlying iterator yields f32.
    pub fn into_rgba_u16(self) -> self::colors::CastingIter<'a, self::colors::RgbaU16> {
        self::colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGBA f32, with default alpha 1.0.  Lossy if the
    /// underlying iterator yields u16.
    pub fn into_rgba_f32(self) -> self::colors::CastingIter<'a, self::colors::RgbaF32> {
        self::colors::CastingIter::new(self)
    }
}

impl<'a> IterIndices<'a> {
    /// Reinterpret indices as u32, which can fit any possible index.
    pub fn into_u32(self) -> self::indices::CastingIter<'a, self::indices::U32> {
        self::indices::CastingIter::new(self)
    }
}

impl<'a> IterJoints<'a> {
    /// Reinterpret joints as u16, which can fit any possible joint.
    pub fn into_u16(self) -> self::joints::CastingIter<'a, self::joints::U16> {
        self::joints::CastingIter::new(self)
    }
}

impl<'a> IterTexCoords<'a> {
    /// Reinterpret texture coordinates as u8.  Lossy if the underlying iterator
    /// yields u16 or f32.
    pub fn into_u8(self) -> self::tex_coords::CastingIter<'a, self::tex_coords::U8> {
        self::tex_coords::CastingIter::new(self)
    }

    /// Reinterpret texture coordinates as u16.  Lossy if the underlying
    /// iterator yields f32.
    pub fn into_u16(self) -> self::tex_coords::CastingIter<'a, self::tex_coords::U16> {
        self::tex_coords::CastingIter::new(self)
    }

    /// Reinterpret texture coordinates as f32.  Lossy if the underlying
    /// iterator yields u16.
    pub fn into_f32(self) -> self::tex_coords::CastingIter<'a, self::tex_coords::F32> {
        self::tex_coords::CastingIter::new(self)
    }
}

impl<'a> IterWeights<'a> {
    /// Reinterpret weights as u8.  Lossy if the underlying iterator yields u16
    /// or f32.
    pub fn into_u8(self) -> self::weights::CastingIter<'a, self::weights::U8> {
        self::weights::CastingIter::new(self)
    }

    /// Reinterpret weights as u16.  Lossy if the underlying iterator yields
    /// f32.
    pub fn into_u16(self) -> self::weights::CastingIter<'a, self::weights::U16> {
        self::weights::CastingIter::new(self)
    }

    /// Reinterpret weights as f32.  Lossy if the underlying iterator yields
    /// u16.
    pub fn into_f32(self) -> self::weights::CastingIter<'a, self::weights::F32> {
        self::weights::CastingIter::new(self)
    }
}
