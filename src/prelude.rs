//! Prelude for the grixy crate, re-exporting commonly used items.
//!
//! ```rust
//! use grixy::prelude::*;
//!
//! let mut grid = GridBuf::<u8, _, _>::new(5, 5);
//! grid.set(Pos::new(4, 4), 42);
//!
//! assert_eq!(grid.get(Pos::new(4, 4)), Some(&42));
//! ```

#[cfg(feature = "buffer")]
pub use crate::buf::GridBuf;
pub use crate::core::{HasSize as _, Pos, Rect, Size};
pub use crate::ops::{
    GridIter as _, GridRead as _, GridWrite as _,
    layout::{Block, ColumnMajor, Layout as _, Linear as _, RowMajor},
};
pub use crate::transform::GridConvertExt as _;
