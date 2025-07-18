use ixy::index::Layout;

use crate::{
    core::{Pos, Rect},
    grid::{BoundedGrid, GridBase},
};

/// Read elements from a 2-dimensional grid position.
pub trait GridRead: GridBase {
    /// Returns a reference to an element at a specified position.
    ///
    /// If the position is out of bounds, it returns `None`.
    fn get(&self, pos: Pos) -> Option<&Self::Element>;

    /// Returns an iterator over a subset of elements in a row of the grid.
    ///
    /// Positions out of bound are skipped.
    ///
    /// ## Implementation
    ///
    /// The default implementation invokes `get` for each position in the specified range.
    fn row_iter(&self, start: Pos, length: usize) -> impl Iterator<Item = &Self::Element> + '_ {
        (0..length).filter_map(move |i| {
            let pos = Pos::new(start.x + i, start.y);
            self.get(pos)
        })
    }

    /// Returns an iterator over a subset of elements in a column of the grid.
    ///
    /// Positions out of bound are skipped.
    ///
    /// ## Implementation
    ///
    /// The default implementation invokes `get` for each position in the specified range.
    fn col_iter(&self, start: Pos, length: usize) -> impl Iterator<Item = &Self::Element> + '_ {
        (0..length).filter_map(move |i| {
            let pos = Pos::new(start.x, start.y + i);
            self.get(pos)
        })
    }

    /// Returns an iterator over each element in a rectangular region of the grid.
    ///
    /// Positions out of bound are skipped.
    ///
    /// ## Implementation
    ///
    /// The default implementation invokes `get` for each position in the specified rectangle.
    fn rect_iter<L: Layout>(&self, bounds: Rect) -> impl Iterator<Item = &Self::Element> {
        L::iter_pos(bounds).filter_map(move |pos| self.get(pos))
    }
}

/// Read elements from a 2-dimensional grid position without bounds checking.
pub trait GridReadUnchecked: GridBase {
    /// Returns a reference to an element, without doing bounds checking.
    ///
    /// ## Safety
    ///
    /// Calling this method with an out-of-bounds position is _[undefined behavior][]_.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    unsafe fn get_unchecked(&self, pos: Pos) -> &Self::Element;

    /// Returns a raw iterator over a subset of elements in a row of the grid, without bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure all positions are in bounds.
    unsafe fn row_iter_unchecked(
        &self,
        start: Pos,
        length: usize,
    ) -> impl Iterator<Item = &Self::Element> + '_ {
        (0..length).map(move |i| {
            let pos = Pos::new(start.x + i, start.y);
            unsafe { self.get_unchecked(pos) }
        })
    }

    /// Returns a raw iterator over a subset of elements in a column of the grid, without bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure all positions are in bounds.
    unsafe fn col_iter_unchecked(
        &self,
        start: Pos,
        length: usize,
    ) -> impl Iterator<Item = &Self::Element> + '_ {
        (0..length).map(move |i| {
            let pos = Pos::new(start.x, start.y + i);
            unsafe { self.get_unchecked(pos) }
        })
    }

    /// Returns a raw iterator over each element in a rectangular region of the grid, without bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure all positions in the rectangle are in bounds.
    unsafe fn rect_iter_unchecked<L: Layout>(
        &self,
        bounds: Rect,
    ) -> impl Iterator<Item = &Self::Element> {
        L::iter_pos(bounds).map(move |pos| unsafe { self.get_unchecked(pos) })
    }
}

/// Automatically implement `GridRead` when `GridReadUnchecked` + `BoundedGrid` are implemented.
impl<T: GridReadUnchecked + BoundedGrid> GridRead for T {
    fn get(&self, pos: Pos) -> Option<&Self::Element> {
        if self.contains_pos(pos) {
            Some(unsafe { self.get_unchecked(pos) })
        } else {
            None
        }
    }

    fn row_iter(&self, start: Pos, length: usize) -> impl Iterator<Item = &Self::Element> + '_ {
        let start = start.clamp(
            Pos::ORIGIN,
            Pos::new(
                self.width().saturating_sub(1),
                self.height().saturating_sub(1),
            ),
        );
        let length = length.min(self.width() - start.x);
        unsafe { self.row_iter_unchecked(start, length) }
    }

    fn col_iter(&self, start: Pos, length: usize) -> impl Iterator<Item = &Self::Element> + '_ {
        let start = start.clamp(
            Pos::ORIGIN,
            Pos::new(
                self.width().saturating_sub(1),
                self.height().saturating_sub(1),
            ),
        );
        let length = length.min(self.height() - start.y);
        unsafe { self.col_iter_unchecked(start, length) }
    }

    fn rect_iter<L: Layout>(&self, bounds: Rect) -> impl Iterator<Item = &Self::Element> {
        // TODO: Add Size.to_rect()
        let size = unsafe { Rect::from_ltrb_unchecked(0, 0, self.width(), self.height()) };
        let bounds = bounds.intersect(size);
        L::iter_pos(bounds).map(move |pos| unsafe { self.get_unchecked(pos) })
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;
    use alloc::{vec, vec::Vec};
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

    impl GridReadUnchecked for TestGrid {
        unsafe fn get_unchecked(&self, pos: Pos) -> &Self::Element {
            &self.grid[pos.y][pos.x]
        }
    }

    #[test]
    fn test_get_ok() {
        let grid = TestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        assert_eq!(grid.get(Pos::new(1, 1)), Some(&5));
    }

    #[test]
    fn test_get_out_of_bounds_x() {
        let grid = TestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        assert_eq!(grid.get(Pos::new(3, 1)), None);
    }

    #[test]
    fn test_get_out_of_bounds_y() {
        let grid = TestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        assert_eq!(grid.get(Pos::new(1, 3)), None);
    }

    #[test]
    fn test_row_iter() {
        let grid = TestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let iter: Vec<_> = grid.row_iter(Pos::new(0, 1), 3).collect();
        assert_eq!(iter, vec![&4, &5, &6]);
    }

    #[test]
    fn test_col_iter() {
        let grid = TestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let iter: Vec<_> = grid.col_iter(Pos::new(1, 0), 3).collect();
        assert_eq!(iter, vec![&2, &5, &8]);
    }

    #[test]
    fn test_rect_iter() {
        let grid = TestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let bounds = Rect::from_ltrb(0, 0, 2, 2).unwrap();
        let iter: Vec<_> = grid.rect_iter::<RowMajor>(bounds).collect();
        assert_eq!(iter, vec![&1, &2, &4, &5]);
    }

    #[test]
    fn test_get_unchecked_ok() {
        let grid = TestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let val = unsafe { grid.get_unchecked(Pos::new(2, 2)) };
        assert_eq!(val, &9);
    }

    #[test]
    fn test_row_iter_unchecked() {
        let grid = TestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let iter: Vec<_> = unsafe { grid.row_iter_unchecked(Pos::new(0, 0), 3).collect() };
        assert_eq!(iter, vec![&1, &2, &3]);
    }

    #[test]
    fn test_col_iter_unchecked() {
        let grid = TestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let iter: Vec<_> = unsafe { grid.col_iter_unchecked(Pos::new(2, 0), 3).collect() };
        assert_eq!(iter, vec![&3, &6, &9]);
    }

    #[test]
    fn test_rect_iter_unchecked() {
        let grid = TestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let bounds = Rect::from_ltrb(1, 1, 3, 3).unwrap();
        let iter: Vec<_> = unsafe { grid.rect_iter_unchecked::<RowMajor>(bounds).collect() };
        assert_eq!(iter, vec![&5, &6, &8, &9]);
    }
}
