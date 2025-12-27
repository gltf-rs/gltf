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

use crate::mesh;

use crate::accessor::Iter;
use crate::Buffer;

/// XYZ vertex positions of type `[f32; 3]`.
#[cfg(not(feature = "KHR_mesh_quantization"))]
pub type ReadPositions<'a> = Iter<'a, [f32; 3]>;

/// XYZ vertex positions (quantized or unquantized).
#[cfg(feature = "KHR_mesh_quantization")]
pub enum ReadPositions<'a> {
    /// XYZ vertex positions of type `[i8; 3]`.
    I8(Iter<'a, [i8; 3]>, bool), // (iterator, normalized)
    /// XYZ vertex positions of type `[u8; 3]`.
    U8(Iter<'a, [u8; 3]>, bool), // (iterator, normalized)
    /// XYZ vertex positions of type `[i16; 3]`.
    I16(Iter<'a, [i16; 3]>, bool), // (iterator, normalized)
    /// XYZ vertex positions of type `[u16; 3]`.
    U16(Iter<'a, [u16; 3]>, bool), // (iterator, normalized)
    /// XYZ vertex positions of type `[f32; 3]`.
    F32(Iter<'a, [f32; 3]>),
}

#[cfg(feature = "KHR_mesh_quantization")]
impl<'a> ReadPositions<'a> {
    /// Returns the number of elements remaining in the iterator.
    pub fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::I8(iter, _) => iter.size_hint(),
            Self::U8(iter, _) => iter.size_hint(),
            Self::I16(iter, _) => iter.size_hint(),
            Self::U16(iter, _) => iter.size_hint(),
            Self::F32(iter) => iter.size_hint(),
        }
    }
}

#[cfg(feature = "KHR_mesh_quantization")]
impl<'a> Iterator for ReadPositions<'a> {
    type Item = [f32; 3];

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::I8(iter, normalized) => {
                iter.next().map(|[x, y, z]| {
                    if *normalized {
                        // BYTE normalized: f = max(c / 127.0, -1.0)
                        [
                            (x as f32 / 127.0).max(-1.0),
                            (y as f32 / 127.0).max(-1.0),
                            (z as f32 / 127.0).max(-1.0),
                        ]
                    } else {
                        [x as f32, y as f32, z as f32]
                    }
                })
            }
            Self::U8(iter, normalized) => {
                iter.next().map(|[x, y, z]| {
                    if *normalized {
                        // UNSIGNED_BYTE normalized: f = c / 255.0
                        [x as f32 / 255.0, y as f32 / 255.0, z as f32 / 255.0]
                    } else {
                        [x as f32, y as f32, z as f32]
                    }
                })
            }
            Self::I16(iter, normalized) => {
                iter.next().map(|[x, y, z]| {
                    if *normalized {
                        // SHORT normalized: f = max(c / 32767.0, -1.0)
                        [
                            (x as f32 / 32767.0).max(-1.0),
                            (y as f32 / 32767.0).max(-1.0),
                            (z as f32 / 32767.0).max(-1.0),
                        ]
                    } else {
                        [x as f32, y as f32, z as f32]
                    }
                })
            }
            Self::U16(iter, normalized) => {
                iter.next().map(|[x, y, z]| {
                    if *normalized {
                        // UNSIGNED_SHORT normalized: f = c / 65535.0
                        [x as f32 / 65535.0, y as f32 / 65535.0, z as f32 / 65535.0]
                    } else {
                        [x as f32, y as f32, z as f32]
                    }
                })
            }
            Self::F32(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.size_hint()
    }
}

#[cfg(feature = "KHR_mesh_quantization")]
impl<'a> ExactSizeIterator for ReadPositions<'a> {}

/// XYZ vertex normals of type `[f32; 3]`.
#[cfg(not(feature = "KHR_mesh_quantization"))]
pub type ReadNormals<'a> = Iter<'a, [f32; 3]>;

/// XYZ vertex normals (quantized or unquantized).
#[cfg(feature = "KHR_mesh_quantization")]
pub enum ReadNormals<'a> {
    /// XYZ vertex normals of type `[i8; 3]`.
    I8(Iter<'a, [i8; 3]>, bool), // (iterator, normalized)
    /// XYZ vertex normals of type `[i16; 3]`.
    I16(Iter<'a, [i16; 3]>, bool), // (iterator, normalized)
    /// XYZ vertex normals of type `[f32; 3]`.
    F32(Iter<'a, [f32; 3]>),
}

#[cfg(feature = "KHR_mesh_quantization")]
impl<'a> Iterator for ReadNormals<'a> {
    type Item = [f32; 3];

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::I8(iter, normalized) => {
                iter.next().map(|[x, y, z]| {
                    if *normalized {
                        // BYTE normalized: f = max(c / 127.0, -1.0)
                        [
                            (x as f32 / 127.0).max(-1.0),
                            (y as f32 / 127.0).max(-1.0),
                            (z as f32 / 127.0).max(-1.0),
                        ]
                    } else {
                        [x as f32, y as f32, z as f32]
                    }
                })
            }
            Self::I16(iter, normalized) => {
                iter.next().map(|[x, y, z]| {
                    if *normalized {
                        // SHORT normalized: f = max(c / 32767.0, -1.0)
                        [
                            (x as f32 / 32767.0).max(-1.0),
                            (y as f32 / 32767.0).max(-1.0),
                            (z as f32 / 32767.0).max(-1.0),
                        ]
                    } else {
                        [x as f32, y as f32, z as f32]
                    }
                })
            }
            Self::F32(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::I8(iter, _) => iter.size_hint(),
            Self::I16(iter, _) => iter.size_hint(),
            Self::F32(iter) => iter.size_hint(),
        }
    }
}

#[cfg(feature = "KHR_mesh_quantization")]
impl<'a> ExactSizeIterator for ReadNormals<'a> {}

/// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
/// sign value (-1 or +1) indicating the handedness of the tangent basis.
#[cfg(not(feature = "KHR_mesh_quantization"))]
pub type ReadTangents<'a> = Iter<'a, [f32; 4]>;

/// XYZW vertex tangents (quantized or unquantized) where the `w` component is a
/// sign value (-1 or +1) indicating the handedness of the tangent basis.
#[cfg(feature = "KHR_mesh_quantization")]
pub enum ReadTangents<'a> {
    /// XYZW vertex tangents of type `[i8; 4]`.
    I8(Iter<'a, [i8; 4]>, bool), // (iterator, normalized)
    /// XYZW vertex tangents of type `[i16; 4]`.
    I16(Iter<'a, [i16; 4]>, bool), // (iterator, normalized)
    /// XYZW vertex tangents of type `[f32; 4]`.
    F32(Iter<'a, [f32; 4]>),
}

#[cfg(feature = "KHR_mesh_quantization")]
impl<'a> Iterator for ReadTangents<'a> {
    type Item = [f32; 4];

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::I8(iter, normalized) => {
                iter.next().map(|[x, y, z, w]| {
                    if *normalized {
                        // BYTE normalized: f = max(c / 127.0, -1.0)
                        [
                            (x as f32 / 127.0).max(-1.0),
                            (y as f32 / 127.0).max(-1.0),
                            (z as f32 / 127.0).max(-1.0),
                            (w as f32 / 127.0).max(-1.0),
                        ]
                    } else {
                        [x as f32, y as f32, z as f32, w as f32]
                    }
                })
            }
            Self::I16(iter, normalized) => {
                iter.next().map(|[x, y, z, w]| {
                    if *normalized {
                        // SHORT normalized: f = max(c / 32767.0, -1.0)
                        [
                            (x as f32 / 32767.0).max(-1.0),
                            (y as f32 / 32767.0).max(-1.0),
                            (z as f32 / 32767.0).max(-1.0),
                            (w as f32 / 32767.0).max(-1.0),
                        ]
                    } else {
                        [x as f32, y as f32, z as f32, w as f32]
                    }
                })
            }
            Self::F32(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::I8(iter, _) => iter.size_hint(),
            Self::I16(iter, _) => iter.size_hint(),
            Self::F32(iter) => iter.size_hint(),
        }
    }
}

#[cfg(feature = "KHR_mesh_quantization")]
impl<'a> ExactSizeIterator for ReadTangents<'a> {}

#[cfg(feature = "KHR_mesh_quantization")]
impl<'a> Iterator for ReadPositionDisplacements<'a> {
    type Item = [f32; 3];

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::I8(iter, normalized) => {
                iter.next().map(|[x, y, z]| {
                    if *normalized {
                        // BYTE normalized: f = max(c / 127.0, -1.0)
                        [
                            (x as f32 / 127.0).max(-1.0),
                            (y as f32 / 127.0).max(-1.0),
                            (z as f32 / 127.0).max(-1.0),
                        ]
                    } else {
                        [x as f32, y as f32, z as f32]
                    }
                })
            }
            Self::I16(iter, normalized) => {
                iter.next().map(|[x, y, z]| {
                    if *normalized {
                        // SHORT normalized: f = max(c / 32767.0, -1.0)
                        [
                            (x as f32 / 32767.0).max(-1.0),
                            (y as f32 / 32767.0).max(-1.0),
                            (z as f32 / 32767.0).max(-1.0),
                        ]
                    } else {
                        [x as f32, y as f32, z as f32]
                    }
                })
            }
            Self::F32(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::I8(iter, _) => iter.size_hint(),
            Self::I16(iter, _) => iter.size_hint(),
            Self::F32(iter) => iter.size_hint(),
        }
    }
}

#[cfg(feature = "KHR_mesh_quantization")]
impl<'a> ExactSizeIterator for ReadPositionDisplacements<'a> {}

#[cfg(feature = "KHR_mesh_quantization")]
impl<'a> Iterator for ReadNormalDisplacements<'a> {
    type Item = [f32; 3];

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::I8(iter, normalized) => {
                iter.next().map(|[x, y, z]| {
                    if *normalized {
                        // BYTE normalized: f = max(c / 127.0, -1.0)
                        [
                            (x as f32 / 127.0).max(-1.0),
                            (y as f32 / 127.0).max(-1.0),
                            (z as f32 / 127.0).max(-1.0),
                        ]
                    } else {
                        [x as f32, y as f32, z as f32]
                    }
                })
            }
            Self::I16(iter, normalized) => {
                iter.next().map(|[x, y, z]| {
                    if *normalized {
                        // SHORT normalized: f = max(c / 32767.0, -1.0)
                        [
                            (x as f32 / 32767.0).max(-1.0),
                            (y as f32 / 32767.0).max(-1.0),
                            (z as f32 / 32767.0).max(-1.0),
                        ]
                    } else {
                        [x as f32, y as f32, z as f32]
                    }
                })
            }
            Self::F32(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::I8(iter, _) => iter.size_hint(),
            Self::I16(iter, _) => iter.size_hint(),
            Self::F32(iter) => iter.size_hint(),
        }
    }
}

#[cfg(feature = "KHR_mesh_quantization")]
impl<'a> ExactSizeIterator for ReadNormalDisplacements<'a> {}

#[cfg(feature = "KHR_mesh_quantization")]
impl<'a> Iterator for ReadTangentDisplacements<'a> {
    type Item = [f32; 3];

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::I8(iter, normalized) => {
                iter.next().map(|[x, y, z]| {
                    if *normalized {
                        // BYTE normalized: f = max(c / 127.0, -1.0)
                        [
                            (x as f32 / 127.0).max(-1.0),
                            (y as f32 / 127.0).max(-1.0),
                            (z as f32 / 127.0).max(-1.0),
                        ]
                    } else {
                        [x as f32, y as f32, z as f32]
                    }
                })
            }
            Self::I16(iter, normalized) => {
                iter.next().map(|[x, y, z]| {
                    if *normalized {
                        // SHORT normalized: f = max(c / 32767.0, -1.0)
                        [
                            (x as f32 / 32767.0).max(-1.0),
                            (y as f32 / 32767.0).max(-1.0),
                            (z as f32 / 32767.0).max(-1.0),
                        ]
                    } else {
                        [x as f32, y as f32, z as f32]
                    }
                })
            }
            Self::F32(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::I8(iter, _) => iter.size_hint(),
            Self::I16(iter, _) => iter.size_hint(),
            Self::F32(iter) => iter.size_hint(),
        }
    }
}

#[cfg(feature = "KHR_mesh_quantization")]
impl<'a> ExactSizeIterator for ReadTangentDisplacements<'a> {}

/// XYZ vertex position displacements of type `[f32; 3]`.
#[cfg(not(feature = "KHR_mesh_quantization"))]
pub type ReadPositionDisplacements<'a> = Iter<'a, [f32; 3]>;

/// XYZ vertex position displacements (quantized or unquantized).
#[cfg(feature = "KHR_mesh_quantization")]
pub enum ReadPositionDisplacements<'a> {
    /// XYZ vertex position displacements of type `[i8; 3]`.
    I8(Iter<'a, [i8; 3]>, bool), // (iterator, normalized)
    /// XYZ vertex position displacements of type `[i16; 3]`.
    I16(Iter<'a, [i16; 3]>, bool), // (iterator, normalized)
    /// XYZ vertex position displacements of type `[f32; 3]`.
    F32(Iter<'a, [f32; 3]>),
}

/// XYZ vertex normal displacements of type `[f32; 3]`.
#[cfg(not(feature = "KHR_mesh_quantization"))]
pub type ReadNormalDisplacements<'a> = Iter<'a, [f32; 3]>;

/// XYZ vertex normal displacements (quantized or unquantized).
#[cfg(feature = "KHR_mesh_quantization")]
pub enum ReadNormalDisplacements<'a> {
    /// XYZ vertex normal displacements of type `[i8; 3]`.
    I8(Iter<'a, [i8; 3]>, bool), // (iterator, normalized)
    /// XYZ vertex normal displacements of type `[i16; 3]`.
    I16(Iter<'a, [i16; 3]>, bool), // (iterator, normalized)
    /// XYZ vertex normal displacements of type `[f32; 3]`.
    F32(Iter<'a, [f32; 3]>),
}

/// XYZ vertex tangent displacements.
#[cfg(not(feature = "KHR_mesh_quantization"))]
pub type ReadTangentDisplacements<'a> = Iter<'a, [f32; 3]>;

/// XYZ vertex tangent displacements (quantized or unquantized).
#[cfg(feature = "KHR_mesh_quantization")]
pub enum ReadTangentDisplacements<'a> {
    /// XYZ vertex tangent displacements of type `[i8; 3]`.
    I8(Iter<'a, [i8; 3]>, bool), // (iterator, normalized)
    /// XYZ vertex tangent displacements of type `[i16; 3]`.
    I16(Iter<'a, [i16; 3]>, bool), // (iterator, normalized)
    /// XYZ vertex tangent displacements of type `[f32; 3]`.
    F32(Iter<'a, [f32; 3]>),
}

/// Vertex colors.
#[derive(Clone, Debug)]
pub enum ReadColors<'a> {
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
pub enum ReadIndices<'a> {
    /// Index data of type U8
    U8(Iter<'a, u8>),
    /// Index data of type U16
    U16(Iter<'a, u16>),
    /// Index data of type U32
    U32(Iter<'a, u32>),
}

/// Vertex joints.
#[derive(Clone, Debug)]
pub enum ReadJoints<'a> {
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
pub enum ReadTexCoords<'a> {
    /// UV texture co-ordinates of type `[u8; 2]>`.
    U8(Iter<'a, [u8; 2]>, bool), // (iterator, normalized)
    /// UV texture co-ordinates of type `[u16; 2]>`.
    U16(Iter<'a, [u16; 2]>, bool), // (iterator, normalized)
    /// UV texture co-ordinates of type `[f32; 2]`.
    F32(Iter<'a, [f32; 2]>),
    #[cfg(feature = "KHR_mesh_quantization")]
    /// UV texture co-ordinates of type `[i8; 2]>`.
    I8(Iter<'a, [i8; 2]>, bool), // (iterator, normalized)
    #[cfg(feature = "KHR_mesh_quantization")]
    /// UV texture co-ordinates of type `[i16; 2]>`.
    I16(Iter<'a, [i16; 2]>, bool), // (iterator, normalized)
}

/// Weights.
#[derive(Clone, Debug)]
pub enum ReadWeights<'a> {
    /// Weights of type `[u8; 4]`.
    U8(Iter<'a, [u8; 4]>),
    /// Weights of type `[u16; 4]`.
    U16(Iter<'a, [u16; 4]>),
    /// Weights of type `[f32; 4]`.
    F32(Iter<'a, [f32; 4]>),
}

/// Morph targets.
#[derive(Clone, Debug)]
pub struct ReadMorphTargets<'a, 's, F>
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    pub(crate) index: usize,
    pub(crate) reader: mesh::Reader<'a, 's, F>,
}

impl<'a, 's, F> ExactSizeIterator for ReadMorphTargets<'a, 's, F> where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>
{
}

impl<'a, 's, F> Iterator for ReadMorphTargets<'a, 's, F>
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    type Item = (
        Option<ReadPositionDisplacements<'s>>,
        Option<ReadNormalDisplacements<'s>>,
        Option<ReadTangentDisplacements<'s>>,
    );
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.reader
            .primitive
            .morph_targets()
            .nth(self.index - 1)
            .map(|morph_target| {
                #[cfg(feature = "KHR_mesh_quantization")]
                let positions = morph_target.positions().and_then(|accessor| {
                    let normalized = accessor.normalized();
                    match accessor.data_type() {
                        json::accessor::ComponentType::I8 => {
                            // For morph targets: POSITION with BYTE can be normalized or unnormalized
                            Iter::new(accessor, self.reader.get_buffer_data.clone())
                                .map(|iter| ReadPositionDisplacements::I8(iter, normalized))
                        }
                        json::accessor::ComponentType::I16 => {
                            // For morph targets: POSITION with SHORT can be normalized or unnormalized
                            Iter::new(accessor, self.reader.get_buffer_data.clone())
                                .map(|iter| ReadPositionDisplacements::I16(iter, normalized))
                        }
                        json::accessor::ComponentType::F32 => {
                            Iter::new(accessor, self.reader.get_buffer_data.clone())
                                .map(ReadPositionDisplacements::F32)
                        }
                        _ => None, // Invalid component types for morph target positions
                    }
                });
                #[cfg(not(feature = "KHR_mesh_quantization"))]
                let positions = morph_target
                    .positions()
                    .and_then(|accessor| Iter::new(accessor, self.reader.get_buffer_data.clone()));

                #[cfg(feature = "KHR_mesh_quantization")]
                let normals = morph_target.normals().and_then(|accessor| {
                    let normalized = accessor.normalized();
                    match accessor.data_type() {
                        json::accessor::ComponentType::I8 => {
                            // For morph targets: NORMAL with BYTE must be normalized
                            if !normalized {
                                return None;
                            }
                            Iter::new(accessor, self.reader.get_buffer_data.clone())
                                .map(|iter| ReadNormalDisplacements::I8(iter, normalized))
                        }
                        json::accessor::ComponentType::I16 => {
                            // For morph targets: NORMAL with SHORT must be normalized
                            if !normalized {
                                return None;
                            }
                            Iter::new(accessor, self.reader.get_buffer_data.clone())
                                .map(|iter| ReadNormalDisplacements::I16(iter, normalized))
                        }
                        json::accessor::ComponentType::F32 => {
                            Iter::new(accessor, self.reader.get_buffer_data.clone())
                                .map(ReadNormalDisplacements::F32)
                        }
                        _ => None, // Invalid component types for morph target normals
                    }
                });
                #[cfg(not(feature = "KHR_mesh_quantization"))]
                let normals = morph_target
                    .normals()
                    .and_then(|accessor| Iter::new(accessor, self.reader.get_buffer_data.clone()));

                #[cfg(feature = "KHR_mesh_quantization")]
                let tangents = morph_target.tangents().and_then(|accessor| {
                    let normalized = accessor.normalized();
                    match accessor.data_type() {
                        json::accessor::ComponentType::I8 => {
                            // For morph targets: TANGENT with BYTE must be normalized
                            if !normalized {
                                return None;
                            }
                            Iter::new(accessor, self.reader.get_buffer_data.clone())
                                .map(|iter| ReadTangentDisplacements::I8(iter, normalized))
                        }
                        json::accessor::ComponentType::I16 => {
                            // For morph targets: TANGENT with SHORT must be normalized
                            if !normalized {
                                return None;
                            }
                            Iter::new(accessor, self.reader.get_buffer_data.clone())
                                .map(|iter| ReadTangentDisplacements::I16(iter, normalized))
                        }
                        json::accessor::ComponentType::F32 => {
                            Iter::new(accessor, self.reader.get_buffer_data.clone())
                                .map(ReadTangentDisplacements::F32)
                        }
                        _ => None, // Invalid component types for morph target tangents
                    }
                });
                #[cfg(not(feature = "KHR_mesh_quantization"))]
                let tangents = morph_target
                    .tangents()
                    .and_then(|accessor| Iter::new(accessor, self.reader.get_buffer_data.clone()));

                (positions, normals, tangents)
            })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.reader.primitive.morph_targets().size_hint()
    }
}

impl<'a> ReadColors<'a> {
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

impl<'a> ReadIndices<'a> {
    /// Reinterpret indices as u32, which can fit any possible index.
    pub fn into_u32(self) -> self::indices::CastingIter<'a, self::indices::U32> {
        self::indices::CastingIter::new(self)
    }
}

impl<'a> ReadJoints<'a> {
    /// Reinterpret joints as u16, which can fit any possible joint.
    pub fn into_u16(self) -> self::joints::CastingIter<'a, self::joints::U16> {
        self::joints::CastingIter::new(self)
    }
}

impl<'a> ReadTexCoords<'a> {
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

impl<'a> ReadWeights<'a> {
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
