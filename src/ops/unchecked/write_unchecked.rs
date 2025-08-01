use crate::{
    core::{GridError, HasSize, Layout, Pos, Rect},
    ops::{GridWrite, unchecked::TrustedSizeGrid},
};

/// Write elements to a 2-dimensional grid position without bounds checking.
pub trait GridWriteUnchecked {
    /// The type of elements in the grid.
    type Element;

    /// The type of layout used for the grid.
    type Layout: Layout;

    /// Sets the element at a specified position without bounds checking.
    ///
    /// ## Safety
    ///
    /// Calling this method with an out-of-bounds position is _[undefined behavior][]_.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    unsafe fn set_unchecked(&mut self, pos: Pos, value: Self::Element);

    /// Sets elements within a rectangular region of the grid without bounds checking.
    ///
    /// Elements are set in an order agreeable to the grid's internal layout. Out-of-bounds
    /// elements are skipped, and the bounding rectangle is treated as _exclusive_ of the right and
    /// bottom edges.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that all positions in the rectangle are valid positions in the grid.
    ///
    /// ## Performance
    ///
    /// The default implementation uses [`Layout::iter_pos`] to iterate over the rectangle,
    /// involving a call to [`GridWriteUnchecked::set_unchecked`] for each element. Other
    /// implementations may optimize this, for example by using a more efficient iteration strategy
    /// (for linear writes, etc.).
    unsafe fn fill_rect_unchecked(&mut self, dst: Rect, mut f: impl FnMut(Pos) -> Self::Element) {
        Self::Layout::iter_pos(dst).for_each(|pos| unsafe {
            self.set_unchecked(pos, f(pos));
        });
    }

    /// Sets elements within a rectangular region of the grid without bounds checking.
    ///
    /// Elements are set in an order agreeable to the grid's internal layout. Out-of-bounds
    /// elements are skipped, and the bounding rectangle is treated as _exclusive_ of the right and
    /// bottom edges.
    ///
    /// If the provided iterator has fewer elements than the rectangle, the remaining elements will
    /// not be set. If the iterator has more elements than the rectangle, the behavior is undefined.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that all positions in the rectangle are valid positions in the grid
    /// and that the iterator does not yield more elements than the rectangle can hold.
    ///
    /// ## Performance
    ///
    /// The default implementation uses [`Layout::iter_pos`] to iterate over the rectangle,
    /// involving a call to [`GridWriteUnchecked::set_unchecked`] for each element. Other
    /// implementations may optimize this, for example by using a more efficient iteration strategy
    /// (for linear writes, etc.).
    unsafe fn fill_rect_iter_unchecked(
        &mut self,
        dst: Rect,
        iter: impl IntoIterator<Item = Self::Element>,
    ) {
        Self::Layout::iter_pos(dst)
            .zip(iter)
            .for_each(|(pos, value)| unsafe {
                self.set_unchecked(pos, value);
            });
    }

    /// Sets elements within a rectangular region of the grid without bounds checking.
    ///
    /// Elements are set in an order agreeable to the grid's internal layout. Out-of-bounds
    /// elements are skipped, and the bounding rectangle is treated as _exclusive_ of the right and
    /// bottom edges.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that all positions in the rectangle are valid positions in the grid.
    ///
    /// ## Performance
    ///
    /// The default implementation uses [`Layout::iter_pos`] to iterate over the rectangle,
    /// involving bounds checking for each element. Other implementations may optimize this, for
    /// example by using a more efficient iteration strategy (for linear reads, reduced bounds
    /// checking, etc.).
    unsafe fn fill_rect_solid_unchecked(&mut self, bounds: Rect, value: Self::Element)
    where
        Self::Element: Copy,
    {
        unsafe { self.fill_rect_unchecked(bounds, |_| value) };
    }
}

/// Automatically implement `GridWrite` when `GridWriteUnchecked` + `TrustedSizeGrid` are implemented.
impl<T: GridWriteUnchecked + TrustedSizeGrid> GridWrite for T {
    type Element = T::Element;
    type Layout = T::Layout;

    fn set(&mut self, pos: Pos, value: Self::Element) -> Result<(), GridError> {
        if self.contains(pos) {
            unsafe {
                self.set_unchecked(pos, value);
            }
            Ok(())
        } else {
            Err(GridError)
        }
    }

    fn fill_rect(&mut self, bounds: Rect, f: impl FnMut(Pos) -> Self::Element) {
        let size = self.size().to_rect();
        let rect = bounds.intersect(size);
        unsafe { self.fill_rect_unchecked(rect, f) }
    }

    fn fill_rect_iter(&mut self, dst: Rect, iter: impl IntoIterator<Item = Self::Element>) {
        let size = self.size().to_rect();
        let rect = dst.intersect(size);
        unsafe { self.fill_rect_iter_unchecked(rect, iter) }
    }

