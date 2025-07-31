use crate::{
    core::{Layout as _, Pos, Rect, RowMajor},
    ops::convert::{GridCopied, GridMapped},
};

/// Read elements from a 2-dimensional grid position.
pub trait GridRead {
    /// The type of elements in the grid.
    type Element<'a>: 'a
    where
        Self: 'a;

    /// Returns a reference to an element at a specified position.
    ///
    /// If the position is out of bounds, it returns `None`.
    fn get(&self, pos: Pos) -> Option<Self::Element<'_>>;

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
    fn iter_rect(&self, bounds: Rect) -> impl Iterator<Item = Self::Element<'_>> {
        RowMajor::iter_pos(bounds).filter_map(|pos| self.get(pos))
    }

    /// Creates a grid that copies all of its elements.
    ///
    /// This is useful when you have a `GridRead<&T>`, but need a `GridRead<T>`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use grixy::{core::Pos, ops::GridRead, buf::VecGrid};
    ///
    /// // By default, `VecGrid` returns references to its elements (similar to `Vec`).
    /// let grid = VecGrid::new_filled_row_major(3, 3, 1);
    /// assert_eq!(grid.get(Pos::new(1, 1)), Some(&1));
    ///
    /// // We can create a `GridRead` that returns owned copies of the elements.
    /// let copied = grid.copied();
    /// assert_eq!(copied.get(Pos::new(1, 1)), Some(1));
    /// ```
    fn copied<'a, T>(&'a self) -> GridCopied<'a, T, Self>
    where
        Self: Sized,
        Self: GridRead<Element<'a> = &'a T>,
        T: 'a + Copy,
    {
        GridCopied { source: self }
    }

    /// Creates a grid that applies a mapping function to its elements.
    ///
    /// This is useful when you want to transform the elements of a grid lazily.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use grixy::{core::Pos, ops::GridRead, buf::VecGrid};
    ///
    /// let grid = VecGrid::new_filled_row_major(3, 3, 1);
    /// let mapped = grid.map(|&x| x * 2);
    /// assert_eq!(mapped.get(Pos::new(1, 1)), Some(2));
    /// ```
    fn map<'a, S, F, T>(&'a self, map_fn: F) -> GridMapped<'a, S, F, Self, T>
    where
        Self: Sized,
        Self: GridRead<Element<'a> = S>,
        S: 'a,
        T: 'a,
        F: Fn(S) -> T,
    {
        GridMapped {
            source: self,
            map_fn,
        }
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
        type Element<'a> = u8;

        fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
            if pos.x < 3 && pos.y < 3 {
                Some(self.grid[pos.y][pos.x])
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
            5, 6,
            8, 9,
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
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
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
