//! Core types used in the Grixy crate.

use core::{error::Error, fmt::Display};

pub use ixy::HasSize;

/// A 2-dimensional position type.
///
/// This is a wrapper around [`ixy::Pos`] that uses `usize` for coordinates.
pub type Pos = ixy::Pos<usize>;

/// A 2-dimensional rectangle type.
///
/// This is a wrapper around [`ixy::Rect`] that uses `usize` for coordinates.
pub type Rect = ixy::Rect<usize>;

/// A 2-dimensional size type.
///
/// This is a wrapper around [`ixy::Size`] that uses `usize` for dimensions.
pub type Size = ixy::Size;

/// An error type for operations on or creating a `Grid`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum GridError {
    /// Could not access an element at a specified position due to it being out of bounds.
    OutOfBounds { pos: Pos },
}

impl Display for GridError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GridError::OutOfBounds { pos } => write!(f, "Position out of bounds: {pos}"),
        }
    }
}

impl Error for GridError {}
