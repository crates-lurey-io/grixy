use ixy::index::{Layout, RowMajor};

use crate::core::{Pos, Rect};

/// Read elements from a 2-dimensional grid position.
pub trait GridRead {
    /// The type of elements in the grid.
    type Element;

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
    /// The default implementation uses [`RowMajor::iter_pos`] to iterate over the rectangle,
    /// involving bounds checking for each element. Other implementations may optimize this, for
    /// example by using a more efficient iteration strategy (for linear reads, reduced bounds
    /// checking, etc.).
    fn iter_rect(&self, bounds: Rect) -> impl Iterator<Item = &Self::Element> {
        RowMajor::iter_pos(bounds).filter_map(|pos| self.get(pos))
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

    impl GridRead for CheckedGridTest {
        type Element = u8;

        fn get(&self, pos: Pos) -> Option<&Self::Element> {
            if pos.x < 3 && pos.y < 3 {
                Some(&self.grid[pos.y][pos.x])
            } else {
                None
            }
        }
    }

    #[test]
    fn rect_iter_completely_in_bounds() {
        let grid = CheckedGridTest {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let cells = grid
            .iter_rect(Rect::from_ltwh(1, 1, 2, 2))
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
            .iter_rect(Rect::from_ltwh(0, 0, 4, 4))
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
            .iter_rect(Rect::from_ltwh(3, 3, 2, 2))
            .collect::<Vec<_>>();
        assert!(cells.is_empty());
    }
}
