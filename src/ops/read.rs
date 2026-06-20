use crate::{
    core::{Pos, Rect},
    ops::{
        ExactSizeGrid, GridBase,
        layout::{self, Traversal as _},
    },
};

/// Read elements from a 2-dimensional grid position.
pub trait GridRead: GridBase {
    /// The type of elements in the grid.
    type Element<'a>: 'a
    where
        Self: 'a;

    /// The type of layout used for the grid.
    ///
    /// ## Implementation
    ///
    /// It is not guaranteed that the internal storage of the grid matches this layout, but methods
    /// that return iterators over the grid's elements or positions should return them in the
    /// traversal order defined by this layout.
    ///
    /// [`RowMajor`][layout::RowMajor] is a reasonable default implementation for most grids.
    type Layout: layout::Traversal;

    /// Returns a reference to an element at a specified position.
    ///
    /// If the position is out of bounds, it returns `None`.
    fn get(&self, pos: Pos) -> Option<Self::Element<'_>>;

    /// Returns an iterator over elements in a rectangular region of the grid.
    ///
    /// Elements are returned in an order agreeable to the grid's internal layout. Out-of-bounds
    /// elements are skipped, and the bounding rectangle is treated as _exclusive_ of the right and
    /// bottom edges.
    ///
    /// ## Performance
    ///
    /// The default implementation uses [`Traversal::iter_pos`] to iterate over the rectangle,
    /// involving bounds checking for each element. Other implementations may optimize this, for
    /// example by using a more efficient iteration strategy (for linear reads, reduced bounds
    /// checking, etc.).
    ///
    /// [`Traversal::iter_pos`]: layout::Traversal::iter_pos
    fn iter_rect(&self, bounds: Rect) -> impl Iterator<Item = Self::Element<'_>> {
        Self::Layout::iter_pos(self.trim_rect(bounds)).filter_map(move |pos| self.get(pos))
    }

    /// Returns an iterator over `(position, element)` pairs in a rectangular region.
    ///
    /// Positions and elements are yielded in the traversal order defined by `Self::Layout`.
    /// Out-of-bounds positions are skipped.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use grixy::prelude::*;
    ///
    /// let grid = GridBuf::new_filled(3, 3, 42u8);
    /// let pairs: Vec<_> = grid
    ///     .iter_rect_with_pos(Rect::from_ltwh(1, 1, 2, 2))
    ///     .collect();
    /// assert_eq!(pairs[0], (Pos::new(1, 1), &42u8));
    /// ```
    fn iter_rect_with_pos(
        &self,
        bounds: Rect,
    ) -> impl Iterator<Item = (Pos, Self::Element<'_>)> {
        let trimmed = self.trim_rect(bounds);
        Self::Layout::iter_pos(trimmed).filter_map(move |pos| {
            self.get(pos).map(|elem| (pos, elem))
        })
    }
}

/// A trait for grids that can be iterated over.
pub trait GridIter: GridRead {
    /// Returns an iterator over the elements of the grid.
    fn iter(&self) -> impl Iterator<Item = Self::Element<'_>>;

    /// Returns an iterator over `(position, element)` pairs for the entire grid.
    ///
    /// This is the grixy equivalent of `rg::Grid::cells()`.
    fn iter_with_pos(&self) -> impl Iterator<Item = (Pos, Self::Element<'_>)>;

    /// Alias for [`iter_with_pos`](GridIter::iter_with_pos).
    ///
    /// Matches the naming convention of `ndarray::indexed_iter()` and
    /// `image::ImageBuffer::enumerate_pixels()`.
    fn cells(&self) -> impl Iterator<Item = (Pos, Self::Element<'_>)> {
        self.iter_with_pos()
    }
}

impl<T> GridIter for T
where
    T: GridRead + ExactSizeGrid,
{
    fn iter(&self) -> impl Iterator<Item = Self::Element<'_>> {
        self.iter_rect(Rect::from_ltwh(0, 0, self.width(), self.height()))
    }

    fn iter_with_pos(&self) -> impl Iterator<Item = (Pos, Self::Element<'_>)> {
        self.iter_rect_with_pos(Rect::from_ltwh(0, 0, self.width(), self.height()))
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use crate::{buf::GridBuf, core::Size, ops::layout::RowMajor, transform::GridConvertExt as _};
    use alloc::vec::Vec;

    struct CheckedGridTest {
        grid: [[u8; 3]; 3],
    }

    impl GridBase for CheckedGridTest {
        fn size_hint(&self) -> (Size, Option<Size>) {
            let size = Size::new(3, 3);
            (size, Some(size))
        }
    }

    impl GridRead for CheckedGridTest {
        type Element<'a> = u8;

        type Layout = RowMajor;

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

    #[test]
    fn collect() {
        let grid = GridBuf::new_filled(3, 3, 1);
        let collected = grid.copied().flatten::<Vec<_>, RowMajor>();
        assert_eq!(collected.get(Pos::new(1, 1)), Some(&1));
        assert_eq!(collected.get(Pos::new(3, 3)), None);
    }

    #[test]
    fn iter() {
        let grid = GridBuf::new_filled(3, 3, 1);
        let collected: Vec<_> = grid.copied().iter().collect();
        assert_eq!(collected.len(), 9);
        assert!(collected.iter().all(|&x| x == 1));
    }
}
