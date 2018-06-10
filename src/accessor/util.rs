use std::mem;
use byteorder::{LE, ByteOrder};
use std::marker::PhantomData;

/// Represents items that can be read by an `Accessor`.
pub trait Item {
    /// Create an object of this type from a byte slice.
    fn from_slice(slice: &[u8]) -> Self;
}

/// Visits the items in an `Accessor`.
#[derive(Copy, Clone, Debug)]
pub struct Iter<'a, T> {
    stride: usize,
    data: &'a [u8],
    _phantom: PhantomData<T>,
}

impl Item for i8 {
    fn from_slice(slice: &[u8]) -> Self {
        slice[0] as i8
    }
}

impl Item for i16 {
    fn from_slice(slice: &[u8]) -> Self {
        LE::read_i16(slice)
    }
}

impl Item for u8 {
    fn from_slice(slice: &[u8]) -> Self {
        slice[0]
    }
}

impl Item for u16 {
    fn from_slice(slice: &[u8]) -> Self {
        LE::read_u16(slice)
    }
}

impl Item for u32 {
    fn from_slice(slice: &[u8]) -> Self {
        LE::read_u32(slice)
    }
}

impl Item for f32 {
    fn from_slice(slice: &[u8]) -> Self {
        LE::read_f32(slice)
    }
}

impl<T: Item> Item for [T; 2] {
    fn from_slice(slice: &[u8]) -> Self {
        assert!(slice.len() >= 2 * mem::size_of::<T>());
        [T::from_slice(slice),
         T::from_slice(&slice[mem::size_of::<T>() ..])]
    }
}

impl<T: Item> Item for [T; 3] {
    fn from_slice(slice: &[u8]) -> Self {
        assert!(slice.len() >= 3 * mem::size_of::<T>());
        [T::from_slice(slice),
         T::from_slice(&slice[1 * mem::size_of::<T>() ..]),
         T::from_slice(&slice[2 * mem::size_of::<T>() ..])]
    }
}

impl<T: Item> Item for [T; 4] {
    fn from_slice(slice: &[u8]) -> Self {
        assert!(slice.len() >= 4 * mem::size_of::<T>());
        [T::from_slice(slice),
         T::from_slice(&slice[1 * mem::size_of::<T>() ..]),
         T::from_slice(&slice[2 * mem::size_of::<T>() ..]),
         T::from_slice(&slice[3 * mem::size_of::<T>() ..])]
    }
}

impl<'a, T> Iter<'a, T> {
    /// Constructor.
    pub fn new(
        accessor: super::Accessor,
        buffer_data: &'a [u8],
    ) -> Iter<'a, T> {
        debug_assert_eq!(mem::size_of::<T>(), accessor.size());
        debug_assert!(mem::size_of::<T>() > 0);
        let view = accessor.view();
        let stride = view.stride().unwrap_or(mem::size_of::<T>());
        debug_assert!(stride >= mem::size_of::<T>());
        let start = view.offset() + accessor.offset();
        let end = start + stride * (accessor.count() - 1) + mem::size_of::<T>();
        let data = &buffer_data[start .. end];
        Iter { stride, data, _phantom: PhantomData }
    }
}

impl<'a, T: Item> ExactSizeIterator for Iter<'a, T> {}
impl<'a, T: Item> Iterator for Iter<'a, T> {
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
