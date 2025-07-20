use crate::buf::GridBuf;

/// A 2-dimensional grid implemented by a slice buffer.
///
/// This is a convenience type for using slices as the underlying buffer.
///
/// ## Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`][].
///
/// [`Layout`]: `ixy::index::Layout`
pub type SliceGrid<'a, T, L> = GridBuf<T, &'a [T], L>;

/// A 2-dimensional grid implemented by a mutable slice buffer.
///
/// This is a convenience type for using mutable slices as the underlying buffer.
///
/// ## Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`][].
///
/// [`Layout`]: `ixy::index::Layout`
pub type SliceMutGrid<'a, T, L> = GridBuf<T, &'a mut [T], L>;

#[cfg(test)]
mod tests {
    use crate::core::Pos;

    use super::*;

    #[test]
    fn impl_slice() {
        let data: &[u8] = &[1, 2, 3, 4, 5, 6];
        let grid = SliceGrid::with_buffer_row_major(data, 2, 3).unwrap();

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&1));
        assert_eq!(grid.get(Pos::new(1, 2)), Some(&6));
    }

    #[test]
    fn impl_slice_mut() {
        let mut data: [u8; 6] = [1, 2, 3, 4, 5, 6];
        let mut grid = SliceMutGrid::with_buffer_row_major(&mut data, 2, 3).unwrap();

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&1));
        assert_eq!(grid.get(Pos::new(1, 2)), Some(&6));

        *grid.get_mut(Pos::new(0, 0)).unwrap() = 10;
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&10));
    }
}
