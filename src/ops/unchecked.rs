//! Unchecked operations for grids.
//!
//! These traits and operations allow for unchecked access to grid elements, bypassing safety
//! checks. They are intended for use in performance-critical code where the caller guarantees that
//! the operations are safe.

mod read;
mod write;

pub use read::GridReadUnchecked;
pub use write::GridWriteUnchecked;

use crate::ops::ExactSizeGrid;

/// A grid that reports an accuate size using `size_hint()`.
///
/// ## Safety
///
/// If the dimensions provide are not accurate, it may lead to _[undefined behavior][]_.
///
/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
pub unsafe trait TrustedSizeGrid: ExactSizeGrid {}
