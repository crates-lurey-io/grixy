use crate::core::{Pos, Rect, Size};

/// A base trait for grids.
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

    /// Returns the given rectangle, trimmed to fit within the grid's size.
    ///
    /// By default, uses the upper bound of `size_hint()` to determine the grid's size.
    ///
    /// If the upper bound is `None`, it returns the rectangle unchanged.
    ///
    /// ## Implementation
    ///
    /// It is not enforced that the returned rectangle is valid for the grid; `trim_rect()` is
    /// primarily intended for use in optimizations, such as trimming a bounding rectangle before
    /// performing operations on the grid, but must not be trusted to e.g., omit bounds checks in
    /// unsafe code.
    ///
    /// The default implementation intersects the rectangle with the upper bound of `size_hint()`.
    fn trim_rect(&self, rect: Rect) -> Rect {
        let (_, max) = self.size_hint();
        if let Some(max) = max {
            rect.intersect(Rect::new(Pos::ORIGIN, max))
        } else {
            rect
        }
    }
}

/// A trait for grids that have a known (exact) size.
pub trait ExactSizeGrid {
    /// Returns the width of the grid, in columns.
    ///
    /// ## Implementation
    ///
    /// It is not enforced that the grid contains exactly the number of elements specified by the
    /// width. A buggy grid may contain fewer or more elements than the width indicates and should
    /// not lead to memory safety violations.
    fn width(&self) -> usize;

    /// Returns the height of the grid, in rows.
    ///
    /// ## Implementation
    ///
    /// It is not enforced that the grid contains exactly the number of elements specified by the
    /// height. A buggy grid may contain fewer or more elements than the height indicates and should
    /// not lead to memory safety violations.
    fn height(&self) -> usize;

    /// Returns the exact size of the grid.
    ///
    /// This method should return a `Size` that represents the exact number of elements in the grid.
    ///
    /// ## Implementation
    ///
    /// It is not enforced that the grid contains exactly the number of elements specified by the
    /// size. A buggy grid may contain fewer or more elements than the size indicates and should not
    /// lead to memory safety violations.
    fn size(&self) -> Size {
        Size {
            width: self.width(),
            height: self.height(),
        }
    }

    /// Returns whether the given position is valid for this grid.
    fn contains(&self, pos: Pos) -> bool {
        pos.x < self.width() && pos.y < self.height()
    }
}
