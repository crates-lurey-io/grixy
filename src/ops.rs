//! Grid operations and utilities for working with 2D grids.
//!
//! Implementing these traits allows for generic operations on grids.
//!
//! Some of these traits are automatically imported by the [`prelude`](crate::prelude).
//!
//! ## Examples
//!
//! Using [`GridRead`] to read from a grid:
//!
//! ```rust
//! use grixy::{core::Pos, buf::GridBuf, ops::GridRead};
//!
//! let grid = GridBuf::new_filled(10, 10, 42);
//! assert_eq!(grid.get(Pos::new(5, 5)), Some(&42));
//! ```
//!
//! Implementing [`GridWrite`] to write to a grid:
//!
//! ```rust
//! use grixy::prelude::*;
//!
//! struct MyGrid {
//!    grid: Vec<u8>,
//!    width: usize,
//! }
//!
//! impl GridBase for MyGrid {
//!   fn size_hint(&self) -> (Size, Option<Size>) {
//!     let size = Size::new(self.width, self.grid.len() / self.width);
//!     (size, Some(size))
//!   }
//! }
//!
//! impl GridWrite for MyGrid {
//!    type Element = u8;
//!    type Layout = RowMajor;
//!
//!    fn set(&mut self, pos: Pos, value: Self::Element) -> Result<(), GridError> {
//!        if pos.x >= self.width || pos.y >= self.grid.len() / self.width {
//!          return Err(GridError::OutOfBounds { pos });
//!        }
//!        let index = pos.y * self.width + pos.x;
//!        self.grid[index] = value;
//!        Ok(())
//!    }
//! }
//!
//! let mut my_grid = MyGrid {
//!   grid: vec![0; 100],
//!   width: 10,
//! };
//!
//! my_grid.set(Pos::new(5, 5), 42);
//! assert_eq!(my_grid.grid[55], 42);
//! ```

pub mod layout;
pub mod unchecked;

#[cfg(feature = "alloc")]
mod alloc;

#[cfg(feature = "cell")]
mod cell;

mod base;
mod draw;
mod read;
mod write;

pub use base::{ExactSizeGrid, GridBase};
pub use draw::copy_rect;
pub use read::{GridIter, GridRead};
pub use write::GridWrite;
