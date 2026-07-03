use ixy::HasSize;

use crate::{
    core::{GridError, Pos, Rect},
    ops::{
        ExactSizeGrid, GridBase,
        layout::{self, Layout as _},
    },
};

/// Write elements to a 2-dimensional grid position.
///
/// # Naming convention
///
/// Method names follow `std::slice`'s `fill`/`fill_with` convention: [`fill`](GridWrite::fill)
/// and [`fill_rect`](GridWrite::fill_rect) take a single `Copy` value, while the `_with` suffix
/// (e.g. [`fill_with`](GridWrite::fill_with)) takes a position-driven closure, and
/// `_from_iter` (e.g. [`fill_from_iter`](GridWrite::fill_from_iter)) consumes an iterator.
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

    /// Sets the element at a specified position.
    ///
    /// ## Errors
    ///
    /// Returns an error if the position is out of bounds.
    fn set(&mut self, pos: Pos, value: Self::Element) -> Result<(), GridError>;

    /// Clears the grid, setting all elements to their default value.
    ///
    /// Elements are set in an order agreeable to the grid's internal layout.
    fn clear(&mut self)
    where
        Self::Element: Default,
        Self: ExactSizeGrid,
    {
        self.clear_rect(self.size().to_rect());
    }

    /// Sets every element in the grid to `value`.
    ///
    /// Elements are set in an order agreeable to the grid's internal layout.
    fn fill(&mut self, value: Self::Element)
    where
        Self::Element: Copy,
        Self: ExactSizeGrid,
    {
        self.fill_rect(self.size().to_rect(), value);
    }

    /// Sets every element in the grid using a position-driven closure.
    ///
    /// Elements are set in an order agreeable to the grid's internal layout.
    fn fill_with(&mut self, f: impl FnMut(Pos) -> Self::Element)
    where
        Self: ExactSizeGrid,
    {
        self.fill_rect_with(self.size().to_rect(), f);
    }

    /// Sets elements within the grid from an iterator.
    ///
    /// Elements are set in an order agreeable to the grid's internal layout.
    fn fill_from_iter(&mut self, iter: impl Iterator<Item = Self::Element>)
    where
        Self: ExactSizeGrid,
    {
        self.fill_rect_from_iter(self.size().to_rect(), iter);
    }

    /// Clears a rectangular region of the grid, setting all elements to their default value.
    ///
    /// Elements are set in an order agreeable to the grid's internal layout. Out-of-bounds
    /// elements are skipped, and the bounding rectangle is treated as _exclusive_ of the right
    /// and bottom edges.
    ///
    /// ## Performance
    ///
    /// The default implementation uses [`Layout::iter_pos`] to iterate over the rectangle,
    /// involving bounds checking for each element. Other implementations may optimize this, for
    /// example by using a more efficient iteration strategy (for linear reads, reduced bounds
    /// checking, etc.).
    ///
    /// [`Layout::iter_pos`]: layout::Layout::iter_pos
    fn clear_rect(&mut self, bounds: Rect)
    where
        Self::Element: Default,
    {
        self.fill_rect_with(bounds, |_| Default::default());
    }

    /// Sets every element within a rectangular region of the grid to `value`.
    ///
    /// Elements are set in an order agreeable to the grid's internal layout. Out-of-bounds
    /// elements are skipped, and the bounding rectangle is treated as _exclusive_ of the right and
    /// bottom edges.
    ///
    /// ## Performance
    ///
    /// The default implementation delegates to [`Self::fill_rect_with`], wrapping the value in a
    /// closure. Specialized implementations may use `memset`-style operations for `Copy` types.
    fn fill_rect(&mut self, dst: Rect, value: Self::Element)
    where
        Self::Element: Copy,
    {
        self.fill_rect_with(dst, |_| value);
    }

    /// Sets elements within a rectangular region of the grid using a position-driven closure.
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
    ///
    /// [`Layout::iter_pos`]: layout::Layout::iter_pos
    fn fill_rect_with(&mut self, bounds: Rect, mut f: impl FnMut(Pos) -> Self::Element) {
        Self::Layout::iter_pos(self.trim_rect(bounds)).for_each(|pos| {
            let _ = self.set(pos, f(pos));
        });
    }

    /// Sets elements within a rectangular region of the grid from an iterator.
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
    ///
    /// [`Layout::iter_pos`]: layout::Layout::iter_pos
    fn fill_rect_from_iter(&mut self, dst: Rect, iter: impl IntoIterator<Item = Self::Element>) {
        Self::Layout::iter_pos(self.trim_rect(dst))
            .zip(iter)
            .for_each(|(pos, value)| {
                let _ = self.set(pos, value);
            });
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::{core::Size, ops::layout::RowMajor};

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
                Err(GridError::OutOfBounds { pos })
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
    fn impl_checked_fill_rect_with() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 3, 3).unwrap();
        grid.fill_rect_with(bounds, |_| 42);
        assert_eq!(grid.grid, [[42; 3]; 3]);
    }

    #[test]
    fn impl_checked_fill_rect_from_iter() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 3, 3).unwrap();
        grid.fill_rect_from_iter(bounds, vec![42; 9]);
        assert_eq!(grid.grid, [[42; 3]; 3]);
    }

    #[test]
    fn impl_checked_fill_rect() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 3, 3).unwrap();
        grid.fill_rect(bounds, 42);
        assert_eq!(grid.grid, [[42; 3]; 3]);
    }
}
