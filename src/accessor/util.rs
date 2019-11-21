use std::{iter, mem};
use byteorder::{LE, ByteOrder};
use std::marker::PhantomData;

use crate::{accessor, buffer};

fn buffer_view_slice<'a, 's>(
    view: buffer::View<'a>,
    get_buffer_data: &dyn Fn(buffer::Buffer<'a>) -> Option<&'s [u8]>,
) -> Option<&'s [u8]> {
    let start = view.offset();
    let end = start + view.length();
    get_buffer_data(view.buffer())
        .map(|slice| &slice[start..end])
}

/// General iterator for an accessor.
#[derive(Clone, Debug)]
pub enum Iter<'a, T: Item> {
    /// Standard accessor iterator.
    Standard(ItemIter<'a, T>),

    /// Iterator for accessor with sparse values.
    Sparse(SparseIter<'a, T>),
}

impl<'a, T: Item> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            &mut Iter::Standard(ref mut iter) => iter.next(),
            &mut Iter::Sparse(ref mut iter) => iter.next(),
        }
    }

    fn nth(&mut self, nth: usize) -> Option<Self::Item> {
        match self {
            &mut Iter::Standard(ref mut iter) => iter.nth(nth),
            &mut Iter::Sparse(ref mut iter) => iter.nth(nth),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self {
            Iter::Standard(iter) => iter.last(),
            Iter::Sparse(iter) => iter.last(),
        }
    }

    fn count(self) -> usize {
        match self {
            Iter::Standard(iter) => iter.count(),
            Iter::Sparse(iter) => iter.count(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            &Iter::Standard(ref iter) => iter.size_hint(),
            &Iter::Sparse(ref iter) => iter.size_hint(),
        }
    }
}

impl<'a, T: Item> ExactSizeIterator for Iter<'a, T> {}

/// Iterator over indices of sparse accessor.
#[derive(Clone, Debug)]
pub enum SparseIndicesIter<'a> {
    /// 8-bit indices.
    U8(ItemIter<'a, u8>),
    /// 16-bit indices.
    U16(ItemIter<'a, u16>),
    /// 32-bit indices.
    U32(ItemIter<'a, u32>),
}

impl<'a> Iterator for SparseIndicesIter<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            SparseIndicesIter::U8(ref mut iter) => iter.next().map(|x| x as u32),
            SparseIndicesIter::U16(ref mut iter) => iter.next().map(|x| x as u32),
            SparseIndicesIter::U32(ref mut iter) => iter.next(),
        }
    }
}

/// Iterates over a sparse accessor.
#[derive(Clone, Debug)]
pub struct SparseIter<'a, T: Item> {
    /// Base value iterator.
    ///
    /// This can be `None` if the base buffer view is not set. In this case the base values are all zero.
    base: Option<ItemIter<'a, T>>,

    /// Sparse indices iterator.
    indices: iter::Peekable<SparseIndicesIter<'a>>,

    /// Sparse values iterator.
    values: ItemIter<'a, T>,

    /// Iterator counter.
    counter: u32,
}

impl<'a, T: Item> SparseIter<'a, T> {
    /// Constructor.
    /// 
    /// Here `base` is allowed to be `None` when the base buffer view is not explicitly specified.
    pub fn new(
        base: Option<ItemIter<'a, T>>,
        indices: SparseIndicesIter<'a>,
        values: ItemIter<'a, T>,
    ) -> Self {
        SparseIter {
            base,
            indices: indices.peekable(),
            values: values,
            counter: 0,
        }
    }
}

impl<'a, T: Item> Iterator for SparseIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let next_base_value = self.base.as_mut().map(|iter| iter.next()).unwrap_or(Some(T::zero()));
        if next_base_value.is_none() {
            return None;
        }

        let mut next_value = next_base_value.unwrap();
        let next_sparse_index = self.indices.peek().clone();
        if let Some(index) = next_sparse_index {
            if *index == self.counter {
                self.indices.next(); // advance
                next_value = self.values.next().unwrap();
            }
        }

        self.counter += 1;

        Some(next_value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.values.len() - self.counter as usize;
        (hint, Some(hint))
    }
}

