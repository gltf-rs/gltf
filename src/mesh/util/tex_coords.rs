use std::marker::PhantomData;

use crate::Normalize;

use super::ReadTexCoords;

/// Casting iterator for `TexCoords`.
#[derive(Clone, Debug)]
pub struct CastingIter<'a, T>(ReadTexCoords<'a>, PhantomData<T>);

/// Type which describes how to cast any texture coordinate into pair of u8.
#[derive(Clone, Debug)]
pub struct U8;

/// Type which describes how to cast any texture coordinate into pair of u16.
#[derive(Clone, Debug)]
pub struct U16;

/// Type which describes how to cast any texture coordinate into pair of f32.
#[derive(Clone, Debug)]
pub struct F32;

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Output;

    /// Cast from u8 pair.
    fn cast_u8(x: [u8; 2], normalized: bool) -> Self::Output;

    /// Cast from u16 pair.
    fn cast_u16(x: [u16; 2], normalized: bool) -> Self::Output;

    /// Cast from f32 pair.
    fn cast_f32(x: [f32; 2]) -> Self::Output;

    #[cfg(feature = "KHR_mesh_quantization")]
    /// Cast from i8 pair.
    fn cast_i8(x: [i8; 2], normalized: bool) -> Self::Output;

    #[cfg(feature = "KHR_mesh_quantization")]
    /// Cast from i16 pair.
    fn cast_i16(x: [i16; 2], normalized: bool) -> Self::Output;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: ReadTexCoords<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `TexCoords` object.
    pub fn unwrap(self) -> ReadTexCoords<'a> {
        self.0
    }
}

impl<'a, A: Cast> ExactSizeIterator for CastingIter<'a, A> {}
impl<'a, A: Cast> Iterator for CastingIter<'a, A> {
    type Item = A::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            ReadTexCoords::U8(ref mut i, normalized) => i.next().map(|v| A::cast_u8(v, normalized)),
            ReadTexCoords::U16(ref mut i, normalized) => {
                i.next().map(|v| A::cast_u16(v, normalized))
            }
            ReadTexCoords::F32(ref mut i) => i.next().map(A::cast_f32),
            #[cfg(feature = "KHR_mesh_quantization")]
            ReadTexCoords::I8(ref mut i, normalized) => i.next().map(|v| A::cast_i8(v, normalized)),
            #[cfg(feature = "KHR_mesh_quantization")]
            ReadTexCoords::I16(ref mut i, normalized) => {
                i.next().map(|v| A::cast_i16(v, normalized))
            }
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            ReadTexCoords::U8(ref mut i, normalized) => i.nth(x).map(|v| A::cast_u8(v, normalized)),
            ReadTexCoords::U16(ref mut i, normalized) => {
                i.nth(x).map(|v| A::cast_u16(v, normalized))
            }
            ReadTexCoords::F32(ref mut i) => i.nth(x).map(A::cast_f32),
            #[cfg(feature = "KHR_mesh_quantization")]
            ReadTexCoords::I8(ref mut i, normalized) => i.nth(x).map(|v| A::cast_i8(v, normalized)),
            #[cfg(feature = "KHR_mesh_quantization")]
            ReadTexCoords::I16(ref mut i, normalized) => {
                i.nth(x).map(|v| A::cast_i16(v, normalized))
            }
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            ReadTexCoords::U8(i, normalized) => i.last().map(|v| A::cast_u8(v, normalized)),
            ReadTexCoords::U16(i, normalized) => i.last().map(|v| A::cast_u16(v, normalized)),
            ReadTexCoords::F32(i) => i.last().map(A::cast_f32),
            #[cfg(feature = "KHR_mesh_quantization")]
            ReadTexCoords::I8(i, normalized) => i.last().map(|v| A::cast_i8(v, normalized)),
            #[cfg(feature = "KHR_mesh_quantization")]
            ReadTexCoords::I16(i, normalized) => i.last().map(|v| A::cast_i16(v, normalized)),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            ReadTexCoords::U8(ref i, _) => i.size_hint(),
            ReadTexCoords::U16(ref i, _) => i.size_hint(),
            ReadTexCoords::F32(ref i) => i.size_hint(),
            #[cfg(feature = "KHR_mesh_quantization")]
            ReadTexCoords::I8(ref i, _) => i.size_hint(),
            #[cfg(feature = "KHR_mesh_quantization")]
            ReadTexCoords::I16(ref i, _) => i.size_hint(),
        }
    }
}

impl Cast for U8 {
    type Output = [u8; 2];

