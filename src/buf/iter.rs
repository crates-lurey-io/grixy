use crate::GridBuf;
use ixy::index::Layout;

impl<'a, T, B, L> IntoIterator for &'a GridBuf<T, B, L>
where
    B: AsRef<[T]> + AsMut<[T]>,
    L: Layout,
{
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.as_ref().iter()
    }
}

impl<'a, T, B, L> IntoIterator for &'a mut GridBuf<T, B, L>
where
    B: AsRef<[T]> + AsMut<[T]>,
    L: Layout,
{
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.as_mut().iter_mut()
    }
}
