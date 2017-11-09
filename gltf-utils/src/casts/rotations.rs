use std::marker::PhantomData;

use super::norm_i8_as_f32;
use super::norm_u8_as_f32;
use super::norm_i16_as_f32;
use super::norm_u16_as_f32;

use Rotations;

/// Casting iterator for `Rotations`.
#[derive(Debug, Copy, Clone)]
pub struct CastingIter<'a, T>(Rotations<'a>, PhantomData<T>);

/// Type which describes how to cast any weight into f32.
#[derive(Debug, Copy, Clone)]
pub struct F32;

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Into;

    /// Cast from i8.
    fn from_i8(x: [i8; 4]) -> Self::Into;

    /// Cast from u8.
    fn from_u8(x: [u8; 4]) -> Self::Into;

    /// Cast from i16.
    fn from_i16(x: [i16; 4]) -> Self::Into;

    /// Cast from u16.
    fn from_u16(x: [u16; 4]) -> Self::Into;

    /// Cast from f32.
    fn from_f32(x: [f32; 4]) -> Self::Into;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: Rotations<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `Rotations` object.
    pub fn unwrap(self) -> Rotations<'a> {
        self.0
    }
}

impl<'a, A: Cast> Iterator for CastingIter<'a, A> {
    type Item = A::Into;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Rotations::I8(ref mut i)  => i.next().map(A::from_i8),
            Rotations::U8(ref mut i)  => i.next().map(A::from_u8),
            Rotations::I16(ref mut i) => i.next().map(A::from_i16),
            Rotations::U16(ref mut i) => i.next().map(A::from_u16),
            Rotations::F32(ref mut i) => i.next().map(A::from_f32),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            Rotations::I8(ref mut i)  => i.nth(x).map(A::from_i8),
            Rotations::U8(ref mut i)  => i.nth(x).map(A::from_u8),
            Rotations::I16(ref mut i) => i.nth(x).map(A::from_i16),
            Rotations::U16(ref mut i) => i.nth(x).map(A::from_u16),
            Rotations::F32(ref mut i) => i.nth(x).map(A::from_f32),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            Rotations::I8(i)  => i.last().map(A::from_i8),
            Rotations::U8(i)  => i.last().map(A::from_u8),
            Rotations::I16(i) => i.last().map(A::from_i16),
            Rotations::U16(i) => i.last().map(A::from_u16),
            Rotations::F32(i) => i.last().map(A::from_f32),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Rotations::I8(ref i)  => i.size_hint(),
            Rotations::U8(ref i)  => i.size_hint(),
            Rotations::I16(ref i) => i.size_hint(),
            Rotations::U16(ref i) => i.size_hint(),
            Rotations::F32(ref i) => i.size_hint(),
        }
    }
}

impl Cast for F32 {
    type Into = [f32; 4];

    fn from_i8(x: [i8; 4]) -> Self::Into {
        [norm_i8_as_f32(x[0]), norm_i8_as_f32(x[1]),
         norm_i8_as_f32(x[2]), norm_i8_as_f32(x[3])]
    }

    fn from_u8(x: [u8; 4]) -> Self::Into {
        [norm_u8_as_f32(x[0]), norm_u8_as_f32(x[1]),
         norm_u8_as_f32(x[2]), norm_u8_as_f32(x[3])]
    }

    fn from_i16(x: [i16; 4]) -> Self::Into {
        [norm_i16_as_f32(x[0]), norm_i16_as_f32(x[1]),
         norm_i16_as_f32(x[2]), norm_i16_as_f32(x[3])]
    }

    fn from_u16(x: [u16; 4]) -> Self::Into {
        [norm_u16_as_f32(x[0]), norm_u16_as_f32(x[1]),
         norm_u16_as_f32(x[2]), norm_u16_as_f32(x[3])]
    }

    fn from_f32(x: [f32; 4]) -> Self::Into { x }
}
