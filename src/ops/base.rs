use crate::core::Size;

/// A base trait for grids that provides common functionality across both read and write operations.
pub trait GridBase {
    /// Returns the size of the grid, if known.
    ///
    /// Specifically, `size_hint()` returns a tuple where the first element is the minimum size,
    /// and the second element is the upper bound.
    ///
    /// The second half of the tuple is an [`Option<usize>`]. A `None` here means that either there
    /// is no known upper bound, or the upper bound is larger than [`usize`].
    ///
    /// ## Implementation
    ///
    /// It is not enforced that a grid contains the declared number of elements. A buggy grid may
    /// contain less than the lower bound, or more than the upper bound of elements.
    ///
    /// `size_hint()` is primarily intended to be used for optimizations such as reserving space
    /// when flattening a grid, or to eagerly trim a bounding rectangle to conform to the grid's
    /// size, but must not be trusted to e.g., omit bounds checks in unsafe code. An incorrect
    /// implementation of `size_hint()` should not lead to memory safety violations.
    ///
    /// The default implementation returns `(Size::new(0, 0), None)`, which is always valid.
    fn size_hint(&self) -> (Size, Option<Size>) {
        (Size::new(0, 0), None)
    }
}
