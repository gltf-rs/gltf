use std::marker::PhantomData;

use super::norm_u8_as_f32;
use super::norm_u8_as_u16;
use super::norm_u16_as_f32;
use super::norm_u16_as_u8;
use super::norm_f32_as_u16;
use super::norm_f32_as_u8;

use Colors;

/// Casting iterator for `Colors`.
#[derive(Debug, Clone)]
pub struct CastingIter<'a, T>(Colors<'a>, PhantomData<T>);

/// Type which describes how to cast any color into RGB u8.
#[derive(Debug, Clone)]
pub struct RgbU8;

/// Type which describes how to cast any color into RGB u16.
#[derive(Debug, Clone)]
pub struct RgbU16;

/// Type which describes how to cast any color into RGB f32.
#[derive(Debug, Clone)]
pub struct RgbF32;

/// Type which describes how to cast any color into RGBA u8.
#[derive(Debug, Clone)]
pub struct RgbaU8;

/// Type which describes how to cast any color into RGBA u16.
#[derive(Debug, Clone)]
pub struct RgbaU16;

/// Type which describes how to cast any color into RGBA f32.
#[derive(Debug, Clone)]
pub struct RgbaF32;

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Into;

    /// Cast from RGB u8.
    fn from_rgb_u8(x: [u8; 3]) -> Self::Into;

    /// Cast from RGB u16.
    fn from_rgb_u16(x: [u16; 3]) -> Self::Into;

    /// Cast from RGB f32.
    fn from_rgb_f32(x: [f32; 3]) -> Self::Into;

    /// Cast from RGBA u8.
    fn from_rgba_u8(x: [u8; 4]) -> Self::Into;

    /// Cast from RGBA u16.
    fn from_rgba_u16(x: [u16; 4]) -> Self::Into;

    /// Cast from RGBA f32.
    fn from_rgba_f32(x: [f32; 4]) -> Self::Into;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: Colors<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `Colors` object.
    pub fn unwrap(self) -> Colors<'a> {
        self.0
    }
}

impl<'a, A: Cast> Iterator for CastingIter<'a, A> {
    type Item = A::Into;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Colors::RgbU8(ref mut i)   => i.next().map(A::from_rgb_u8),
            Colors::RgbU16(ref mut i)  => i.next().map(A::from_rgb_u16),
            Colors::RgbF32(ref mut i)  => i.next().map(A::from_rgb_f32),
            Colors::RgbaU8(ref mut i)  => i.next().map(A::from_rgba_u8),
            Colors::RgbaU16(ref mut i) => i.next().map(A::from_rgba_u16),
            Colors::RgbaF32(ref mut i) => i.next().map(A::from_rgba_f32),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            Colors::RgbU8(ref mut i)   => i.nth(x).map(A::from_rgb_u8),
            Colors::RgbU16(ref mut i)  => i.nth(x).map(A::from_rgb_u16),
            Colors::RgbF32(ref mut i)  => i.nth(x).map(A::from_rgb_f32),
            Colors::RgbaU8(ref mut i)  => i.nth(x).map(A::from_rgba_u8),
            Colors::RgbaU16(ref mut i) => i.nth(x).map(A::from_rgba_u16),
            Colors::RgbaF32(ref mut i) => i.nth(x).map(A::from_rgba_f32),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            Colors::RgbU8(i)   => i.last().map(A::from_rgb_u8),
            Colors::RgbU16(i)  => i.last().map(A::from_rgb_u16),
            Colors::RgbF32(i)  => i.last().map(A::from_rgb_f32),
            Colors::RgbaU8(i)  => i.last().map(A::from_rgba_u8),
            Colors::RgbaU16(i) => i.last().map(A::from_rgba_u16),
            Colors::RgbaF32(i) => i.last().map(A::from_rgba_f32),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Colors::RgbU8(ref i)   => i.size_hint(),
            Colors::RgbU16(ref i)  => i.size_hint(),
            Colors::RgbF32(ref i)  => i.size_hint(),
            Colors::RgbaU8(ref i)  => i.size_hint(),
            Colors::RgbaU16(ref i) => i.size_hint(),
            Colors::RgbaF32(ref i) => i.size_hint(),
        }
    }
}

impl Cast for RgbU8 {
    type Into = [u8; 3];

    fn from_rgb_u8(x: [u8; 3]) -> Self::Into { x }

    fn from_rgb_u16(x: [u16; 3]) -> Self::Into {
        [norm_u16_as_u8(x[0]), norm_u16_as_u8(x[1]), norm_u16_as_u8(x[2])]
    }

    fn from_rgb_f32(x: [f32; 3]) -> Self::Into {
        [norm_f32_as_u8(x[0]), norm_f32_as_u8(x[1]), norm_f32_as_u8(x[2])]
    }

    fn from_rgba_u8(x: [u8; 4]) -> Self::Into {
        Self::from_rgb_u8([x[0], x[1], x[2]])
    }

    fn from_rgba_u16(x: [u16; 4]) -> Self::Into {
        Self::from_rgb_u16([x[0], x[1], x[2]])
    }

    fn from_rgba_f32(x: [f32; 4]) -> Self::Into {
        Self::from_rgb_f32([x[0], x[1], x[2]])
    }
}

impl Cast for RgbU16 {
    type Into = [u16; 3];

    fn from_rgb_u8(x: [u8; 3]) -> Self::Into {
        [norm_u8_as_u16(x[0]), norm_u8_as_u16(x[1]), norm_u8_as_u16(x[2])]
    }

