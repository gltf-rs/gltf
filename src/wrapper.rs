use crate::Root;

/// Wrapper type.
pub trait Wrap<'a> {
    /// The wrapper type.
    type Wrapped;

    /// Creates a wrapper type that can resolve references to other
    /// types in the glTF hierarchy.
    fn wrap(&'a self, root: &'a Root) -> Self::Wrapped;

    /// Creates a wrapper type associated with an index.
    fn wrap_indexed(&'a self, root: &'a Root, index: usize) -> Self::Wrapped {
        let _ = index;
        self.wrap(root)
    }
}

impl<'a, T> Wrap<'a> for &'a T
where
    T: 'a + Wrap<'a>,
{
    type Wrapped = <T as Wrap<'a>>::Wrapped;

    fn wrap(&'a self, root: &'a Root) -> Self::Wrapped {
        (*self).wrap(root)
    }
}

impl<'a, T: Copy, const N: usize> Wrap<'a> for [T; N] {
    type Wrapped = Self;

    fn wrap(&'a self, _root: &'a Root) -> Self::Wrapped {
        *self
    }
}

impl<'a, T> Wrap<'a> for Option<T>
where
    T: 'a + Wrap<'a>,
{
    type Wrapped = Option<<T as Wrap<'a>>::Wrapped>;

    fn wrap(&'a self, root: &'a Root) -> Self::Wrapped {
        self.as_ref().map(|item| item.wrap(root))
    }
}

impl<'a> Wrap<'a> for String {
    type Wrapped = &'a str;

    fn wrap(&'a self, _root: &'a Root) -> Self::Wrapped {
        self
    }
}

impl<'a> Wrap<'a> for std::boxed::Box<serde_json::value::RawValue> {
    type Wrapped = &'a serde_json::value::RawValue;

    fn wrap(&'a self, _root: &'a Root) -> Self::Wrapped {
        use std::ops::Deref;
        self.deref()
    }
}

/// Iterator over maps of wrapper data.
#[derive(Clone, Debug)]
pub struct BTreeMapIter<'a, K: Wrap<'a>, V: Wrap<'a>>(
    &'a Root,
    std::collections::btree_map::Iter<'a, K, V>,
);

impl<'a, K: 'a + Wrap<'a>, V: 'a + Wrap<'a>> Iterator for BTreeMapIter<'a, K, V> {
    type Item = (<K as Wrap<'a>>::Wrapped, <V as Wrap<'a>>::Wrapped);

    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|(k, v)| (k.wrap(self.0), v.wrap(self.0)))
    }
}

/// Iterator over maps of wrapper data.
#[derive(Clone, Debug)]
pub struct MapIter<'a, K: Wrap<'a>, V: Wrap<'a>>(
    &'a Root,
    std::collections::hash_map::Iter<'a, K, V>,
);

impl<'a, K: 'a + Wrap<'a>, V: 'a + Wrap<'a>> Iterator for MapIter<'a, K, V> {
    type Item = (<K as Wrap<'a>>::Wrapped, <V as Wrap<'a>>::Wrapped);

    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|(k, v)| (k.wrap(self.0), v.wrap(self.0)))
    }
}

/// Iterator over slices of wrappable data.
#[derive(Clone, Debug)]
pub struct SliceIter<'a, T: Wrap<'a>>(&'a Root, std::iter::Enumerate<std::slice::Iter<'a, T>>);

impl<'a, T: Wrap<'a>> Iterator for SliceIter<'a, T> {
    type Item = <T as Wrap<'a>>::Wrapped;

    fn next(&mut self) -> Option<Self::Item> {
        self.1
            .next()
            .map(|(index, item)| item.wrap_indexed(self.0, index))
    }
}

impl<'a, T: Wrap<'a> + 'a> Wrap<'a> for Vec<T> {
    type Wrapped = SliceIter<'a, T>;

    fn wrap(&'a self, root: &'a Root) -> Self::Wrapped {
        SliceIter(root, self.iter().enumerate())
    }
}
