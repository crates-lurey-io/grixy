//! Core types used in the Grixy crate.

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
pub type Size = ixy::Size<usize>;

/// An error type for operations on or creating a `Grid`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridError;
