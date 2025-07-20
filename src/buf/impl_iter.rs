use ixy::index::Layout;

use crate::buf::GridBuf;

impl<T, B, L> GridBuf<T, B, L>
where
    B: AsRef<[T]>,
    L: Layout,
{
    /// Returns an iterator over the elements of the grid.
    ///
    /// The iterator yields all items in the grid in the order defined by the layout.
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.buffer.as_ref().iter()
    }
}

impl<'a, T, B, L> IntoIterator for &'a GridBuf<T, B, L>
where
    B: AsRef<[T]>,
    L: Layout,
{
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.as_ref().iter()
    }
}
