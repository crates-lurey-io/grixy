#[cfg(feature = "alloc")]
extern crate alloc;

use crate::{buf::GridBuf, ops::layout};
use core::marker::PhantomData;

impl<T, B, L> GridBuf<T, B, L>
where
    B: AsRef<[T]>,
    L: layout::Linear,
{
    /// Returns a grid from an existing buffer with a given width in columns.
    ///
    /// The height is inferred from the buffer length and width.
    ///
    /// Any data type that can be represented as a slice can be used as the buffer type, including
    /// arrays, slices, and vectors.
    ///
    /// ## Panics
    ///
    /// This panics if the buffer length is not a multiple of the width.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use grixy::prelude::*;
    ///
    /// let buffer = vec![1, 2, 3, 4, 5, 6];
    /// let grid = GridBuf::<_, _, RowMajor>::from_buffer(buffer, 3);
    ///
    /// assert_eq!(grid.get(Pos::new(0, 0)), Some(&1));
    /// assert_eq!(grid.get(Pos::new(1, 0)), Some(&2));
    /// assert_eq!(grid.get(Pos::new(2, 1)), Some(&6));
    /// assert_eq!(grid.get(Pos::new(3, 1)), None); // Out of bounds
    /// ```
    #[must_use]
    pub fn from_buffer(buffer: B, width: usize) -> Self {
        let height = buffer.as_ref().len() / width;
        assert!(
            height * width == buffer.as_ref().len(),
            "Buffer length must be a multiple of width"
        );
        Self {
            buffer,
            width,
            height,
            _layout: PhantomData,
            _element: PhantomData,
        }
    }
}

#[cfg(feature = "alloc")]
impl<T> GridBuf<T, alloc::vec::Vec<T>, layout::RowMajor> {
    /// Creates a new grid with the specified width and height, filled with a default value.
    ///
    /// This creates a grid with a row-major layout; see [`new_filled_with_layout`][] to customize.
    ///
    /// [`new_filled_with_layout`]: GridBuf::new_filled_with_layout
    ///
    /// ## Example
    ///
    /// ```rust
    /// use grixy::prelude::*;
    ///
    /// let grid = GridBuf::<u8, _, _>::new(3, 3);
    /// assert_eq!(grid.get(Pos::new(0, 0)), Some(&0));
    /// assert_eq!(grid.get(Pos::new(2, 2)), Some(&0));
    /// assert_eq!(grid.get(Pos::new(3, 3)), None); // Out of bounds
    /// ```
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Copy + Default,
    {
        Self::new_filled(width, height, T::default())
    }
    /// Creates a new grid with the specified width and height, filled with a specified value.
    ///
    /// This creates a grid with a row-major layout; see [`new_filled_with_layout`][] to customize.
    ///
    /// [`new_filled_with_layout`]: GridBuf::new_filled_with_layout
    ///
    /// ## Example
    ///
    /// ```rust
    /// use grixy::prelude::*;
    ///
    /// let grid = GridBuf::new_filled(5, 4, 42);
    ///
    /// assert_eq!(grid.get(Pos::new(0, 0)), Some(&42));
    /// assert_eq!(grid.get(Pos::new(4, 3)), Some(&42));
    /// assert_eq!(grid.get(Pos::new(5, 3)), None); // Out of bounds
    /// ```
    #[must_use]
    pub fn new_filled(width: usize, height: usize, value: T) -> Self
    where
        T: Copy,
    {
        let buffer = alloc::vec![value; width * height];
        Self {
            buffer,
            width,
            height,
            _layout: PhantomData,
            _element: PhantomData,
        }
    }
}

#[cfg(feature = "alloc")]
impl<T, L> GridBuf<T, alloc::vec::Vec<T>, L>
where
    L: layout::Linear,
{
    /// Creates a new grid with the specified width and height, filled with a default value.
    ///
    /// The layout is specified by the type parameter `L`, allowing for different memory layouts.
    ///
    /// ## Example
    #[must_use]
    pub fn new_filled_with_layout(width: usize, height: usize, value: T) -> Self
    where
        T: Copy,
        L: layout::Linear,
    {
        let buffer = alloc::vec![value; width * height];
        Self {
            buffer,
            width,
            height,
            _layout: PhantomData,
            _element: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::{core::Pos, ops::GridRead as _, ops::layout::RowMajor};
    use alloc::vec;

    #[test]
    #[should_panic(expected = "Buffer length must be a multiple of width")]
    fn test_from_buffer_panics_on_invalid_length() {
        let buffer = vec![1, 2, 3];
        let _grid = GridBuf::<_, _, RowMajor>::from_buffer(buffer, 2);
    }

    #[test]
    fn new_filled_with_layout() {
        let grid = GridBuf::<_, _, RowMajor>::new_filled_with_layout(3, 2, 42);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&42));
        assert_eq!(grid.get(Pos::new(2, 1)), Some(&42));
        assert_eq!(grid.get(Pos::new(3, 1)), None); // Out of bounds
    }
}
