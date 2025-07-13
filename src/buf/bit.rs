//! Provides a single-bit, 2D grid data structure backed by a linear buffer.
//!
//! The main type, [`BitGrid`], is highly generic, allowing it to act as a view over any buffer
//! that can be treated as a slice.
//!
//! ## Convenience Types
//!
//! For ease of use, several type aliases are provided for common use cases:
//! - [`VecBitGrid`]: An owned grid backed by a `Vec<u8>`
//! - [`ArrayBitGrid`]: An owned grid backed by a fixed-size array, `[u8; N]`.
//! - [`SliceBitGrid`]: A read-only, borrowed view over an existing slice.
//! - [`SliceMutBitGrid`]: A mutable, borrowed view over an existing slice.
//!
//! # Examples
//!
//! Creating an owned `VecBitGrid` and accessing an element:
//! ```
//! use grixy::{Pos, buf::bit::VecBitGrid};
//!
//! let grid = VecBitGrid::new(10, 5);
//! assert_eq!(grid.get(Pos::new(3, 4)), Some(false));
//! ```

use core::marker::PhantomData;
use ixy::index::{Layout, RowMajor};

/// An unsigned integer type that can be used as a bit buffer.
pub trait HasBits {}

/// A 2-dimensional grid where every individual bit is treated as either `true` or `false`.
///
/// ## Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
#[derive(Debug, Clone)]
pub struct BitGrid<B, L = RowMajor>
where
    B: AsRef<[u8]>,
    L: Layout,
{
    buffer: B,
    width: usize,
    height: usize,
    _layout: PhantomData<L>,
}
