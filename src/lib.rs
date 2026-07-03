//! Zero-cost 2D grids focused on memory consumption and performance.
//!
//! Grixy provides a set of traits and types for working with 2D grids, including traits for reading
//! and writing to grids, as well as implementations for common buffer types based on linear arrays
//! or vectors. The crate is `no_std` compatible, and operates without a dynamic memory allocator;
//! as a result _most_[^1] APIs are lazily evaluated, returning or operating on iterators or
//! references rather than copying data around.
//!
//! [^1]: The [`alloc`](#alloc) feature enables additional functionality based on `alloc`.
//!
//! Possible use-cases include:
//!
//! - 2D games, where grids can represent tile maps, collision detection, or game state
//! - Simulations, where grids can represent physical systems, cellular automata, or spatial data
//! - Pixel rasterization, where grids can represent images, textures, or graphical data
//! - Any other 2D grid-based data structure, such as matrices, graphs, or spatial indexing
//!
//! ## Examples
//!
//! ```rust
//! use grixy::{core::Pos, buf::GridBuf, ops::{GridRead, GridWrite}};
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! enum Tile {
//!    Empty,
//!    Wall,
//! }
//!
//! let mut grid = GridBuf::new_filled(10, 10, Tile::Empty);
//! grid.set(Pos::new(5, 5), Tile::Wall).unwrap();
//! assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Empty));
//! assert_eq!(grid.get(Pos::new(5, 5)), Some(&Tile::Wall));
//! assert_eq!(grid.get(Pos::new(11, 0)), None);
//! ```
//!
//! ## Features
//!
//! The default features are minimal, and useful mostly in library code that operates on grids.
//!
//! ### `alloc`
//!
//! Provides additional (but optional) functionality that uses `alloc`.
//!
//! ### `buffer`
//!
//! Provides the linear `GridBuf` type (and convenience types) through `grixy::buf`.
//!
//! If enabled in combination with `alloc`, `Vec`-based grids are available.
//!
//! ### `cell`
//!
//! Provides `GridWrite` when a mutable cell is wrapping a `GridWrite` type.
//!
//! ## Safety and unchecked operations
//!
//! Grixy provides unchecked variants ([`GridReadUnchecked`], [`GridWriteUnchecked`]) alongside
//! the checked [`GridRead`]/[`GridWrite`] traits, for performance-critical code that wants to
//! skip per-call bounds checks (see [`ops::unchecked`]).
//!
//! The unchecked traits are opt-in and unsafe to call directly: every method requires the caller
//! to prove the position (or every position in a rectangle) is in-bounds. What makes them usable
//! from *safe* code is [`TrustedSizeGrid`]: implementing this unsafe marker trait is a promise
//! that `size_hint()` reports the grid's true dimensions, and unlocks blanket impls that turn
//! `GridReadUnchecked + TrustedSizeGrid` into a safe [`GridRead`] (and likewise for writes) by
//! doing the bounds check once, in the blanket impl, before dispatching to the unchecked method.
//!
//! In short:
//!
//! 1. Implement `GridReadUnchecked`/`GridWriteUnchecked` with the actual (unchecked) access logic.
//! 2. Implement `unsafe impl TrustedSizeGrid for MyGrid {}` once you've verified `size_hint()` is
//!    accurate.
//! 3. `GridRead`/`GridWrite` are now implemented automatically, and bounds-checked for you.
//!
//! **Never implement `TrustedSizeGrid` for a type whose reported size can be wrong.** Doing so
//! nullifies the safety guarantees of every blanket impl built on top of it, and can lead to
//! reading or writing out of bounds, which is undefined behavior.
//!
//! ### Example: a custom grid with unchecked access
//!
//! ```rust
//! use grixy::{
//!     core::{Pos, Rect, Size},
//!     ops::{
//!         ExactSizeGrid, GridBase, GridRead as _,
//!         unchecked::{GridReadUnchecked, TrustedSizeGrid},
//!     },
//! };
//!
//! struct FixedGrid {
//!     cells: [u8; 9], // Always 3x3.
//! }
//!
//! impl GridBase for FixedGrid {
//!     fn size_hint(&self) -> (Size, Option<Size>) {
//!         let size = Size::new(3, 3);
//!         (size, Some(size))
//!     }
//! }
//!
//! impl ExactSizeGrid for FixedGrid {
//!     fn width(&self) -> usize { 3 }
//!     fn height(&self) -> usize { 3 }
//! }
//!
//! impl GridReadUnchecked for FixedGrid {
//!     type Element<'a> = u8;
//!     type Layout = grixy::ops::layout::RowMajor;
//!
//!     unsafe fn get_unchecked(&self, pos: Pos) -> u8 {
//!         // SAFETY: caller guarantees `pos` is within the 3x3 grid.
//!         unsafe { *self.cells.get_unchecked(pos.y * 3 + pos.x) }
//!     }
//! }
//!
//! // SAFETY: `size_hint()` always reports the true, fixed 3x3 size.
//! unsafe impl TrustedSizeGrid for FixedGrid {}
//!
//! // `GridRead` now comes for free, with bounds checking handled by the blanket impl.
//! let grid = FixedGrid { cells: [0, 1, 2, 3, 4, 5, 6, 7, 8] };
//! assert_eq!(grid.get(Pos::new(1, 1)), Some(4));
//! assert_eq!(grid.get(Pos::new(3, 3)), None); // Bounds-checked, not UB.
//! ```
//!
//! [`GridReadUnchecked`]: ops::unchecked::GridReadUnchecked
//! [`GridWriteUnchecked`]: ops::unchecked::GridWriteUnchecked
//! [`TrustedSizeGrid`]: ops::unchecked::TrustedSizeGrid
//! [`GridRead`]: ops::GridRead
//! [`GridWrite`]: ops::GridWrite

#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

pub(crate) mod internal;

#[cfg(feature = "buffer")]
pub mod buf;
pub mod core;
pub mod ops;
pub mod prelude;
pub mod transform;

#[cfg(test)]
pub mod test;
