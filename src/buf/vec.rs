extern crate alloc;

use alloc::{vec, vec::Vec};
use ixy::index::RowMajor;

/// A 2-dimensional grid implemented by a vector buffer.
///
/// This is a convenience type for using `Vec` as the underlying buffer.
///
/// # Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
pub type VecGrid<T, L = RowMajor> = super::GridBuf<T, Vec<T>, L>;

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
        unsafe { Self::with_buffer_unchecked(buffer, width, height) }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::Pos;
    use alloc::{vec, vec::Vec};

    #[test]
    fn impl_vec() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = VecGrid::<_>::with_buffer(data, 2, 3).unwrap();

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&1));
        assert_eq!(grid.get(Pos::new(1, 2)), Some(&6));
    }

    #[test]
    fn vec_new() {
        let grid = VecGrid::<_>::new(10, 5);
        assert_eq!(grid.get(Pos::new(3, 4)), Some(&0));
    }

    #[test]
    fn vec_new_filled() {
        let grid = VecGrid::<_>::new_filled(10, 5, 42);
        assert_eq!(grid.get(Pos::new(3, 4)), Some(&42));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "Buffer size does not match grid dimensions")]
    fn with_buffer_unchecked_panics() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5];
        // width * height = 6, but data.len() = 5
        let _ = unsafe { VecGrid::<_>::with_buffer_unchecked(data, 2, 3) };
    }

    #[test]
    fn out_of_bounds() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = VecGrid::<_>::with_buffer(data, 2, 2);
        assert!(grid.is_err());
    }

    #[test]
    fn into_inner() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = VecGrid::<_>::with_buffer(data, 2, 3).unwrap();
        let (buffer, width, height) = grid.into_inner();

        assert_eq!(width, 2);
        assert_eq!(height, 3);
        assert_eq!(buffer.len(), 6);
        assert_eq!(buffer[0], 1);
    }

    #[test]
    fn iter() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = VecGrid::<_>::with_buffer(data, 2, 3).unwrap();

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
        let grid = VecGrid::<_>::with_buffer(data, 2, 3).unwrap();

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
        let mut grid = VecGrid::<_>::with_buffer(data, 2, 3).unwrap();

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
        let mut grid = VecGrid::<_>::with_buffer(data, 2, 3).unwrap();

        for value in &mut grid {
            *value += 1; // Increment each value
        }

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&2));
        assert_eq!(grid.get(Pos::new(1, 2)), Some(&7));
    }

    #[test]
    fn get_out_of_bounds() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = VecGrid::<_>::with_buffer(data, 2, 3).unwrap();

        assert_eq!(grid.get(Pos::new(2, 0)), None); // Out of bounds
        assert_eq!(grid.get(Pos::new(0, 3)), None); // Out of bounds
    }

    #[test]
    fn get_mut_out_of_bounds() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let mut grid = VecGrid::<_>::with_buffer(data, 2, 3).unwrap();
        assert_eq!(grid.get_mut(Pos::new(2, 0)), None); // Out of bounds
        assert_eq!(grid.get_mut(Pos::new(0, 3)), None); // Out of bounds
    }
}
