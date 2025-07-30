use ixy::index::Layout;

use crate::buf::GridBuf;

/// A 2-dimensional grid implemented by a fixed-size array buffer.
///
/// This is a convenience type for using arrays as the underlying buffer.
///
/// ## Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
pub type ArrayGrid<T, const N: usize, L> = GridBuf<T, [T; N], L>;

impl<T, const N: usize, L> super::GridBuf<T, [T; N], L>
where
    T: Copy,
    L: Layout,
{
    /// Creates a new `GridBuf` backed by a fixed-size array with the specified width and height.
    ///
    /// Each element is initialized to the default value of `T`.
    ///
    /// ## Panics
    ///
    /// Panics if the buffer size does not match the expected size.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self::new_filled(width, height, T::default())
    }

    /// Creates a new `GridBuf` backed by a fixed-size array with the specified width and height.
    ///
    /// Each element is initialized to the provided value.
    ///
    /// ## Panics
    ///
    /// Panics if the buffer size does not match the expected size.
    #[must_use]
    pub fn new_filled(width: usize, height: usize, value: T) -> Self {
        let size = width * height;
        let buffer = [value; N];
        assert!(size <= N, "Buffer size does not match grid dimensions");
        unsafe { Self::with_buffer_unchecked(width, height, buffer) }
    }
}

#[cfg(test)]
mod tests {
    use ixy::index::RowMajor;

    use crate::{buf::ArrayGrid, core::Pos};

    #[test]
    fn impl_arr() {
        let data: [u8; 6] = [1, 2, 3, 4, 5, 6];
        let grid = ArrayGrid::with_buffer_row_major(2, 3, data).unwrap();

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&1));
        assert_eq!(grid.get(Pos::new(1, 2)), Some(&6));
    }

    #[test]
    fn arr_new() {
        let grid = ArrayGrid::<u8, 6, RowMajor>::new(2, 3);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&0));
    }

    #[test]
    fn arr_new_filled() {
        let grid = ArrayGrid::<u8, 6, RowMajor>::new_filled(2, 3, 42);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&42));
    }

    #[test]
    #[should_panic(expected = "Buffer size does not match grid dimensions")]
    fn arr_new_panics() {
        let _grid = ArrayGrid::<u8, 5, RowMajor>::new(2, 3); // This should panic
    }
}
