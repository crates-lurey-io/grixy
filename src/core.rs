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
    OutOfBounds {
        /// The position that was out of bounds.
        pos: Pos,
    },
}

/// Converts a `u16`-based position to a `usize`-based position.
///
/// This conversion is always lossless on all supported platforms (`usize >= 16 bits`).
#[must_use]
pub fn pos_from_u16(p: ixy::Pos<u16>) -> Pos {
    Pos::new(usize::from(p.x), usize::from(p.y))
}

/// Tries to convert a `usize`-based position to a `u16`-based position.
///
/// Returns `None` if either coordinate exceeds `u16::MAX`.
#[must_use]
pub fn pos_to_u16(p: Pos) -> Option<ixy::Pos<u16>> {
    Some(ixy::Pos::new(u16::try_from(p.x).ok()?, u16::try_from(p.y).ok()?))
}

/// Lossless `Rect` conversion from `u16` to `usize` coordinates.
#[must_use]
pub fn rect_from_u16(r: ixy::Rect<u16>) -> Rect {
    Rect::from_ltwh(
        usize::from(r.left()),
        usize::from(r.top()),
        r.width_usize(),
        r.height_usize(),
    )
}

impl Display for GridError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GridError::OutOfBounds { pos } => write!(f, "Position out of bounds: {pos}"),
        }
    }
}

impl Error for GridError {}
