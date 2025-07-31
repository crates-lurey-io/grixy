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
//! [`VecGrid`]: `crate::buf::VecGrid`
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
    core::{Pos, RowMajor},
    ops::GridDraw,
};
use core::marker::PhantomData;

// IMPLEMENATIONS ----------------------------------------------------------------------------------

pub mod bits;

mod inner_array;
pub use inner_array::ArrayGrid;

#[cfg(feature = "alloc")]
mod inner_vec;

#[cfg(feature = "alloc")]
pub use inner_vec::VecGrid;

mod inner_slice;
pub use inner_slice::{SliceGrid, SliceMutGrid};
use ixy::index::Layout;

// TRAIT IMPLS -------------------------------------------------------------------------------------

pub use crate::ops::unchecked::TrustedSizeGrid as _;

mod impl_as_slice;

mod impl_grid;
mod impl_iter;
mod impl_mut;
mod impl_new;

/// A 2-dimensional grid implemented by a linear data buffer.
///
/// ## Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
#[derive(Debug, Clone)]
pub struct GridBuf<T, B, L = RowMajor>
where
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
    L: Layout,
{
    /// Consumes the `GridBuf`, returning the underlying buffer, width, and height.
    #[must_use]
    pub fn into_inner(self) -> (B, usize, usize) {
        (self.buffer, self.width, self.height)
    }
}

impl<T, B, L> GridBuf<T, B, L>
where
    B: AsRef<[T]>,
    L: Layout,
{
    /// Returns a reference of the element at the specified position.'
    ///
    /// If the position is out of bounds, returns `None`.
    pub fn get(&self, pos: Pos) -> Option<&T> {
        if pos.x < self.width && pos.y < self.height {
            Some(&self.buffer.as_ref()[L::to_1d(pos, self.width)])
        } else {
            None
        }
    }
}

impl<T, B, L> GridDraw for GridBuf<T, B, L>
where
    T: Copy,
    B: AsMut<[T]> + AsRef<[T]>,
    L: Layout,
{
    // TODO: Optimize for the linear buffer layout.
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;
    use crate::{
        core::Rect,
        ops::unchecked::{GridReadUnchecked as _, GridWriteUnchecked as _},
    };
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
        assert_eq!(*unsafe { grid.get_unchecked(pos) }, 42);
    }

    #[test]
    fn impl_set_unchecked() {
        let mut grid = VecGrid::new_row_major(5, 4);
        let pos = Pos::new(2, 3);
        unsafe { grid.set_unchecked(pos, 99) };
        assert_eq!(*unsafe { grid.get_unchecked(pos) }, 99);
    }

    #[test]
    fn with_buffer_col_major() {
        let buffer = VecGrid::with_buffer_col_major(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        assert_eq!(buffer.width(), 3);
        assert_eq!(buffer.height(), 3);
        assert_eq!(buffer.get(Pos::new(0, 0)), Some(&1));
        assert_eq!(buffer.get(Pos::new(2, 2)), Some(&9));
    }

    #[test]
    fn with_buffer_col_major_unchecked() {
        let buffer = unsafe {
            VecGrid::with_buffer_col_major_unchecked(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        };
        assert_eq!(buffer.width(), 3);
        assert_eq!(buffer.height(), 3);
        assert_eq!(*unsafe { buffer.get_unchecked(Pos::new(0, 0)) }, 1);
        assert_eq!(*unsafe { buffer.get_unchecked(Pos::new(2, 2)) }, 9);
    }

    #[test]
    fn rect_iter_unchecked() {
        #[rustfmt::skip]
        let buffer = VecGrid::with_buffer_row_major(3, 3, vec![
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ]).unwrap();

        assert_eq!(
            unsafe {
                buffer
                    .iter_rect_unchecked(Rect::from_ltwh(1, 1, 2, 1))
                    .collect::<Vec<_>>()
            },
            vec![&5, &6]
        );
        assert_eq!(
            unsafe {
                buffer
                    .iter_rect_unchecked(Rect::from_ltwh(0, 0, 3, 3))
                    .copied()
                    .collect::<Vec<_>>()
            },
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
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
