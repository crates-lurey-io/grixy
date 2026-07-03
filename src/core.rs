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
///
/// Marked `#[non_exhaustive]` even though it currently has a single variant: grid operations are
/// expected to grow additional failure modes over time (for example, buffer/layout mismatches on
/// construction), and this lets those be added without a breaking change to downstream `match`
/// expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum GridError {
    /// Could not access an element at a specified position due to it being out of bounds.
    OutOfBounds {
        /// The position that was out of bounds.
        pos: Pos,
    },
}

impl Display for GridError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GridError::OutOfBounds { pos } => write!(f, "Position out of bounds: {pos}"),
        }
    }
}

impl Error for GridError {}
