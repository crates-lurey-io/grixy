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

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
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
