//! 2-dimensional grids with `usize`'d coordinate positions.
//!
//! This crate provides traits and implementations for working with 2-dimensional grids that are
//! indexed by `usize`'d coordinates, i.e. for projects such as 2D games, simulations, pixel
//! rasterization, and more, with a focus on performance and safety.
//!
//! ## Examples
//!
//! ```rust
//! use grixy::{core::Pos, buf::VecGrid, ops::{GridRead, GridWrite}};
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! enum Tile {
//!    Empty,
//!    Wall,
//! }
//!
//! let mut grid = VecGrid::new_filled_row_major(10, 10, Tile::Empty);
//! grid.set(Pos::new(5, 5), Tile::Wall).unwrap();
//! assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Empty));
//! assert_eq!(grid.get(Pos::new(5, 5)), Some(&Tile::Wall));
//! assert_eq!(grid.get(Pos::new(11, 0)), None);
//! ```
//!
//! ## Features
//!
//! ### `alloc`
//!
//! Provides additional (but optional) types that use `alloc::vec`.
//!
//! ### `bytemuck`
//!
//! Provides support for using `bytemuck` to eligible `GridBuffer` instances to slices of bytes

#![no_std]

pub(crate) mod internal;

pub mod buf;
pub mod core;
pub mod ops;
pub mod prelude;

#[cfg(test)]
pub mod test;
