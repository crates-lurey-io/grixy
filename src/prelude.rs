//! Prelude for the grixy crate, re-exporting commonly used items.
//!
//! ```rust
//! use grixy::prelude::*;
//!
//! let mut grid = VecGrid::new_row_major(10, 10);
//! grid.set(Pos::new(5, 5), 42);
//!
//! assert_eq!(grid.get(Pos::new(5, 5)), Some(&42));
//! ```

#[cfg(feature = "alloc")]
pub use crate::buf::VecGrid;

#[cfg(feature = "buffer")]
pub use crate::buf::{ArrayGrid, GridBuf, SliceGrid, SliceMutGrid};

pub use crate::core::{HasSize as _, Pos, Rect, RowMajor, Size};
pub use crate::ops::{GridRead as _, GridWrite as _};
