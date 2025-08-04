use crate::{
    core::{HasSize as _, Pos, Rect},
    ops::{
        GridRead,
        layout::{self, Layout as _},
        unchecked::TrustedSizeGrid,
    },
};

/// Read elements from a 2-dimensional grid position without bounds checking.
pub trait GridReadUnchecked {
    /// The type of elements in the grid.
    type Element<'a>: 'a
    where
        Self: 'a;

    /// The layout of the grid, which determines how elements are stored and accessed.
    type Layout: layout::Layout;

    /// Returns an element, without doing bounds checking.
    ///
    /// ## Safety
    ///
    /// Calling this method with an out-of-bounds position is _[undefined behavior][]_.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    unsafe fn get_unchecked(&self, pos: Pos) -> Self::Element<'_>;

    /// Returns an iterator over elements in a rectangular region of the grid.
    ///
    /// Elements are returned in an order agreeable to the grid's internal layout.
    ///
    /// The bounding rectangle is treated as _exclusive_ of the right and bottom edges.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that all positions in the rectangle are valid positions in the grid.
    ///
    /// ## Performance
    ///
    /// The default implementation iterates over the rectangle in a traversal order defined by
    /// [`GridReadUnchecked::Layout`], making an individual call to `get_unchecked` for each
    /// position in the rectangle.
    ///
    /// Implementations may optimize this, for example by using a more efficient iteration strategy
    /// (for linear reads, etc.).
    unsafe fn iter_rect_unchecked(&self, bounds: Rect) -> impl Iterator<Item = Self::Element<'_>> {
        layout::RowMajor::iter_pos(bounds).map(move |pos| unsafe { self.get_unchecked(pos) })
    }
}

/// Automatically implement `GridRead` when `GridReadUnchecked` + `TrustedSizeGrid` are implemented.
impl<T: GridReadUnchecked + TrustedSizeGrid> GridRead for T {
    type Element<'a>
        = T::Element<'a>
    where
        Self: 'a;

    type Layout = T::Layout;

    fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
        if self.contains(pos) {
            Some(unsafe { self.get_unchecked(pos) })
        } else {
            None
        }
    }

    fn iter_rect(&self, bounds: Rect) -> impl Iterator<Item = Self::Element<'_>> {
        let size = self.size().to_rect();
        let rect = bounds.intersect(size);
        unsafe { self.iter_rect_unchecked(rect) }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::ops::{layout::RowMajor, unchecked::TrustedSizeGrid};
    use alloc::vec::Vec;

    struct UncheckedTestGrid {
        grid: [[u8; 3]; 3],
    }

    unsafe impl TrustedSizeGrid for UncheckedTestGrid {
        fn width(&self) -> usize {
            3
        }

        fn height(&self) -> usize {
            3
        }
    }

    impl GridReadUnchecked for UncheckedTestGrid {
        type Element<'a> = u8;

        type Layout = RowMajor;

        unsafe fn get_unchecked(&self, pos: Pos) -> Self::Element<'_> {
            self.grid[pos.y][pos.x]
        }

        unsafe fn iter_rect_unchecked(
            &self,
            _bounds: Rect,
        ) -> impl Iterator<Item = Self::Element<'_>> {
            core::iter::empty()
        }
    }

    #[test]
    fn test_get_ok() {
        let grid = UncheckedTestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        assert_eq!(grid.get(Pos::new(1, 1)), Some(5));
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
        assert_eq!(val, 9);
    }

    #[test]
    fn rect_iter_completely_in_bounds_unchecked_impl() {
        let grid = UncheckedTestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let cells = grid
            .iter_rect(Rect::from_ltwh(1, 1, 2, 2))
            .collect::<Vec<_>>();
        #[rustfmt::skip]
        assert_eq!(cells, &[
            5, 6,
            8, 9,
        ]);
    }

    #[test]
    fn rect_iter_partially_out_of_bounds_unchecked_impl() {
        let grid = UncheckedTestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let cells = grid
            .iter_rect(Rect::from_ltwh(0, 0, 4, 4))
            .collect::<Vec<_>>();
        #[rustfmt::skip]
        assert_eq!(cells, &[
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ]);
    }

    #[test]
    fn rect_iter_completely_out_of_bounds_unchecked_impl() {
        let grid = UncheckedTestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let cells = grid
            .iter_rect(Rect::from_ltwh(3, 3, 2, 2))
            .collect::<Vec<_>>();
        assert!(cells.is_empty());
    }
}
