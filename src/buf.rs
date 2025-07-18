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
//! use grixy::{core::Pos, buf::VecGrid};
//!
//! let grid = VecGrid::new_filled_row_major(10, 5, 42);
//! assert_eq!(grid.get(Pos::new(3, 4)), Some(&42));
//! ```

use crate::{
    core::{GridError, Layout, Pos, RowMajor},
    grid::{BoundedGrid, GridBase, GridReadUnchecked, GridWriteUnchecked},
};
use core::marker::PhantomData;

mod array;
pub use array::ArrayGrid;

pub mod bits;

mod iter;

mod r#mut;

mod slice;
use ixy::index::ColMajor;
pub use slice::*;

#[cfg(feature = "alloc")]
mod vec;

#[cfg(feature = "alloc")]
pub use vec::VecGrid;

/// A 2-dimensional grid implemented by a linear data buffer.
///
/// ## Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
#[derive(Debug, Clone)]
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

impl<T, B> GridBuf<T, B, RowMajor>
where
    B: AsRef<[T]>,
{
    /// Creates a `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// The data buffer is expected to be in [`RowMajor`] order.
    ///
    /// ## Errors
    ///
    /// Returns an error if the buffer size does not match the expected size.
    pub fn with_buffer_row_major(
        buffer: B,
        width: usize,
        height: usize,
    ) -> Result<Self, GridError> {
        Self::with_buffer(buffer, width, height)
    }

    /// Creates a new `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// The data buffer is expected to be in [`RowMajor`] order.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that the buffer is large enough to hold `width * height` elements.
    pub unsafe fn with_buffer_row_major_unchecked(buffer: B, width: usize, height: usize) -> Self {
        unsafe { Self::with_buffer_unchecked(buffer, width, height) }
    }
}

impl<T, B> GridBuf<T, B, ColMajor>
where
    B: AsRef<[T]>,
{
    /// Creates a `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// The data buffer is expected to be in [`ColMajor`] order.
    ///
    /// ## Errors
    ///
    /// Returns an error if the buffer size does not match the expected size.
    pub fn with_buffer_col_major(
        buffer: B,
        width: usize,
        height: usize,
    ) -> Result<Self, GridError> {
        Self::with_buffer(buffer, width, height)
    }

    /// Creates a new `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// The data buffer is expected to be in [`ColMajor`] order.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that the buffer is large enough to hold `width * height` elements.
    pub unsafe fn with_buffer_col_major_unchecked(buffer: B, width: usize, height: usize) -> Self {
        unsafe { Self::with_buffer_unchecked(buffer, width, height) }
    }
}