impl<'a, T: Item> ExactSizeIterator for SparseIter<'a, T> {}

/// Represents items that can be read by an [`Accessor`].
///
/// [`Accessor`]: struct.Accessor.html
pub trait Item {
    /// Create an object of this type from a byte slice.
    fn from_slice(slice: &[u8]) -> Self;
    /// Create an object of this type that represents a zero value.
    fn zero() -> Self;
}

/// Visits the items in an [`Accessor`].
///
/// [`Accessor`]: struct.Accessor.html
#[derive(Copy, Clone, Debug)]
pub struct ItemIter<'a, T: Item> {
    stride: usize,
    data: &'a [u8],
    _phantom: PhantomData<T>,
}

impl Item for i8 {
    fn from_slice(slice: &[u8]) -> Self {
        slice[0] as i8
    }
    fn zero() -> Self {
        0
    }
}

impl Item for i16 {
    fn from_slice(slice: &[u8]) -> Self {
        LE::read_i16(slice)
    }
    fn zero() -> Self {
        0
    }
}

impl Item for u8 {
    fn from_slice(slice: &[u8]) -> Self {
        slice[0]
    }
    fn zero() -> Self {
        0
    }
}

impl Item for u16 {
    fn from_slice(slice: &[u8]) -> Self {
        LE::read_u16(slice)
    }
    fn zero() -> Self {
        0
    }
}

impl Item for u32 {
    fn from_slice(slice: &[u8]) -> Self {
        LE::read_u32(slice)
    }
    fn zero() -> Self {
        0
    }
}

impl Item for f32 {
    fn from_slice(slice: &[u8]) -> Self {
        LE::read_f32(slice)
    }
    fn zero() -> Self {
        0.0
    }
}

impl<T: Item + Copy> Item for [T; 2] {
    fn from_slice(slice: &[u8]) -> Self {
        assert!(slice.len() >= 2 * mem::size_of::<T>());
        [T::from_slice(slice),
         T::from_slice(&slice[mem::size_of::<T>() ..])]
    }
    fn zero() -> Self {
        [T::zero(); 2]
    }
}

impl<T: Item + Copy> Item for [T; 3] {
    fn from_slice(slice: &[u8]) -> Self {
        assert!(slice.len() >= 3 * mem::size_of::<T>());
        [T::from_slice(slice),
         T::from_slice(&slice[1 * mem::size_of::<T>() ..]),
         T::from_slice(&slice[2 * mem::size_of::<T>() ..])]
    }
    fn zero() -> Self {
        [T::zero(); 3]
    }
}

impl<T: Item + Copy> Item for [T; 4] {
    fn from_slice(slice: &[u8]) -> Self {
        assert!(slice.len() >= 4 * mem::size_of::<T>());
        [T::from_slice(slice),
         T::from_slice(&slice[1 * mem::size_of::<T>() ..]),
         T::from_slice(&slice[2 * mem::size_of::<T>() ..]),
         T::from_slice(&slice[3 * mem::size_of::<T>() ..])]
    }
    fn zero() -> Self {
        [T::zero(); 4]
    }
}

impl<'a, T: Item> ItemIter<'a, T> {
    /// Constructor.
    pub fn new(slice: &'a [u8], stride: usize) -> Self {
        ItemIter {
            data: slice,
            stride: stride,
            _phantom: PhantomData,
        }
    }
}

