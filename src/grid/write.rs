use ixy::index::Layout;

use crate::{
    core::{GridError, Pos, Rect},
    grid::{BoundedGrid, GridBase},
};

/// Write elements to a 2-dimensional grid position.
pub trait GridWrite: GridBase {
    /// Sets the element at a specified position.
    ///
    /// ## Errors
    ///
    /// Returns an error if the position is out of bounds.
    fn set(&mut self, pos: Pos, value: Self::Element) -> Result<(), GridError>;

    /// Sets a subset of elements in a row of the grid from an iterator.
    ///
    /// Positions out of bound are skipped.
    ///
    /// ## Implementation
    ///
    /// The default implementation invokes `set` for each position in the specified range.
    fn set_row(&mut self, start: Pos, with: impl IntoIterator<Item = Self::Element>) {
        let y = start.y;
        let mut x = start.x;
        for value in with {
            let pos = Pos::new(x, y);
            let res = self.set(pos, value);
            if res.is_err() {
                break;
            }
            x += 1;
        }
    }

    /// Sets a subset of elements in a column of the grid from an iterator.
    ///
    /// Positions out of bound are skipped.
    ///
    /// ## Implementation
    ///
    /// The default implementation invokes `set` for each position in the specified range.
    fn set_col(&mut self, start: Pos, with: impl IntoIterator<Item = Self::Element>) {
        let x = start.x;
        let mut y = start.y;
        for value in with {
            let pos = Pos::new(x, y);
            let res = self.set(pos, value);
            if res.is_err() {
                break;
            }
            y += 1;
        }
    }

    /// Sets a rectangular region of the grid from an iterator.
    ///
    /// Positions out of bound are skipped.
    ///
    /// ## Implementation
    ///
    /// The default implementation invokes `set` for each position in the specified rectangle.
    fn set_rect<L: Layout>(&mut self, bounds: Rect, with: impl IntoIterator<Item = Self::Element>) {
        L::iter_pos(bounds).zip(with).for_each(|(pos, value)| {
            let _ = self.set(pos, value);
        });
    }
}

/// Write elements to a 2-dimensional grid position without bounds checking.
pub trait GridWriteUnchecked: GridBase {
    /// Sets the element at a specified position without bounds checking.
    ///
    /// ## Safety
    ///
    /// Calling this method with an out-of-bounds position is _[undefined behavior][]_.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    unsafe fn set_unchecked(&mut self, pos: Pos, value: Self::Element);

    /// Sets a subset of elements in a row of the grid from an iterator without bounds checking.
    ///
    /// ## Safety
    ///
    /// Calling this method with out-of-bounds positions is undefined behavior.
    unsafe fn set_row_unchecked(
        &mut self,
        start: Pos,
        with: impl IntoIterator<Item = Self::Element>,
    ) {
        let y = start.y;
        let mut x = start.x;
        for value in with {
            let pos = Pos::new(x, y);
            unsafe { self.set_unchecked(pos, value) };
            x += 1;
        }
    }

    /// Sets a subset of elements in a column of the grid from an iterator without bounds checking.
    ///
    /// ## Safety
    ///
    /// Calling this method with out-of-bounds positions is undefined behavior.
    unsafe fn set_col_unchecked(
        &mut self,
        start: Pos,
        with: impl IntoIterator<Item = Self::Element>,
    ) {
        let x = start.x;
        let mut y = start.y;
        for value in with {
            let pos = Pos::new(x, y);
            unsafe { self.set_unchecked(pos, value) };
            y += 1;
        }
    }

    /// Sets a rectangular region of the grid from an iterator without bounds checking.
    ///
    /// ## Safety
    ///
    /// Calling this method with out-of-bounds positions is undefined behavior.
    unsafe fn set_rect_unchecked<L: Layout>(
        &mut self,
        bounds: Rect,
        with: impl IntoIterator<Item = Self::Element>,
    ) {
        L::iter_pos(bounds).zip(with).for_each(|(pos, value)| {
            unsafe { self.set_unchecked(pos, value) };
        });
    }
}

/// Automatically implement `GridWrite` when `GridWriteUnchecked` + `BoundedGrid` are implemented.
impl<T: GridWriteUnchecked + BoundedGrid> GridWrite for T {
    fn set(&mut self, pos: Pos, value: Self::Element) -> Result<(), GridError> {
        if self.contains_pos(pos) {
            unsafe {
                self.set_unchecked(pos, value);
                Ok(())
            }
        } else {
            Err(GridError)
        }
    }

    // TODO: Optimize set_row, set_col, and set_rect to use unchecked methods if safe.
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;
    use alloc::vec;
    use ixy::index::RowMajor;

    struct TestGrid {
        grid: [[u8; 3]; 3],
    }

    impl GridBase for TestGrid {
        type Element = u8;
    }

    unsafe impl BoundedGrid for TestGrid {
        fn width(&self) -> usize {
            3
        }

        fn height(&self) -> usize {
            3
        }
    }

    impl GridWriteUnchecked for TestGrid {
        unsafe fn set_unchecked(&mut self, pos: Pos, value: Self::Element) {
            self.grid[pos.y][pos.x] = value;
        }
    }

    #[test]
    fn test_set_ok() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 1, y: 1 };
        grid.set(pos, 42).unwrap();
        assert_eq!(grid.grid[1][1], 42);
    }

    #[test]
    fn test_set_out_of_bounds_x() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 3, y: 1 };
        assert!(grid.set(pos, 42).is_err());
    }

    #[test]
    fn test_set_out_of_bounds_y() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 1, y: 3 };
        assert!(grid.set(pos, 42).is_err());
    }

    #[test]
    fn test_set_row() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let start = Pos { x: 0, y: 1 };
        grid.set_row(start, vec![1, 2, 3]);
        assert_eq!(grid.grid[1], [1, 2, 3]);
    }

    #[test]
    fn test_set_col() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let start = Pos { x: 1, y: 0 };
        grid.set_col(start, vec![4, 5, 6]);
        assert_eq!(grid.grid[0][1], 4);
    }

    #[test]
    fn test_set_rect() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 3, 3).unwrap();
        grid.set_rect::<RowMajor>(bounds, vec![7, 8, 9]);
        assert_eq!(grid.grid, [[7, 8, 9], [0, 0, 0], [0, 0, 0]]);
    }

    #[test]
    fn test_set_unchecked_in_bounds() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 2, y: 2 };
        unsafe {
            grid.set_unchecked(pos, 99);
        }
        assert_eq!(grid.grid[2][2], 99);
    }

    #[test]
    fn test_set_row_unchecked_in_bounds() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let start = Pos { x: 0, y: 0 };
        unsafe {
            grid.set_row_unchecked(start, vec![10, 20, 30]);
        }
        assert_eq!(grid.grid[0], [10, 20, 30]);
    }

    #[test]
    fn test_set_col_unchecked_in_bounds() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let start = Pos { x: 2, y: 0 };
        unsafe {
            grid.set_col_unchecked(start, vec![11, 22, 33]);
        }
        assert_eq!(grid.grid[0][2], 11);
        assert_eq!(grid.grid[1][2], 22);
        assert_eq!(grid.grid[2][2], 33);
    }

    #[test]
    fn test_set_rect_unchecked_in_bounds() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 3, 1).unwrap();
        unsafe {
            grid.set_rect_unchecked::<RowMajor>(bounds, vec![5, 6, 7]);
        }
        assert_eq!(grid.grid[0], [5, 6, 7]);
    }
}
