//! Provides a generic, 2D grid data structure backed by a linear buffer.
//!
//! The main type, [`GridBuf`], is highly generic, allowing it to act as a view over any buffer
//! that can be treated as a slice.
//!
//! ## Convenience Types
//!
//! For ease of use, several type aliases are provided for common use cases:
//! - [`VecGrid`]: An owned grid backed by a `Vec<T>` (requires the `alloc` feature).
//! - [`ArrayGrid`]: An owned grid backed by a fixed-size array, `[T; N]`.
//! - [`SliceGrid`]: A read-only, borrowed view over an existing slice.
//! - [`SliceMutGrid`]: A mutable, borrowed view over an existing slice.
//!
//! # Examples
//!
//! Creating an owned `VecGrid` and accessing an element:
//! ```
//! use grixy::{Pos, buf::VecGrid};
//!
//! let grid = VecGrid::<_>::new_filled(10, 5, 42);
//! assert_eq!(grid.get(Pos::new(3, 4)), Some(&42));
//! ```

use crate::{Pos, error::GridError};
use core::marker::PhantomData;
pub use ixy::index::{ColMajor, Layout, RowMajor};

#[cfg(feature = "alloc")]
mod alloc {
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
}

#[cfg(feature = "alloc")]
pub use alloc::VecGrid;

/// A 2-dimensional grid implemented by a fixed-size array buffer.
///
/// This is a convenience type for using arrays as the underlying buffer.
///
/// # Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
pub type ArrayGrid<T, const N: usize, L = RowMajor> = GridBuf<T, [T; N], L>;

impl<T, const N: usize, L> super::GridBuf<T, [T; N], L>
where
    T: Copy,
    L: Layout,
{
    /// Creates a new `GridBuf` backed by a fixed-size array with the specified width and height.
    ///
    /// Each element is initialized to the default value of `T`.
    ///
    /// # Panics
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
    /// # Panics
    ///
    /// Panics if the buffer size does not match the expected size.
    #[must_use]
    pub fn new_filled(width: usize, height: usize, value: T) -> Self {
        let size = width * height;
        let buffer = [value; N];
        assert!(size <= N, "Buffer size does not match grid dimensions");
        unsafe { Self::with_buffer_unchecked(buffer, width, height) }
    }
}

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

/// A 2-dimensional grid implemented by a linear data buffer.
///
/// # Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
pub struct GridBuf<T, B, L = RowMajor>
where
    B: AsRef<[T]>,
    L: Layout,
{
    buffer: B,
    width: usize,
    height: usize,
    _element: PhantomData<T>,
    _layout: PhantomData<L>,
}

impl<T, B, L> GridBuf<T, B, L>
where
    B: AsRef<[T]>,
    L: Layout,
{
    /// Creates a `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer size does not match the expected size.
    pub fn with_buffer(buffer: B, width: usize, height: usize) -> Result<Self, GridError> {
        let expected_size = width * height;
        if buffer.as_ref().len() != expected_size {
            return Err(GridError);
        }
        Ok(unsafe { Self::with_buffer_unchecked(buffer, width, height) })
    }

    /// Creates a new `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the buffer is large enough to hold `width * height` elements.
    pub unsafe fn with_buffer_unchecked(buffer: B, width: usize, height: usize) -> Self {
        debug_assert_eq!(
            buffer.as_ref().len(),
            width * height,
            "Buffer size does not match grid dimensions"
        );
        Self {
            buffer,
            width,
            height,
            _element: PhantomData,
            _layout: PhantomData,
        }
    }

    /// Returns a reference of the element at the specified position.'
    ///
    /// If the position is out of bounds, returns `None`.
    pub fn get(&self, pos: Pos) -> Option<&T> {
        if pos.x < self.width && pos.y < self.height {
            Some(&self.buffer.as_ref()[L::to_1d(pos, self.width).index])
        } else {
            None
        }
    }

    /// Consumes the `GridBuf`, returning the underlying buffer, width, and height.
    #[must_use]
    pub fn into_inner(self) -> (B, usize, usize) {
        (self.buffer, self.width, self.height)
    }

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
    B: AsRef<[T]> + AsMut<[T]>,
    L: Layout,
{
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.as_ref().iter()
    }
}