impl<'a, 's, T: Item> Iter<'s, T> {
    /// Constructor.
    pub fn new<F>(
        accessor: super::Accessor<'a>,
        get_buffer_data: F,
    ) -> Option<Iter<'s, T>>
        where F: Clone + Fn(buffer::Buffer<'a>) -> Option<&'s [u8]>,
    {
        let is_sparse = accessor.sparse().is_some();
        if is_sparse {
            let sparse = accessor.sparse();
            let indices = sparse.as_ref().unwrap().indices();
            let values = sparse.as_ref().unwrap().values();
            let base_iter = {
                // Using `if let` here instead of map to preserve the early return behavior.
                if let Some(view) = accessor.view() {
                    let stride = view.stride().unwrap_or(mem::size_of::<T>());
                    let start = accessor.offset();
                    let end = start + stride * (accessor.count() - 1) + mem::size_of::<T>();
                    let subslice = if let Some(slice) = buffer_view_slice(view, &get_buffer_data) {
                        &slice[start..end]
                    } else {
                        return None
                    };
                    Some(ItemIter::new(subslice, stride))
                } else {
                    None
                }
            };
            let sparse_count = sparse.as_ref().unwrap().count() as usize;
            let index_iter = {
                let view = indices.view();
                let index_size = indices.index_type().size();
                let stride = view.stride().unwrap_or(index_size);
                let subslice = if let Some(slice) = buffer_view_slice(view, &get_buffer_data) {
                    let start = indices.offset() as usize;
                    let end = start + stride * (sparse_count - 1) + index_size;
                    &slice[start..end]
                } else {
                    return None
                };
                match indices.index_type() {
                    accessor::sparse::IndexType::U8 => SparseIndicesIter::U8(ItemIter::new(subslice, stride)),
                    accessor::sparse::IndexType::U16 => SparseIndicesIter::U16(ItemIter::new(subslice, stride)),
                    accessor::sparse::IndexType::U32 => SparseIndicesIter::U32(ItemIter::new(subslice, stride)),
                }
            };
            let value_iter = {
                let view = values.view();
                let stride = view.stride().unwrap_or(mem::size_of::<T>());
                let subslice = if let Some(slice) = buffer_view_slice(view, &get_buffer_data) {
                    let start = values.offset() as usize;
                    let end = start + stride * (sparse_count - 1) + mem::size_of::<T>();
                    &slice[start..end]
                } else {
                    return None
                };
                ItemIter::new(subslice, stride)
            };
            Some(Iter::Sparse(SparseIter::new(base_iter, index_iter, value_iter)))
        } else {
            debug_assert_eq!(mem::size_of::<T>(), accessor.size());
            debug_assert!(mem::size_of::<T>() > 0);
            if let Some(view) = accessor.view() {
                let stride = view.stride().unwrap_or(mem::size_of::<T>());
                debug_assert!(stride >= mem::size_of::<T>(), "Mismatch in stride, expected at least {} stride but found {}", mem::size_of::<T>(), stride);
                let start = accessor.offset();
                let end = start + stride * (accessor.count() - 1) + mem::size_of::<T>();
                let subslice = if let Some(slice) = buffer_view_slice(view, &get_buffer_data) {
                    &slice[start..end]
                } else {
                    return None
                };
                Some(Iter::Standard(ItemIter { stride, data: subslice, _phantom: PhantomData }))
            } else {
                None
            }
        }
    }
}

impl<'a, T: Item> ExactSizeIterator for ItemIter<'a, T> {}
impl<'a, T: Item> Iterator for ItemIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let stride = if self.data.len() >= self.stride {
            Some(self.stride)
        } else if self.data.len() >= mem::size_of::<T>() {
            Some(mem::size_of::<T>())
        } else {
            None
        };
        if let Some(stride) = stride {
            let (val, data) = self.data.split_at(stride);
            let val = T::from_slice(val);
            self.data = data;
            Some(val)
        } else {
            None
        }
    }

    fn nth(&mut self, nth: usize) -> Option<Self::Item> {
        if let Some(val_data) = self.data.get(nth * self.stride ..) {
            if val_data.len() >= mem::size_of::<T>() {
                let val = T::from_slice(val_data);
                self.data = &val_data[self.stride.min(val_data.len()) ..];
                Some(val)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn last(self) -> Option<Self::Item> {
        if self.data.len() >= mem::size_of::<T>() {
            self.data
                .get((self.data.len() - 1) / self.stride * self.stride ..)
                .map(T::from_slice)
        } else {
            None
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.data.len() / self.stride
            + (self.data.len() % self.stride >= mem::size_of::<T>()) as usize;
        (hint, Some(hint))
    }
}