    fn fill_rect_solid(&mut self, dst: Rect, value: Self::Element)
    where
        Self::Element: Copy,
    {
        let size = self.size().to_rect();
        let rect = dst.intersect(size);
        unsafe { self.fill_rect_solid_unchecked(rect, value) }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::core::RowMajor;
    use alloc::vec;

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

    impl GridWriteUnchecked for UncheckedTestGrid {
        type Element = u8;
        type Layout = RowMajor;

        unsafe fn set_unchecked(&mut self, pos: Pos, value: Self::Element) {
            self.grid[pos.y][pos.x] = value;
        }
    }

    #[test]
    fn impl_unsafe_set_ok() {
        let mut grid = UncheckedTestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 1, y: 1 };
        grid.set(pos, 42).unwrap();
        assert_eq!(grid.grid[1][1], 42);
    }

    #[test]
    fn impl_unsafe_set_out_of_bounds_x() {
        let mut grid = UncheckedTestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 3, y: 1 };
        assert!(grid.set(pos, 42).is_err());
        assert_eq!(grid.grid, [[0, 0, 0], [0, 0, 0], [0, 0, 0]]);
    }

    #[test]
    fn impl_unsafe_set_out_of_bounds_y() {
        let mut grid = UncheckedTestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 1, y: 3 };
        assert!(grid.set(pos, 42).is_err());
        assert_eq!(grid.grid, [[0, 0, 0], [0, 0, 0], [0, 0, 0]]);
    }

    #[test]
    fn impl_unsafe_set_unchecked_in_bounds() {
        let mut grid = UncheckedTestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 2, y: 2 };
        unsafe {
            grid.set_unchecked(pos, 99);
        }
        assert_eq!(grid.grid[2][2], 99);
    }

    #[test]
    fn impl_unsafe_fill_rect_complete() {
        let mut grid = UncheckedTestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 3, 3).unwrap();
        grid.fill_rect(bounds, |_| 42);
        assert_eq!(grid.grid, [[42; 3]; 3]);
    }

    #[test]
    fn impl_unsafe_fill_rect_partial_in_bounds() {
        let mut grid = UncheckedTestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 2, 2).unwrap();
        grid.fill_rect(bounds, |pos| if pos.x == 1 && pos.y == 1 { 99 } else { 42 });
        assert_eq!(grid.grid, [[42, 42, 0], [42, 99, 0], [0, 0, 0]]);
    }

    #[test]
    fn impl_unsafe_fill_rect_partial_out_of_bounds() {
        let mut grid = UncheckedTestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(1, 1, 4, 4).unwrap(); // Out of bounds on the right and bottom
        grid.fill_rect(bounds, |_| 42);
        assert_eq!(grid.grid, [[0, 0, 0], [0, 42, 42], [0, 42, 42]]);
    }

    #[test]
    fn impl_unsafe_fill_rect_iter_complete() {
        let mut grid = UncheckedTestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 3, 3).unwrap();
        grid.fill_rect_iter(bounds, vec![42; 9]);
        assert_eq!(grid.grid, [[42; 3]; 3]);
    }

    #[test]
    fn impl_unsafe_fill_rect_iter_partial_in_bounds() {
        let mut grid = UncheckedTestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 2, 2).unwrap();
        grid.fill_rect_iter(bounds, vec![42, 99]);

        #[rustfmt::skip]
        assert_eq!(grid.grid, [
            [42, 99, 0],
            [0,  0,  0],
            [0,  0,  0]
        ]);
    }

    #[test]
    fn impl_unsafe_fill_rect_iter_partial_in_bounds_with_extra() {
        let mut grid = UncheckedTestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 2, 1).unwrap();
        grid.fill_rect_iter(bounds, vec![42, 99, 100]);

        #[rustfmt::skip]
        assert_eq!(grid.grid, [
            [42, 99, 0],
            [0,  0,  0],
            [0,  0,  0]
        ]);
    }

    #[test]
    fn impl_unsafe_fill_rect_iter_partial_out_of_bounds() {
        let mut grid = UncheckedTestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(1, 1, 4, 4).unwrap(); // Out of bounds on the right and bottom
        grid.fill_rect_iter(bounds, vec![42, 99, 100]);

        #[rustfmt::skip]
        assert_eq!(grid.grid, [
            [0, 0, 0],
            [0, 42, 99],
            [0, 100, 0]
        ]);
    }

    #[test]
    fn impl_unsafe_fill_rect_iter_out_of_bounds() {
        let mut grid = UncheckedTestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(3, 3, 4, 4).unwrap(); // Out of bounds on the right and bottom
        grid.fill_rect_iter(bounds, vec![42, 99, 100]);

        #[rustfmt::skip]
        assert_eq!(grid.grid, [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ]);
    }

    #[test]
    fn impl_unsafe_fill_rect_solid() {
        let mut grid = UncheckedTestGrid { grid: [[0; 3]; 3] };
        let bounds = Rect::from_ltrb(0, 0, 3, 3).unwrap();
        grid.fill_rect_solid(bounds, 42);

        assert_eq!(grid.grid, [[42; 3]; 3]);
    }
}
