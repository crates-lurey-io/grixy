//! 2-dimensional grids.

#![no_std]

pub mod buf;
pub use buf::GridBuf;

mod error;
pub use error::GridError;

/// A 2-dimensional position type.
///
/// This is a wrapper around [`ixy::Pos`] that uses `usize` for coordinates.
pub type Pos = ixy::Pos<usize>;

/// A 2-dimensional rectangle type.
///
/// This is a wrapper around [`ixy::Rect`] that uses `usize` for coordinates.
pub type Rect = ixy::Rect<usize>;
