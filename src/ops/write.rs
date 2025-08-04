use crate::{
    core::{GridError, Pos, Rect},
    ops::layout,
};

/// Write elements to a 2-dimensional grid position.
pub trait GridWrite {
    /// The type of elements in the grid.
    type Element;

    /// The type of layout used for the grid.
    type Layout: layout::Layout;

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
    fn fill_rect(&mut self, _bounds: Rect, mut _f: impl FnMut(Pos) -> Self::Element) {
        todo!()
        // Self::Layout::iter_pos(bounds).for_each(|pos| {
        //     let _ = self.set(pos, f(pos));
        // });
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
    fn fill_rect_iter(&mut self, _dst: Rect, _iter: impl IntoIterator<Item = Self::Element>) {
        todo!()
        // Self::Layout::iter_pos(dst)
        //     .zip(iter)
        //     .for_each(|(pos, value)| {
        //         let _ = self.set(pos, value);
        //     });
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
