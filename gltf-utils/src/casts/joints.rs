use std::marker::PhantomData;

use Joints;

/// Casting iterator for `Joints`.
#[derive(Debug, Clone)]
pub struct CastingIter<'a, T>(Joints<'a>, PhantomData<T>);

/// Type which describes how to cast any joint into u16.
#[derive(Debug, Clone)]
pub struct U16;

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Into;

    /// Cast from u8.
    fn from_u8(x: [u8; 4]) -> Self::Into;

    /// Cast from u16.
    fn from_u16(x: [u16; 4]) -> Self::Into;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: Joints<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `Joints` object.
    pub fn unwrap(self) -> Joints<'a> {
        self.0
    }
}

impl<'a, A: Cast> Iterator for CastingIter<'a, A> {
    type Item = A::Into;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Joints::U8(ref mut i)  => i.next().map(A::from_u8),
            Joints::U16(ref mut i) => i.next().map(A::from_u16),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            Joints::U8(ref mut i)  => i.nth(x).map(A::from_u8),
            Joints::U16(ref mut i) => i.nth(x).map(A::from_u16),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            Joints::U8(i)  => i.last().map(A::from_u8),
            Joints::U16(i) => i.last().map(A::from_u16),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Joints::U8(ref i)  => i.size_hint(),
            Joints::U16(ref i) => i.size_hint(),
        }
    }
}

impl Cast for U16 {
    type Into = [u16; 4];

    fn from_u8(x: [u8; 4]) -> Self::Into {
        [x[0] as u16, x[1] as u16, x[2] as u16, x[3] as u16]
    }

    fn from_u16(x: [u16; 4]) -> Self::Into { x }
}
