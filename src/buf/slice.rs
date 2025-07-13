use crate::GridBuf;
use ixy::index::RowMajor;

/// A 2-dimensional grid implemented by a slice buffer.
///
/// This is a convenience type for using slices as the underlying buffer.
///
/// # Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
pub type SliceGrid<'a, T, L = RowMajor> = GridBuf<T, &'a [T], L>;

/// A 2-dimensional grid implemented by a mutable slice buffer.
///
/// This is a convenience type for using mutable slices as the underlying buffer.
///
/// # Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
pub type SliceMutGrid<'a, T, L = RowMajor> = GridBuf<T, &'a mut [T], L>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pos;

    #[test]
    fn impl_slice() {
        let data: &[u8] = &[1, 2, 3, 4, 5, 6];
        let grid = SliceGrid::<_>::with_buffer(data, 2, 3).unwrap();

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&1));
        assert_eq!(grid.get(Pos::new(1, 2)), Some(&6));
    }

    #[test]
    fn impl_slice_mut() {
        let mut data: [u8; 6] = [1, 2, 3, 4, 5, 6];
        let mut grid = SliceMutGrid::<_>::with_buffer(&mut data, 2, 3).unwrap();

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&1));
        assert_eq!(grid.get(Pos::new(1, 2)), Some(&6));

        *grid.get_mut(Pos::new(0, 0)).unwrap() = 10;
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&10));
    }
}
