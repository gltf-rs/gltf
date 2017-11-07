use std::marker::PhantomData;

use Indices;

#[derive(Debug, Copy, Clone)]
pub struct CastingIter<'a, T>(Indices<'a>, PhantomData<T>);

#[derive(Debug, Copy, Clone)]
pub struct U32;

pub trait Cast {
    type Into;

    fn from_u8(x: u8) -> Self::Into;
    fn from_u16(x: u16) -> Self::Into;
    fn from_u32(x: u32) -> Self::Into;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: Indices<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    pub fn unwrap(self) -> Indices<'a> {
        self.0
    }
}

impl<'a, A: Cast> Iterator for CastingIter<'a, A> {
    type Item = A::Into;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Indices::U8(ref mut i)  => i.next().map(A::from_u8),
            Indices::U16(ref mut i) => i.next().map(A::from_u16),
            Indices::U32(ref mut i) => i.next().map(A::from_u32),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            Indices::U8(ref mut i)  => i.nth(x).map(A::from_u8),
            Indices::U16(ref mut i) => i.nth(x).map(A::from_u16),
            Indices::U32(ref mut i) => i.nth(x).map(A::from_u32),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            Indices::U8(i)  => i.last().map(A::from_u8),
            Indices::U16(i) => i.last().map(A::from_u16),
            Indices::U32(i) => i.last().map(A::from_u32),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Indices::U8(ref i)  => i.size_hint(),
            Indices::U16(ref i) => i.size_hint(),
            Indices::U32(ref i) => i.size_hint(),
        }
    }
}

impl Cast for U32 {
    type Into = u32;

    fn from_u8(x: u8) -> Self::Into { x as Self::Into }
    fn from_u16(x: u16) -> Self::Into { x as Self::Into }
    fn from_u32(x: u32) -> Self::Into { x }
}