impl<T, B, L> GridBuf<T, B, L>
where
    B: AsRef<[T]> + AsMut<[T]>,
    L: Layout,
{
    /// Returns a mutable reference of the element at the specified position.
    ///
    /// If the position is out of bounds, returns `None`.
    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut T> {
        if pos.x < self.width && pos.y < self.height {
            Some(&mut self.buffer.as_mut()[L::to_1d(pos, self.width).index])
        } else {
            None
        }
    }

    /// Returns an iterator that allows modifying each element in the grid.
    ///
    /// The iterator yields mutable references in the order defined by the layout.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.buffer.as_mut().iter_mut()
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

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
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
    fn impl_arr() {
        let data: [u8; 6] = [1, 2, 3, 4, 5, 6];
        let grid = ArrayGrid::<_, 6>::with_buffer(data, 2, 3).unwrap();

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&1));
        assert_eq!(grid.get(Pos::new(1, 2)), Some(&6));
    }

    #[test]
    fn arr_new() {
        let grid = ArrayGrid::<_, 6>::new(2, 3);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&0));
    }

    #[test]
    fn arr_new_filled() {
        let grid = ArrayGrid::<_, 6>::new_filled(2, 3, 42);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&42));
    }

    #[test]
    #[should_panic(expected = "Buffer size does not match grid dimensions")]
    fn arr_new_panics() {
        let _grid = ArrayGrid::<u8, 5>::new(2, 3); // This should panic
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "Buffer size does not match grid dimensions")]
    fn with_buffer_unchecked_panics() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5];
        // width * height = 6, but data.len() = 5
        let _ = unsafe { GridBuf::<u8, Vec<u8>, RowMajor>::with_buffer_unchecked(data, 2, 3) };
    }

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

    #[test]
    fn out_of_bounds() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = GridBuf::<u8, Vec<u8>, RowMajor>::with_buffer(data, 2, 2);
        assert!(grid.is_err());
    }

    #[test]
    fn into_inner() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = GridBuf::<u8, Vec<u8>, RowMajor>::with_buffer(data, 2, 3).unwrap();
        let (buffer, width, height) = grid.into_inner();

        assert_eq!(width, 2);
        assert_eq!(height, 3);
        assert_eq!(buffer.len(), 6);
        assert_eq!(buffer[0], 1);
    }

    #[test]
    fn iter() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = GridBuf::<u8, Vec<u8>, RowMajor>::with_buffer(data, 2, 3).unwrap();

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
        let grid = GridBuf::<u8, Vec<u8>, RowMajor>::with_buffer(data, 2, 3).unwrap();

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
        let mut grid = GridBuf::<u8, Vec<u8>, RowMajor>::with_buffer(data, 2, 3).unwrap();

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
        let mut grid = GridBuf::<u8, Vec<u8>, RowMajor>::with_buffer(data, 2, 3).unwrap();

        for value in &mut grid {
            *value += 1; // Increment each value
        }

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&2));
        assert_eq!(grid.get(Pos::new(1, 2)), Some(&7));
    }

    #[test]
    fn get_out_of_bounds() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let grid = GridBuf::<u8, Vec<u8>, RowMajor>::with_buffer(data, 2, 3).unwrap();

        assert_eq!(grid.get(Pos::new(2, 0)), None); // Out of bounds
        assert_eq!(grid.get(Pos::new(0, 3)), None); // Out of bounds
    }

    #[test]
    fn get_mut_out_of_bounds() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let mut grid = GridBuf::<u8, Vec<u8>, RowMajor>::with_buffer(data, 2, 3).unwrap();
        assert_eq!(grid.get_mut(Pos::new(2, 0)), None); // Out of bounds
        assert_eq!(grid.get_mut(Pos::new(0, 3)), None); // Out of bounds
    }
}
