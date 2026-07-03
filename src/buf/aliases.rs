//! Convenience type aliases for common [`GridBuf`](crate::buf::GridBuf) instantiations.
//!
//! `GridBuf<T, B, L>` is fully generic over its buffer and layout, which is powerful but verbose
//! for the common cases. These aliases spell out the usual combinations.

use crate::buf::GridBuf;
use crate::ops::layout::RowMajor;

/// A slice-backed, row-major grid borrowing an immutable `&[T]`.
///
/// Backed by `GridBuf<T, &[T], RowMajor>`; construct with
/// [`GridBuf::from_buffer`](crate::buf::GridBuf::from_buffer). Works without the `alloc` feature,
/// including over stack-allocated arrays.
///
/// ```rust
/// use grixy::{buf::SliceGrid, core::Pos, ops::GridRead as _};
///
/// let data = [1, 2, 3, 4, 5, 6];
/// let grid = SliceGrid::from_buffer(&data[..], 3);
/// assert_eq!(grid.get(Pos::new(0, 0)), Some(&1));
/// ```
pub type SliceGrid<'a, T> = GridBuf<T, &'a [T], RowMajor>;

/// A slice-backed, row-major grid borrowing a mutable `&mut [T]`.
///
/// Backed by `GridBuf<T, &mut [T], RowMajor>`; construct with
/// [`GridBuf::from_buffer`](crate::buf::GridBuf::from_buffer). Works without the `alloc` feature,
/// including over stack-allocated arrays.
///
/// ```rust
/// use grixy::{buf::SliceGridMut, core::Pos, ops::GridWrite as _};
///
/// let mut data = [0; 6];
/// let mut grid = SliceGridMut::from_buffer(&mut data[..], 3);
/// grid.set(Pos::new(0, 0), 42).unwrap();
/// ```
pub type SliceGridMut<'a, T> = GridBuf<T, &'a mut [T], RowMajor>;

#[cfg(feature = "alloc")]
mod alloc_aliases {
    extern crate alloc;

    use super::{GridBuf, RowMajor};
    use crate::ops::layout::ColumnMajor;
    use alloc::vec::Vec;

    /// A `Vec`-backed, row-major grid.
    ///
    /// Backed by `GridBuf<T, Vec<T>, RowMajor>`; construct with
    /// [`GridBuf::new`](crate::buf::GridBuf::new) or
    /// [`GridBuf::new_filled`](crate::buf::GridBuf::new_filled).
    ///
    /// ```rust
    /// use grixy::{buf::VecGrid, core::Pos, ops::GridRead as _};
    ///
    /// let grid: VecGrid<u8> = VecGrid::new_filled(3, 3, 42);
    /// assert_eq!(grid.get(Pos::new(1, 1)), Some(&42));
    /// ```
    pub type VecGrid<T> = GridBuf<T, Vec<T>, RowMajor>;

    /// A `Vec`-backed, column-major grid.
    ///
    /// Backed by `GridBuf<T, Vec<T>, ColumnMajor>`; construct with
    /// [`GridBuf::new_filled_with_layout`](crate::buf::GridBuf::new_filled_with_layout).
    pub type VecGridColMajor<T> = GridBuf<T, Vec<T>, ColumnMajor>;
}

#[cfg(feature = "alloc")]
pub use alloc_aliases::{VecGrid, VecGridColMajor};
