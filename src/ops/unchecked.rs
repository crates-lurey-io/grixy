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

/// A grid that reports an accurate size using `size_hint()`.
///
/// ## Safety
///
/// `size_hint()` must return an upper bound that matches the exact size from [`ExactSizeGrid`].
/// If the dimensions are not accurate, safe code that relies on them (via the blanket impls in
/// [`GridReadUnchecked`] and [`GridWriteUnchecked`]) may call the unchecked methods with an
/// out-of-bounds position, which is _[undefined behavior][]_.
///
/// Specifically, implementors must satisfy:
///
/// - `size_hint().1 == Some(Size::new(self.width(), self.height()))`
/// - `trim_rect(rect)` must return a rect fully contained within the grid
///
/// Implementing `TrustedSizeGrid` for a type that violates these invariants nullifies the safety
/// guarantees of any type that uses `GridReadUnchecked + TrustedSizeGrid` or
/// `GridWriteUnchecked + TrustedSizeGrid` via the blanket impls of [`GridRead`]/[`GridWrite`].
///
/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
/// [`GridRead`]: crate::ops::GridRead
/// [`GridWrite`]: crate::ops::GridWrite
pub unsafe trait TrustedSizeGrid: ExactSizeGrid {}
