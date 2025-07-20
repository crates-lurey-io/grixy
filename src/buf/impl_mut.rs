use crate::{buf::GridBuf, core::Pos};
use ixy::index::Layout;

impl<T, B, L> GridBuf<T, B, L>
where
    B: AsMut<[T]>,
    L: Layout,
{
    /// Returns a mutable reference of the element at the specified position.
    ///
    /// If the position is out of bounds, returns `None`.
    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut T> {
        if pos.x < self.width && pos.y < self.height {
            Some(&mut self.buffer.as_mut()[L::to_1d(pos, self.width)])
        } else {
            None
        }
    }

    /// Returns an iterator that allows modifying each element in the grid.
    ///
    /// The iterator yields mutable references in the order defined by the layout.
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.buffer.as_mut().iter_mut()
    }
}

impl<'a, T, B, L> IntoIterator for &'a mut GridBuf<T, B, L>
where
    B: AsMut<[T]>,
    L: Layout,
{
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.as_mut().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use ixy::index::RowMajor;

    use crate::buf::ArrayGrid;

    use super::*;

    #[test]
    fn get_mut_x_out_of_bounds() {
        let mut grid = ArrayGrid::<u8, 50, RowMajor>::new_filled(10, 5, 42);
        assert_eq!(grid.get_mut(Pos::new(10, 4)), None);
    }

    #[test]
    fn get_mut_y_out_of_bounds() {
        let mut grid = ArrayGrid::<u8, 50, RowMajor>::new_filled(10, 5, 42);
        assert_eq!(grid.get_mut(Pos::new(9, 5)), None);
    }

    #[test]
    fn get_mut() {
        let mut grid = ArrayGrid::<u8, 50, RowMajor>::new_filled(10, 5, 42);
        assert_eq!(grid.get_mut(Pos::new(0, 0)), Some(&mut 42));
        assert_eq!(grid.get_mut(Pos::new(9, 4)), Some(&mut 42));
    }
}
