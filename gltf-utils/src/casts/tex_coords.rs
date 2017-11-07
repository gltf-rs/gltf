use std::marker::PhantomData;

use super::u8_as_norm_f32;
use super::u8_as_u16_norm;
use super::u16_as_norm_f32;
use super::u16_as_u8_norm;
use super::norm_f32_as_u16;
use super::norm_f32_as_u8;

use TexCoords;

#[derive(Debug, Copy, Clone)]
pub struct CastingIter<'a, T>(TexCoords<'a>, PhantomData<T>);

#[derive(Debug, Copy, Clone)]
pub struct U8;

#[derive(Debug, Copy, Clone)]
pub struct U16;

#[derive(Debug, Copy, Clone)]
pub struct F32;

pub trait Cast {
    type Into;

    fn from_u8(x: [u8; 2]) -> Self::Into;
    fn from_u16(x: [u16; 2]) -> Self::Into;
    fn from_f32(x: [f32; 2]) -> Self::Into;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: TexCoords<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    pub fn unwrap(self) -> TexCoords<'a> {
        self.0
    }
}

impl<'a, A: Cast> Iterator for CastingIter<'a, A> {
    type Item = A::Into;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            TexCoords::U8(ref mut i)  => i.next().map(A::from_u8),
            TexCoords::U16(ref mut i) => i.next().map(A::from_u16),
            TexCoords::F32(ref mut i) => i.next().map(A::from_f32),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            TexCoords::U8(ref mut i)  => i.nth(x).map(A::from_u8),
            TexCoords::U16(ref mut i) => i.nth(x).map(A::from_u16),
            TexCoords::F32(ref mut i) => i.nth(x).map(A::from_f32),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            TexCoords::U8(i)  => i.last().map(A::from_u8),
            TexCoords::U16(i) => i.last().map(A::from_u16),
            TexCoords::F32(i) => i.last().map(A::from_f32),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            TexCoords::U8(ref i)  => i.size_hint(),
            TexCoords::U16(ref i) => i.size_hint(),
            TexCoords::F32(ref i) => i.size_hint(),
        }
    }
}

impl Cast for U8 {
    type Into = [u8; 2];

    fn from_u8(x: [u8; 2]) -> Self::Into { x }

    fn from_u16(x: [u16; 2]) -> Self::Into {
        [u16_as_u8_norm(x[0]), u16_as_u8_norm(x[1])]
    }

    fn from_f32(x: [f32; 2]) -> Self::Into {
        [norm_f32_as_u8(x[0]), norm_f32_as_u8(x[1])]
    }
}

impl Cast for U16 {
    type Into = [u16; 2];

    fn from_u8(x: [u8; 2]) -> Self::Into {
        [u8_as_u16_norm(x[0]), u8_as_u16_norm(x[1])]
    }

    fn from_u16(x: [u16; 2]) -> Self::Into { x }

    fn from_f32(x: [f32; 2]) -> Self::Into {
        [norm_f32_as_u16(x[0]), norm_f32_as_u16(x[1])]
    }
}

impl Cast for F32 {
    type Into = [f32; 2];

    fn from_u8(x: [u8; 2]) -> Self::Into {
        [u8_as_norm_f32(x[0]), u8_as_norm_f32(x[1])]
    }

    fn from_u16(x: [u16; 2]) -> Self::Into {
        [u16_as_norm_f32(x[0]), u16_as_norm_f32(x[1])]
    }

    fn from_f32(x: [f32; 2]) -> Self::Into { x }
}