    fn cast_u8(x: [u8; 2], normalized: bool) -> Self::Output {
        if normalized {
            x.normalize()
        } else {
            x
        }
    }

    fn cast_u16(x: [u16; 2], normalized: bool) -> Self::Output {
        if normalized {
            x.normalize()
        } else {
            [x[0] as u8, x[1] as u8]
        }
    }

    fn cast_f32(x: [f32; 2]) -> Self::Output {
        x.normalize()
    }

    #[cfg(feature = "KHR_mesh_quantization")]
    fn cast_i8(x: [i8; 2], normalized: bool) -> Self::Output {
        if normalized {
            // BYTE normalized: f = max(c / 127.0, -1.0), then convert to u8
            let f = [
                (x[0] as f32 / 127.0).max(-1.0),
                (x[1] as f32 / 127.0).max(-1.0),
            ];
            f.normalize()
        } else {
            [x[0] as u8, x[1] as u8]
        }
    }

    #[cfg(feature = "KHR_mesh_quantization")]
    fn cast_i16(x: [i16; 2], normalized: bool) -> Self::Output {
        if normalized {
            // SHORT normalized: f = max(c / 32767.0, -1.0), then convert to u8
            let f = [
                (x[0] as f32 / 32767.0).max(-1.0),
                (x[1] as f32 / 32767.0).max(-1.0),
            ];
            f.normalize()
        } else {
            [x[0] as u8, x[1] as u8]
        }
    }
}

impl Cast for U16 {
    type Output = [u16; 2];

    fn cast_u8(x: [u8; 2], normalized: bool) -> Self::Output {
        if normalized {
            x.normalize()
        } else {
            [x[0] as u16, x[1] as u16]
        }
    }

    fn cast_u16(x: [u16; 2], normalized: bool) -> Self::Output {
        if normalized {
            x.normalize()
        } else {
            x
        }
    }

    fn cast_f32(x: [f32; 2]) -> Self::Output {
        x.normalize()
    }

    #[cfg(feature = "KHR_mesh_quantization")]
    fn cast_i8(x: [i8; 2], normalized: bool) -> Self::Output {
        if normalized {
            // BYTE normalized: f = max(c / 127.0, -1.0), then convert to u16
            let f = [
                (x[0] as f32 / 127.0).max(-1.0),
                (x[1] as f32 / 127.0).max(-1.0),
            ];
            f.normalize()
        } else {
            [x[0] as u16, x[1] as u16]
        }
    }

    #[cfg(feature = "KHR_mesh_quantization")]
    fn cast_i16(x: [i16; 2], normalized: bool) -> Self::Output {
        if normalized {
            // SHORT normalized: f = max(c / 32767.0, -1.0), then convert to u16
            let f = [
                (x[0] as f32 / 32767.0).max(-1.0),
                (x[1] as f32 / 32767.0).max(-1.0),
            ];
            f.normalize()
        } else {
            [x[0] as u16, x[1] as u16]
        }
    }
}

impl Cast for F32 {
    type Output = [f32; 2];

    fn cast_u8(x: [u8; 2], normalized: bool) -> Self::Output {
        if normalized {
            // UNSIGNED_BYTE normalized: f = c / 255.0
            [x[0] as f32 / 255.0, x[1] as f32 / 255.0]
        } else {
            [x[0] as f32, x[1] as f32]
        }
    }

    fn cast_u16(x: [u16; 2], normalized: bool) -> Self::Output {
        if normalized {
            // UNSIGNED_SHORT normalized: f = c / 65535.0
            [x[0] as f32 / 65535.0, x[1] as f32 / 65535.0]
        } else {
            [x[0] as f32, x[1] as f32]
        }
    }

    fn cast_f32(x: [f32; 2]) -> Self::Output {
        x
    }

    #[cfg(feature = "KHR_mesh_quantization")]
    fn cast_i8(x: [i8; 2], normalized: bool) -> Self::Output {
        if normalized {
            // BYTE normalized: f = max(c / 127.0, -1.0)
            [
                (x[0] as f32 / 127.0).max(-1.0),
                (x[1] as f32 / 127.0).max(-1.0),
            ]
        } else {
            [x[0] as f32, x[1] as f32]
        }
    }

    #[cfg(feature = "KHR_mesh_quantization")]
    fn cast_i16(x: [i16; 2], normalized: bool) -> Self::Output {
        if normalized {
            // SHORT normalized: f = max(c / 32767.0, -1.0)
            [
                (x[0] as f32 / 32767.0).max(-1.0),
                (x[1] as f32 / 32767.0).max(-1.0),
            ]
        } else {
            [x[0] as f32, x[1] as f32]
        }
    }
}
