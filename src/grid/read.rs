use ixy::index::{Layout, RowMajor};

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

    /// Returns an iterator over elements in a rectangular region of the grid.
    ///
    /// Elements are returned in an order agreeable to the grid's internal layout, which defaults to
    /// [`RowMajor`], but can be overridden. Out-of-bounds elements are skipped, and the bounding
    /// rectangle is treated as _exclusive_ of the right and bottom edges.
    ///
    /// ## Performance
    ///
    /// The default implementation uses [`ops::get_rect`] to iterate over the rectangle, involving
    /// bounds checking for each element. Other implementations may optimize this, for example by
    /// using a more efficient iteration strategy (for linear reads, reduced bounds checking, etc.).
    fn rect_iter(&self, bounds: Rect) -> impl Iterator<Item = &Self::Element> {
        RowMajor::iter_pos(bounds).filter_map(|pos| self.get(pos))
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

    /// Returns an iterator over elements in a rectangular region of the grid.
    ///
    ///Elements are returned in an order agreeable to the grid's internal layout, which defaults to
    /// [`RowMajor`], but can be overridden. The bounding rectangle is treated as _exclusive_ of the
    /// right and bottom edges.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that all positions in the rectangle are valid positions in the grid.
    ///
    /// ## Performance
    ///
    /// The default implementation uses [`ops::get_rect_unchecked`] to iterate over the rectangle,
    /// involving a call to `get_unchecked` for each element. Other implementations may optimize
    /// this, for example by using a more efficient iteration strategy (for linear reads, etc.).
    unsafe fn rect_iter_unchecked(&self, bounds: Rect) -> impl Iterator<Item = &Self::Element> {
        RowMajor::iter_pos(bounds).map(move |pos| unsafe { self.get_unchecked(pos) })
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

    fn rect_iter(&self, bounds: Rect) -> impl Iterator<Item = &Self::Element> {
        // TODO: Add Size.to_rect()
        let size = unsafe { Rect::from_ltrb_unchecked(0, 0, self.width(), self.height()) };
        let rect = bounds.intersect(size);
        unsafe { self.rect_iter_unchecked(rect) }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;
    use alloc::vec::Vec;

    struct CheckedGridTest {
        grid: [[u8; 3]; 3],
    }

    impl GridBase for CheckedGridTest {
        type Element = u8;
    }

    impl GridRead for CheckedGridTest {
        fn get(&self, pos: Pos) -> Option<&Self::Element> {
            if pos.x < 3 && pos.y < 3 {
                Some(&self.grid[pos.y][pos.x])
            } else {
                None
            }
        }
    }

    struct UncheckedTestGrid {
        grid: [[u8; 3]; 3],
    }

    impl GridBase for UncheckedTestGrid {
        type Element = u8;
    }

    unsafe impl BoundedGrid for UncheckedTestGrid {
        fn width(&self) -> usize {
            3
        }

        fn height(&self) -> usize {
            3
        }
    }

    impl GridReadUnchecked for UncheckedTestGrid {
        unsafe fn get_unchecked(&self, pos: Pos) -> &Self::Element {
            &self.grid[pos.y][pos.x]
        }
    }

    #[test]
    fn test_get_ok() {
        let grid = UncheckedTestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        assert_eq!(grid.get(Pos::new(1, 1)), Some(&5));
    }

    #[test]
    fn test_get_out_of_bounds_x() {
        let grid = UncheckedTestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        assert_eq!(grid.get(Pos::new(3, 1)), None);
    }

    #[test]
    fn test_get_out_of_bounds_y() {
        let grid = UncheckedTestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        assert_eq!(grid.get(Pos::new(1, 3)), None);
    }

    #[test]
    fn test_get_unchecked_ok() {
        let grid = UncheckedTestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let val = unsafe { grid.get_unchecked(Pos::new(2, 2)) };
        assert_eq!(val, &9);
    }

    #[test]
    fn rect_iter_completely_in_bounds() {
        let grid = CheckedGridTest {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let cells = grid
            .rect_iter(Rect::from_ltwh(1, 1, 2, 2).unwrap())
            .collect::<Vec<_>>();
        #[rustfmt::skip]
        assert_eq!(cells, &[
            &5, &6, 
            &8, &9,
        ]);
    }

    #[test]
    fn rect_iter_completely_in_bounds_unchecked_impl() {
        let grid = UncheckedTestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let cells = grid
            .rect_iter(Rect::from_ltwh(1, 1, 2, 2).unwrap())
            .collect::<Vec<_>>();
        #[rustfmt::skip]
        assert_eq!(cells, &[
            &5, &6, 
            &8, &9,
        ]);
    }

    #[test]
    fn rect_iter_partially_out_of_bounds() {
        let grid = CheckedGridTest {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let cells = grid
            .rect_iter(Rect::from_ltwh(0, 0, 4, 4).unwrap())
            .collect::<Vec<_>>();
        #[rustfmt::skip]
        assert_eq!(cells, &[
            &1, &2, &3,
            &4, &5, &6,
            &7, &8, &9,
        ]);
    }

    #[test]
    fn rect_iter_partially_out_of_bounds_unchecked_impl() {
        let grid = UncheckedTestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let cells = grid
            .rect_iter(Rect::from_ltwh(0, 0, 4, 4).unwrap())
            .collect::<Vec<_>>();
        #[rustfmt::skip]
        assert_eq!(cells, &[
            &1, &2, &3,
            &4, &5, &6,
            &7, &8, &9,
        ]);
    }

    #[test]
    fn rect_iter_completely_out_of_bounds() {
        let grid = CheckedGridTest {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let cells = grid
            .rect_iter(Rect::from_ltwh(3, 3, 2, 2).unwrap())
            .collect::<Vec<_>>();
        assert!(cells.is_empty());
    }

    #[test]
    fn rect_iter_completely_out_of_bounds_unchecked_impl() {
        let grid = UncheckedTestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let cells = grid
            .rect_iter(Rect::from_ltwh(3, 3, 2, 2).unwrap())
            .collect::<Vec<_>>();
        assert!(cells.is_empty());
    }
}
