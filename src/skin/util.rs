use crate::accessor;

use crate::{Buffer, Skin};

/// Inverse Bind Matrices of type `[[f32; 4]; 4]`.
pub type ReadInverseBindMatrices<'a> = accessor::Iter<'a, [[f32; 4]; 4]>;

/// Skin reader.
#[derive(Clone, Debug)]
pub struct Reader<'a, 's, F>
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    pub(crate) skin: Skin<'a>,
    pub(crate) get_buffer_data: F,
}

impl<'a, 's, F> Reader<'a, 's, F>
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    /// Returns an `Iterator` that reads the inverse bind matrices of
    /// the skin.
    pub fn read_inverse_bind_matrices(&self) -> Option<ReadInverseBindMatrices<'s>> {
        self.skin
            .inverse_bind_matrices()
            .and_then(|accessor| accessor::Iter::new(accessor, self.get_buffer_data.clone()))
    }
}
