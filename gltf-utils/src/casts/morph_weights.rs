use std::marker::PhantomData;

use super::Normalizable;

use MorphWeights;

/// Casting iterator for `MorphWeights`.
#[derive(Debug, Clone)]
pub struct CastingIter<'a, T>(MorphWeights<'a>, PhantomData<T>);

/// Type which describes how to cast any weight into i8.
#[derive(Debug, Clone)]
pub struct I8;

/// Type which describes how to cast any weight into u8.
#[derive(Debug, Clone)]
pub struct U8;

/// Type which describes how to cast any weight into i16.
#[derive(Debug, Clone)]
pub struct I16;

/// Type which describes how to cast any weight into u16.
#[derive(Debug, Clone)]
pub struct U16;

/// Type which describes how to cast any weight into f32.
#[derive(Debug, Clone)]
pub struct F32;

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Output;

    /// Cast from i8.
    fn cast_i8(x: i8) -> Self::Output;

    /// Cast from u8.
    fn cast_u8(x: u8) -> Self::Output;

    /// Cast from i16.
    fn cast_i16(x: i16) -> Self::Output;

    /// Cast from u16.
    fn cast_u16(x: u16) -> Self::Output;

    /// Cast from f32.
    fn cast_f32(x: f32) -> Self::Output;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: MorphWeights<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `MorphWeights` object.
    pub fn unwrap(self) -> MorphWeights<'a> {
        self.0
    }
}

impl<'a, A: Cast> Iterator for CastingIter<'a, A> {
    type Item = A::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            MorphWeights::I8(ref mut i)  => i.next().map(A::cast_i8),
            MorphWeights::U8(ref mut i)  => i.next().map(A::cast_u8),
            MorphWeights::I16(ref mut i) => i.next().map(A::cast_i16),
            MorphWeights::U16(ref mut i) => i.next().map(A::cast_u16),
            MorphWeights::F32(ref mut i) => i.next().map(A::cast_f32),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            MorphWeights::I8(ref mut i)  => i.nth(x).map(A::cast_i8),
            MorphWeights::U8(ref mut i)  => i.nth(x).map(A::cast_u8),
            MorphWeights::I16(ref mut i) => i.nth(x).map(A::cast_i16),
            MorphWeights::U16(ref mut i) => i.nth(x).map(A::cast_u16),
            MorphWeights::F32(ref mut i) => i.nth(x).map(A::cast_f32),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            MorphWeights::I8(i)  => i.last().map(A::cast_i8),
            MorphWeights::U8(i)  => i.last().map(A::cast_u8),
            MorphWeights::I16(i) => i.last().map(A::cast_i16),
            MorphWeights::U16(i) => i.last().map(A::cast_u16),
            MorphWeights::F32(i) => i.last().map(A::cast_f32),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            MorphWeights::I8(ref i)  => i.size_hint(),
            MorphWeights::U8(ref i)  => i.size_hint(),
            MorphWeights::I16(ref i) => i.size_hint(),
            MorphWeights::U16(ref i) => i.size_hint(),
            MorphWeights::F32(ref i) => i.size_hint(),
        }
    }
}

impl Cast for I8 {
    type Output = i8;

    fn cast_i8(x: i8) -> Self::Output {
        x.normalize()
    }

    fn cast_u8(x: u8) -> Self::Output {
        x.normalize()
    }

    fn cast_i16(x: i16) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: u16) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: f32) -> Self::Output {
        x.normalize()
    }
}

impl Cast for U8 {
    type Output = u8;

    fn cast_i8(x: i8) -> Self::Output {
        x.normalize()
    }

    fn cast_u8(x: u8) -> Self::Output {
        x.normalize()
    }

    fn cast_i16(x: i16) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: u16) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: f32) -> Self::Output {
        x.normalize()
    }
}

impl Cast for I16 {
    type Output = i16;

    fn cast_i8(x: i8) -> Self::Output {
        x.normalize()
    }

    fn cast_u8(x: u8) -> Self::Output {
        x.normalize()
    }

    fn cast_i16(x: i16) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: u16) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: f32) -> Self::Output {
        x.normalize()
    }
}

impl Cast for U16 {
    type Output = u16;

    fn cast_i8(x: i8) -> Self::Output {
        x.normalize()
    }

    fn cast_u8(x: u8) -> Self::Output {
        x.normalize()
    }

    fn cast_i16(x: i16) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: u16) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: f32) -> Self::Output {
        x.normalize()
    }
}

impl Cast for F32 {
    type Output = f32;

    fn cast_i8(x: i8) -> Self::Output {
        x.normalize()
    }

    fn cast_u8(x: u8) -> Self::Output {
        x.normalize()
    }

    fn cast_i16(x: i16) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: u16) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: f32) -> Self::Output {
        x.normalize()
    }
}