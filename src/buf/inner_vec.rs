extern crate alloc;

use alloc::{vec, vec::Vec};
use ixy::index::RowMajor;

use crate::core::Pos;

/// A 2-dimensional grid implemented by a vector buffer.
///
/// This is a convenience type for using `Vec` as the underlying buffer.
///
/// ## Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`][].
///
/// [`Layout`]: `ixy::index::Layout`
pub type VecGrid<T, L> = super::GridBuf<T, Vec<T>, L>;

impl<T> super::GridBuf<T, Vec<T>, RowMajor>
where
    T: Copy,
{
    /// Creates a new `GridBuf` backed by a `Vec` with the specified width and height.
    ///
    /// Each element is initialized to the default value of `T` and is stored in [`RowMajor`] order.
    #[must_use]
    pub fn new_row_major(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self::new(width, height)
    }

    /// Creates a new `GridBuf` backed by a `Vec` with the specified width and height.
    ///
    /// Each element is initialized to the provided value and is stored in [`RowMajor`] order.
    #[must_use]
    pub fn new_filled_row_major(width: usize, height: usize, value: T) -> Self
    where
        T: Copy,
    {
        Self::new_filled(width, height, value)
    }
}

impl<T, L> super::GridBuf<T, Vec<T>, L>
where
    T: Copy,
    L: super::Layout,
{
    /// Creates a new `GridBuf` backed by a `Vec` with the specified width and height.
    ///
    /// Each element is initialized to the default value of `T`.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self::new_filled(width, height, T::default())
    }

    /// Creates a new `GridBuf` backed by a `Vec` with the specified width and height.
    ///
    /// Each element is initialized to the provided value.
    #[must_use]
    pub fn new_filled(width: usize, height: usize, value: T) -> Self {
        let size = width * height;
        let buffer = vec![value; size];
        unsafe { Self::with_buffer_unchecked(width, height, buffer) }
    }

    /// Creates a new `GridBuf` backed by a `Vec` with the specified width and height.
    ///
    /// The provided function is used to initialize an element at each position.
    #[must_use]
    pub fn new_generate(width: usize, height: usize, mut f: impl FnMut(Pos) -> T) -> Self {
        let size = width * height;
        let mut buffer = Vec::with_capacity(size);
        for y in 0..height {
            for x in 0..width {
                buffer.push(f(Pos::new(x, y)));
            }
        }
        unsafe { Self::with_buffer_unchecked(width, height, buffer) }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::core::Pos;

    use super::*;
    use alloc::{vec, vec::Vec};

    #[test]
    fn impl_vec() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = VecGrid::with_buffer_row_major(2, 3, data).unwrap();

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&1));
        assert_eq!(grid.get(Pos::new(1, 2)), Some(&6));
    }

    #[test]
    fn vec_new() {
        let grid = VecGrid::new_row_major(10, 5);
        assert_eq!(grid.get(Pos::new(3, 4)), Some(&0));
    }

    #[test]
    fn vec_new_filled() {
        let grid = VecGrid::new_filled_row_major(10, 5, 42);
        assert_eq!(grid.get(Pos::new(3, 4)), Some(&42));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "Buffer size does not match grid dimensions")]
    fn with_buffer_unchecked_panics() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5];
        // width * height = 6, but data.len() = 5
        let _ = unsafe { VecGrid::with_buffer_row_major_unchecked(2, 3, data) };
    }

    #[test]
    fn out_of_bounds() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = VecGrid::with_buffer_row_major(2, 2, data);
        assert!(grid.is_err());
    }

    #[test]
    fn into_inner() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = VecGrid::with_buffer_row_major(2, 3, data).unwrap();
        let (buffer, width, height) = grid.into_inner();

        assert_eq!(width, 2);
        assert_eq!(height, 3);
        assert_eq!(buffer.len(), 6);
        assert_eq!(buffer[0], 1);
    }

    #[test]
    fn iter() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = VecGrid::with_buffer_row_major(2, 3, data).unwrap();

        let mut iter = grid.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn into_iter() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = VecGrid::with_buffer_row_major(2, 3, data).unwrap();

        let mut iter = grid.into_iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let mut grid = VecGrid::with_buffer_row_major(2, 3, data).unwrap();

        #[allow(clippy::explicit_iter_loop)]
        for value in grid.iter_mut() {
            *value += 1; // Increment each value
        }

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&2));
        assert_eq!(grid.get(Pos::new(1, 2)), Some(&7));
    }

    #[test]
    fn into_iter_mut() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let mut grid = VecGrid::with_buffer_row_major(2, 3, data).unwrap();

        for value in &mut grid {
            *value += 1; // Increment each value
        }

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&2));
        assert_eq!(grid.get(Pos::new(1, 2)), Some(&7));
    }

    #[test]
    fn get_out_of_bounds() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = VecGrid::with_buffer_row_major(2, 3, data).unwrap();

        assert_eq!(grid.get(Pos::new(2, 0)), None); // Out of bounds
        assert_eq!(grid.get(Pos::new(0, 3)), None); // Out of bounds
    }

    #[test]
    fn get_mut_out_of_bounds() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let mut grid = VecGrid::with_buffer_row_major(2, 3, data).unwrap();
        assert_eq!(grid.get_mut(Pos::new(2, 0)), None); // Out of bounds
        assert_eq!(grid.get_mut(Pos::new(0, 3)), None); // Out of bounds
    }

    #[test]
    fn new_generate() {
        let grid = VecGrid::<usize, RowMajor>::new_generate(3, 2, |pos| pos.x + pos.y * 3);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&0));
        assert_eq!(grid.get(Pos::new(1, 1)), Some(&4));
        assert_eq!(grid.get(Pos::new(2, 1)), Some(&5));
    }
}
