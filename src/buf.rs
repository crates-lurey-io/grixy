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
//! let grid = VecGrid::<_>::new_filled(10, 5, 42);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_bounded_grid() {
        let grid = VecGrid::<u8>::new(5, 4);
        assert_eq!(grid.width(), 5);
        assert_eq!(grid.height(), 4);
    }

    #[test]
    fn impl_get_unchecked() {
        let grid = VecGrid::<u8>::new_filled(5, 4, 42);
        let pos = Pos::new(2, 3);
        assert_eq!(unsafe { grid.get_unchecked(pos) }, &42);
    }

    #[test]
    fn impl_set_unchecked() {
        let mut grid = VecGrid::<u8>::new(5, 4);
        let pos = Pos::new(2, 3);
        unsafe { grid.set_unchecked(pos, 99) };
        assert_eq!(unsafe { grid.get_unchecked(pos) }, &99);
    }
}
