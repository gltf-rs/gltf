use std::marker::PhantomData;

use super::Normalizable;

use MorphWeights;

/// Casting iterator for `MorphWeights`.
#[derive(Debug, Clone)]
pub struct CastingIter<'a, T>(MorphWeights<'a>, PhantomData<T>);

/// Type which describes how to cast any weight into f32.
#[derive(Debug, Clone)]
pub struct F32;

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Into;

    /// Cast from i8.
    fn from_i8(x: i8) -> Self::Into;

    /// Cast from u8.
    fn from_u8(x: u8) -> Self::Into;

    /// Cast from i16.
    fn from_i16(x: i16) -> Self::Into;

    /// Cast from u16.
    fn from_u16(x: u16) -> Self::Into;

    /// Cast from f32.
    fn from_f32(x: f32) -> Self::Into;
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
    type Item = A::Into;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            MorphWeights::I8(ref mut i)  => i.next().map(A::from_i8),
            MorphWeights::U8(ref mut i)  => i.next().map(A::from_u8),
            MorphWeights::I16(ref mut i) => i.next().map(A::from_i16),
            MorphWeights::U16(ref mut i) => i.next().map(A::from_u16),
            MorphWeights::F32(ref mut i) => i.next().map(A::from_f32),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            MorphWeights::I8(ref mut i)  => i.nth(x).map(A::from_i8),
            MorphWeights::U8(ref mut i)  => i.nth(x).map(A::from_u8),
            MorphWeights::I16(ref mut i) => i.nth(x).map(A::from_i16),
            MorphWeights::U16(ref mut i) => i.nth(x).map(A::from_u16),
            MorphWeights::F32(ref mut i) => i.nth(x).map(A::from_f32),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            MorphWeights::I8(i)  => i.last().map(A::from_i8),
            MorphWeights::U8(i)  => i.last().map(A::from_u8),
            MorphWeights::I16(i) => i.last().map(A::from_i16),
            MorphWeights::U16(i) => i.last().map(A::from_u16),
            MorphWeights::F32(i) => i.last().map(A::from_f32),
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

impl Cast for F32 {
    type Into = f32;

    fn from_i8(x: i8) -> Self::Into {
        Normalizable::normalize(x)
    }

    fn from_u8(x: u8) -> Self::Into {
        Normalizable::normalize(x)
    }

    fn from_i16(x: i16) -> Self::Into {
        Normalizable::normalize(x)
    }

    fn from_u16(x: u16) -> Self::Into {
        Normalizable::normalize(x)
    }

    fn from_f32(x: f32) -> Self::Into {
        Normalizable::normalize(x)
    }
}
