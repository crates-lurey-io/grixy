//! Zero-cost 2D grids for Rust.
//!
//! _Powerful enough for embedded systems, convenient enough for game development._
//!
//! ## Overview
//!
//! This crate provides traits and implementations for working with 2-dimensional grids that are
//! indexed by `usize`'d coordinates, i.e. for projects such as 2D games, simulations, pixel
//! rasterization, and more, with a focus on compatibility with embedded use-cases, performance, and
//! safety.
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
//! _Enabled by default._
//!
//! Provides additional (but optional) types that use `alloc::vec`.
//!
//! ### `buffer`
//!
//! _Enabled by default._
//!
//! Provides the linear `GridBuf` type (and convenience types) through `grixy::buf`.
//!
//! If you are just using traits and types, this feature can be safely disabled.

#![no_std]

pub(crate) mod internal;

#[cfg(feature = "buffer")]
pub mod buf;
pub mod convert;
pub mod core;
pub mod ops;
pub mod prelude;

#[cfg(test)]
pub mod test;