    fn from_rgb_u16(x: [u16; 3]) -> Self::Into { x }

    fn from_rgb_f32(x: [f32; 3]) -> Self::Into {
        [norm_f32_as_u16(x[0]), norm_f32_as_u16(x[1]), norm_f32_as_u16(x[2])]
    }

    fn from_rgba_u8(x: [u8; 4]) -> Self::Into {
        Self::from_rgb_u8([x[0], x[1], x[2]])
    }

    fn from_rgba_u16(x: [u16; 4]) -> Self::Into {
        Self::from_rgb_u16([x[0], x[1], x[2]])
    }

    fn from_rgba_f32(x: [f32; 4]) -> Self::Into {
        Self::from_rgb_f32([x[0], x[1], x[2]])
    }
}

impl Cast for RgbF32 {
    type Into = [f32; 3];

    fn from_rgb_u8(x: [u8; 3]) -> Self::Into {
        [norm_u8_as_f32(x[0]), norm_u8_as_f32(x[1]), norm_u8_as_f32(x[2])]
    }

    fn from_rgb_u16(x: [u16; 3]) -> Self::Into {
        [norm_u16_as_f32(x[0]), norm_u16_as_f32(x[1]), norm_u16_as_f32(x[2])]
    }

    fn from_rgb_f32(x: [f32; 3]) -> Self::Into { x }

    fn from_rgba_u8(x: [u8; 4]) -> Self::Into {
        Self::from_rgb_u8([x[0], x[1], x[2]])
    }

    fn from_rgba_u16(x: [u16; 4]) -> Self::Into {
        Self::from_rgb_u16([x[0], x[1], x[2]])
    }

    fn from_rgba_f32(x: [f32; 4]) -> Self::Into {
        Self::from_rgb_f32([x[0], x[1], x[2]])
    }
}

impl Cast for RgbaU8 {
    type Into = [u8; 4];

    fn from_rgb_u8(x: [u8; 3]) -> Self::Into {
        Self::from_rgba_u8([x[0], x[1], x[2], u8::max_value()])
    }

    fn from_rgb_u16(x: [u16; 3]) -> Self::Into {
        Self::from_rgba_u16([x[0], x[1], x[2], u16::max_value()])
    }

    fn from_rgb_f32(x: [f32; 3]) -> Self::Into {
        Self::from_rgba_f32([x[0], x[1], x[2], 1.0])
    }

    fn from_rgba_u8(x: [u8; 4]) -> Self::Into { x }

    fn from_rgba_u16(x: [u16; 4]) -> Self::Into {
        [norm_u16_as_u8(x[0]), norm_u16_as_u8(x[1]),
         norm_u16_as_u8(x[2]), norm_u16_as_u8(x[2])]
    }

    fn from_rgba_f32(x: [f32; 4]) -> Self::Into {
        [norm_f32_as_u8(x[0]), norm_f32_as_u8(x[1]),
         norm_f32_as_u8(x[2]), norm_f32_as_u8(x[2])]
    }
}

impl Cast for RgbaU16 {
    type Into = [u16; 4];

    fn from_rgb_u8(x: [u8; 3]) -> Self::Into {
        Self::from_rgba_u8([x[0], x[1], x[2], u8::max_value()])
    }

    fn from_rgb_u16(x: [u16; 3]) -> Self::Into {
        Self::from_rgba_u16([x[0], x[1], x[2], u16::max_value()])
    }

    fn from_rgb_f32(x: [f32; 3]) -> Self::Into {
        Self::from_rgba_f32([x[0], x[1], x[2], 1.0])
    }

    fn from_rgba_u8(x: [u8; 4]) -> Self::Into {
        [norm_u8_as_u16(x[0]), norm_u8_as_u16(x[1]),
         norm_u8_as_u16(x[2]), norm_u8_as_u16(x[2])]
    }

    fn from_rgba_u16(x: [u16; 4]) -> Self::Into { x }

    fn from_rgba_f32(x: [f32; 4]) -> Self::Into {
        [norm_f32_as_u16(x[0]), norm_f32_as_u16(x[1]),
         norm_f32_as_u16(x[2]), norm_f32_as_u16(x[2])]
    }
}

impl Cast for RgbaF32 {
    type Into = [f32; 4];

    fn from_rgb_u8(x: [u8; 3]) -> Self::Into {
        Self::from_rgba_u8([x[0], x[1], x[2], u8::max_value()])
    }

    fn from_rgb_u16(x: [u16; 3]) -> Self::Into {
        Self::from_rgba_u16([x[0], x[1], x[2], u16::max_value()])
    }

    fn from_rgb_f32(x: [f32; 3]) -> Self::Into {
        Self::from_rgba_f32([x[0], x[1], x[2], 1.0])
    }

    fn from_rgba_u8(x: [u8; 4]) -> Self::Into {
        [norm_u8_as_f32(x[0]), norm_u8_as_f32(x[1]),
         norm_u8_as_f32(x[2]), norm_u8_as_f32(x[2])]
    }

    fn from_rgba_u16(x: [u16; 4]) -> Self::Into {
        [norm_u16_as_f32(x[0]), norm_u16_as_f32(x[1]),
         norm_u16_as_f32(x[2]), norm_u16_as_f32(x[2])]
    }

    fn from_rgba_f32(x: [f32; 4]) -> Self::Into { x }
}
