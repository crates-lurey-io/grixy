//! Prelude for the grixy crate, re-exporting commonly used items.
//!
//! ```rust
//! use grixy::prelude::*;
//!
//! let mut grid = GridBuf::<u8, _>::new(5, 5);
//! grid.set(Pos::new(4, 4), 42);
//!
//! assert_eq!(grid.get(Pos::new(4, 4)), Some(&42));
//! ```

#[cfg(feature = "buffer")]
pub use crate::buf::GridBuf;
pub use crate::convert::GridConvertExt as _;
pub use crate::core::{HasSize as _, Pos, Rect, RowMajor, Size};
pub use crate::ops::{GridIter as _, GridRead as _, GridWrite as _};