impl<T, B, L> GridBuf<T, B, L>
where
    B: AsRef<[T]>,
    L: Layout,
{
    /// Creates a `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// ## Errors
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
    /// ## Safety
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

impl<T, B, L> GridBase for GridBuf<T, B, L>
where
    B: AsRef<[T]>,
    L: Layout,
{
    type Element = T;
}

unsafe impl<T, B, L> BoundedGrid for GridBuf<T, B, L>
where
    B: AsRef<[T]>,
    L: Layout,
{
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<T, B, L> GridReadUnchecked for GridBuf<T, B, L>
where
    B: AsRef<[T]>,
    L: Layout,
{
    unsafe fn get_unchecked(&self, pos: Pos) -> &T {
        let index = L::to_1d(pos, self.width).index;
        unsafe { self.buffer.as_ref().get_unchecked(index) }
    }

    unsafe fn rect_iter_unchecked(
        &self,
        bounds: crate::core::Rect,
    ) -> impl Iterator<Item = &Self::Element> {
        let slice = self.buffer.as_ref();
        let width = self.width;
        (bounds.top()..bounds.bottom()).flat_map(move |y| {
            let row_start = L::to_1d(Pos::new(bounds.left(), y), width).index;
            slice[row_start..row_start + bounds.width()].iter()
        })
    }
}

impl<T, B, L> GridWriteUnchecked for GridBuf<T, B, L>
where
    B: AsRef<[T]> + AsMut<[T]>,
    L: Layout,
{
    unsafe fn set_unchecked(&mut self, pos: Pos, value: T) {
        let index = L::to_1d(pos, self.width).index;
        unsafe { *self.buffer.as_mut().get_unchecked_mut(index) = value }
    }

    unsafe fn fill_rect_iter_unchecked(
        &mut self,
        bounds: crate::core::Rect,
        iter: impl IntoIterator<Item = Self::Element>,
    ) {
        let slice = self.buffer.as_mut();
        let width = self.width;
        let mut iter = iter.into_iter();
        for y in bounds.top()..bounds.bottom() {
            let x_xtart = L::to_1d(Pos::new(bounds.left(), y), width).index;
            let x_end = x_xtart + bounds.width();
            slice[x_xtart..x_end]
                .iter_mut()
                .zip(&mut iter)
                .for_each(|(cell, value)| *cell = value);
        }
    }

    unsafe fn fill_rect_solid_unchecked(&mut self, bounds: crate::core::Rect, value: Self::Element)
    where
        Self::Element: Copy,
    {
        let slice = self.buffer.as_mut();
        let width = self.width;
        for y in bounds.top()..bounds.bottom() {
            let x_start = L::to_1d(Pos::new(bounds.left(), y), width).index;
            let x_end = x_start + bounds.width();
            slice[x_start..x_end].fill(value);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use crate::core::Rect;

    use super::*;
    use alloc::{vec, vec::Vec};

    #[test]
    fn impl_bounded_grid() {
        let grid = VecGrid::new_filled_row_major(5, 4, 0);
        assert_eq!(grid.width(), 5);
        assert_eq!(grid.height(), 4);
    }

    #[test]
    fn impl_get_unchecked() {
        let grid = VecGrid::new_filled_row_major(5, 4, 42);
        let pos = Pos::new(2, 3);
        assert_eq!(unsafe { grid.get_unchecked(pos) }, &42);
    }

    #[test]
    fn impl_set_unchecked() {
        let mut grid = VecGrid::new_row_major(5, 4);
        let pos = Pos::new(2, 3);
        unsafe { grid.set_unchecked(pos, 99) };
        assert_eq!(unsafe { grid.get_unchecked(pos) }, &99);
    }

    #[test]
    fn with_buffer_col_major() {
        let buffer = VecGrid::with_buffer_col_major(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3).unwrap();
        assert_eq!(buffer.width(), 3);
        assert_eq!(buffer.height(), 3);
        assert_eq!(buffer.get(Pos::new(0, 0)), Some(&1));
        assert_eq!(buffer.get(Pos::new(2, 2)), Some(&9));
    }

    #[test]
    fn with_buffer_col_major_unchecked() {
        let buffer = unsafe {
            VecGrid::with_buffer_col_major_unchecked(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3)
        };
        assert_eq!(buffer.width(), 3);
        assert_eq!(buffer.height(), 3);
        assert_eq!(unsafe { buffer.get_unchecked(Pos::new(0, 0)) }, &1);
        assert_eq!(unsafe { buffer.get_unchecked(Pos::new(2, 2)) }, &9);
    }

    #[test]
    fn rect_iter_unchecked() {
        #[rustfmt::skip]
        let buffer = VecGrid::with_buffer_row_major(vec![
            1, 2, 3, 
            4, 5, 6, 
            7, 8, 9,
        ], 3, 3).unwrap();

        assert_eq!(
            unsafe {
                buffer
                    .rect_iter_unchecked(Rect::from_ltwh(1, 1, 2, 1))
                    .collect::<Vec<_>>()
            },
            vec![&5, &6]
        );
        assert_eq!(
            unsafe {
                buffer
                    .rect_iter_unchecked(Rect::from_ltwh(0, 0, 3, 3))
                    .collect::<Vec<_>>()
            },
            vec![&1, &2, &3, &4, &5, &6, &7, &8, &9]
        );
    }

    #[test]
    fn fill_rect_iter_unchecked() {
        let mut grid = VecGrid::new_row_major(3, 3);
        unsafe {
            grid.fill_rect_iter_unchecked(Rect::from_ltwh(0, 0, 2, 2), vec![1, 2, 3, 4]);
        }
        #[rustfmt::skip]
        assert_eq!(grid.buffer.as_ref() as &[i32], &[
            1, 2, 0,
            3, 4, 0,
            0, 0, 0,
        ]);
    }

    #[test]
    fn fill_rect_solid_unchecked() {
        let mut grid = VecGrid::new_row_major(3, 3);
        unsafe {
            grid.fill_rect_solid_unchecked(Rect::from_ltwh(0, 0, 2, 2), 42);
        }
        #[rustfmt::skip]
        assert_eq!(grid.buffer.as_ref() as &[i32], &[
            42, 42, 0,
            42, 42, 0,
            0, 0, 0,
        ]);
    }
}
