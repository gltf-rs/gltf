use std::marker::PhantomData;

use super::Normalizable;

use Weights;

/// Casting iterator for `Weights`.
#[derive(Debug, Clone)]
pub struct CastingIter<'a, T>(Weights<'a>, PhantomData<T>);

/// Type which describes how to cast any weight into u8.
#[derive(Debug, Clone)]
pub struct U8;

/// Type which describes how to cast any weight into u16.
#[derive(Debug, Clone)]
pub struct U16;

/// Type which describes how to cast any weight into f32.
#[derive(Debug, Clone)]
pub struct F32;

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Into;

    /// Cast from u8.
    fn from_u8(x: [u8; 4]) -> Self::Into;

    /// Cast from u16.
    fn from_u16(x: [u16; 4]) -> Self::Into;

    /// Cast from f32.
    fn from_f32(x: [f32; 4]) -> Self::Into;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: Weights<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `Weights` object.
    pub fn unwrap(self) -> Weights<'a> {
        self.0
    }
}

impl<'a, A: Cast> Iterator for CastingIter<'a, A> {
    type Item = A::Into;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Weights::U8(ref mut i)  => i.next().map(A::from_u8),
            Weights::U16(ref mut i) => i.next().map(A::from_u16),
            Weights::F32(ref mut i) => i.next().map(A::from_f32),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            Weights::U8(ref mut i)  => i.nth(x).map(A::from_u8),
            Weights::U16(ref mut i) => i.nth(x).map(A::from_u16),
            Weights::F32(ref mut i) => i.nth(x).map(A::from_f32),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            Weights::U8(i)  => i.last().map(A::from_u8),
            Weights::U16(i) => i.last().map(A::from_u16),
            Weights::F32(i) => i.last().map(A::from_f32),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Weights::U8(ref i)  => i.size_hint(),
            Weights::U16(ref i) => i.size_hint(),
            Weights::F32(ref i) => i.size_hint(),
        }
    }
}

impl Cast for U8 {
    type Into = [u8; 4];

    fn from_u8(x: [u8; 4]) -> Self::Into {
        Normalizable::normalize(x)
    }

    fn from_u16(x: [u16; 4]) -> Self::Into {
        Normalizable::normalize(x)
    }

    fn from_f32(x: [f32; 4]) -> Self::Into {
        Normalizable::normalize(x)
    }
}

impl Cast for U16 {
    type Into = [u16; 4];

    fn from_u8(x: [u8; 4]) -> Self::Into {
        Normalizable::normalize(x)
    }

    fn from_u16(x: [u16; 4]) -> Self::Into {
        Normalizable::normalize(x)
    }

    fn from_f32(x: [f32; 4]) -> Self::Into {
        Normalizable::normalize(x)
    }
}

impl Cast for F32 {
    type Into = [f32; 4];

    fn from_u8(x: [u8; 4]) -> Self::Into {
        Normalizable::normalize(x)
    }

    fn from_u16(x: [u16; 4]) -> Self::Into {
        Normalizable::normalize(x)
    }

    fn from_f32(x: [f32; 4]) -> Self::Into {
        Normalizable::normalize(x)
    }
}
