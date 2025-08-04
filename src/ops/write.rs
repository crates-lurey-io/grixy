use crate::{
    core::{GridError, Pos, Rect, Size},
    ops::{
        GridBase,
        layout::{self, Layout as _},
    },
};

/// Write elements to a 2-dimensional grid position.
pub trait GridWrite: GridBase {
    /// The type of elements in the grid.
    type Element;

    /// The type of layout used for the grid.
    ///
    /// ## Implementation
    ///
    /// It is not guaranteed that the internal storage of the grid matches this layout, but methods
    /// that return iterators over the grid's elements or positions should return them in the
    /// traversal order defined by this layout.
    ///
    /// [`RowMajor`][layout::RowMajor] is a reasonable default implementation for most grids.
    type Layout: layout::Layout;

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

    /// Sets the element at a specified position.
    ///
    /// ## Errors
    ///
    /// Returns an error if the position is out of bounds.
    fn set(&mut self, pos: Pos, value: Self::Element) -> Result<(), GridError>;

    /// Sets elements within a rectangular region of the grid.
    ///
    /// Elements are set in an order agreeable to the grid's internal layout. Out-of-bounds
    /// elements are skipped, and the bounding rectangle is treated as _exclusive_ of the right and
    /// bottom edges.
    ///
    /// ## Performance
    ///
    /// The default implementation uses [`Layout::iter_pos`] to iterate over the rectangle,
    /// involving bounds checking for each element. Other implementations may optimize this, for
    /// example by using a more efficient iteration strategy (for linear reads, reduced bounds
    /// checking, etc.).
    fn fill_rect(&mut self, bounds: Rect, mut f: impl FnMut(Pos) -> Self::Element) {
        Self::Layout::iter_pos(bounds).for_each(|pos| {
            let _ = self.set(pos, f(pos));
        });
    }

    /// Sets elements within a rectangular region of the grid.
    ///
    /// Elements are set in an order agreeable to the grid's internal layout. Out-of-bounds
    /// elements are skipped, and the bounding rectangle is treated as _exclusive_ of the right and
    /// bottom edges.
    ///
    /// If the provided iterator has fewer elements than the rectangle, the remaining elements will
    /// not be set.
    ///
    /// ## Performance
    ///
    /// The default implementation uses [`Layout::iter_pos`] to iterate over the rectangle,
    /// involving bounds checking for each element. Other implementations may optimize this, for
    /// example by using a more efficient iteration strategy (for linear reads, reduced bounds
    /// checking, etc.).
    fn fill_rect_iter(&mut self, dst: Rect, iter: impl IntoIterator<Item = Self::Element>) {
        Self::Layout::iter_pos(dst)
            .zip(iter)
            .for_each(|(pos, value)| {
                let _ = self.set(pos, value);
            });
    }

    /// Sets elements within a rectangular region of the grid.
    ///
    /// Elements are set in an order agreeable to the grid's internal layout. Out-of-bounds
    /// elements are skipped, and the bounding rectangle is treated as _exclusive_ of the right and
    /// bottom edges.
    ///
    /// ## Performance
    ///
    /// The default implementation uses [`Layout::iter_pos`] to iterate over the rectangle,
    /// involving bounds checking for each element. Other implementations may optimize this, for
    /// example by using a more efficient iteration strategy (for linear reads, reduced bounds
    /// checking, etc.).
    fn fill_rect_solid(&mut self, dst: Rect, value: Self::Element)
    where
        Self::Element: Copy,
    {
        self.fill_rect(dst, |_| value);
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::ops::layout::RowMajor;

    use super::*;
    use alloc::vec;

    struct TestGrid {
        grid: [[u8; 3]; 3],
    }

    impl GridBase for TestGrid {
        fn size_hint(&self) -> (Size, Option<Size>) {
            let size = Size::new(3, 3);
            (size, Some(size))
        }
    }

    impl GridWrite for TestGrid {
        type Element = u8;

        type Layout = RowMajor;

        fn set(&mut self, pos: Pos, value: Self::Element) -> Result<(), GridError> {
            if pos.x < 3 && pos.y < 3 {
                self.grid[pos.y][pos.x] = value;
                Ok(())
            } else {
                Err(GridError)
            }
        }
    }

    #[test]
    fn impl_checked_set_ok() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 1, y: 1 };
        grid.set(pos, 42).unwrap();
        assert_eq!(grid.grid[1][1], 42);
    }

    #[test]
    fn impl_checked_set_out_of_bounds_x() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 3, y: 1 };
        grid.set(pos, 42).unwrap_err();
        assert_eq!(grid.grid[1][1], 0);
    }

    #[test]
    fn impl_checked_set_out_of_bounds_y() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 1, y: 3 };
        grid.set(pos, 42).unwrap_err();
        assert_eq!(grid.grid[1][1], 0);
    }

    #[test]
    fn impl_checked_fill_rect() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 3, 3).unwrap();
        grid.fill_rect(bounds, |_| 42);
        assert_eq!(grid.grid, [[42; 3]; 3]);
    }

    #[test]
    fn impl_checked_fill_rect_iter() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 3, 3).unwrap();
        grid.fill_rect_iter(bounds, vec![42; 9]);
        assert_eq!(grid.grid, [[42; 3]; 3]);
    }

    #[test]
    fn impl_checked_fill_rect_solid() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 3, 3).unwrap();
        grid.fill_rect_solid(bounds, 42);
        assert_eq!(grid.grid, [[42; 3]; 3]);
    }
}
