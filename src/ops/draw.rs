use crate::{
    core::{HasSize, Pos, Rect},
    ops::{GridRead, GridWrite},
};

pub mod blend;

/// Perform draw-like operations by writing to a 2-dimensional grid.
pub trait GridDraw: GridWrite {
    /// Copies a rectangular `src_rect` from a `src` grid.
    ///
    /// The operation starts by copying the top-left corner to the specified `dst_pos`; if there
    /// is insufficient space in the current grid, or the `src_rect` is out of bounds of the `src`
    /// grid, those individual cells are ignored and not copied to/from.
    ///
    /// ## Performance
    ///
    /// The default implementation reads each cell from the `src` grid and writes it to the
    /// destination grid at the specified `dst_pos`, ignoring any cells (on either read or write)
    /// that are out of bounds. This is a straightforward implementation that may not be optimal for
    /// all grid types.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use grixy::{core::{Pos, Rect}, buf::GridBuf, ops::{GridDraw, GridRead}};
    ///
    /// let src = GridBuf::new_filled(3, 3, 1);
    /// let mut dst = GridBuf::new_filled(5, 5, 0);
    ///
    /// // Copy a 3x3 region from the source grid to the destination grid at position (2, 2).
    /// dst.copy_rect(&src.copied(), Rect::from_ltwh(0, 0, 3, 3), Pos::new(2, 2));
    ///
    /// assert_eq!(dst.iter_rect(Rect::from_ltwh(0, 0, 5, 5)).copied().collect::<Vec<_>>(),
    ///            &[0, 0, 0, 0, 0,
    ///              0, 0, 0, 0, 0,
    ///              0, 0, 1, 1, 1,
    ///              0, 0, 1, 1, 1,
    ///              0, 0, 1, 1, 1]);
    /// ```
    fn copy_rect<'src>(
        &mut self,
        src: &'src impl GridRead<Element<'src> = Self::Element>,
        src_rect: Rect,
        dst_pos: Pos,
    ) {
        self.fill_rect_iter(
            Rect::from_ltwh(dst_pos.x, dst_pos.y, src_rect.width(), src_rect.height()),
            src.iter_rect(src_rect),
        );
    }

    /// Copies a rectangular `src_rect` from a `src` grid, scaling the copy by a specified factor.
    ///
    /// The operation starts by copying the top-left corner to the specified `dst_pos`, scaling each
    /// cell by the `scale` factor. If there is insufficient space in the current grid or the
    /// `src_rect` is out of bounds of the `src` grid, those individual cells are ignored and not
    /// copied to/from.
    ///
    /// ## Performance
    ///
    /// The default implementation reads each cell from the `src` grid, scales it by the `scale`
    /// factor, and writes it to the destination grid at the specified `dst_pos`, ignoring
    /// any cells (on either read or write) that are out of bounds. This is a straightforward
    /// implementation that may not be optimal for all grid types.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use grixy::{core::{Pos, Rect}, buf::GridBuf, ops::{GridDraw, GridRead}};
    ///
    /// let src = GridBuf::<_, _>::from_buffer(vec![1, 2, 3, 4], 2);
    /// let mut dst = GridBuf::new_filled(5, 5, 0);
    ///
    /// assert_eq!(src.iter_rect(Rect::from_ltwh(0, 0, 2, 2)).copied().collect::<Vec<_>>(),
    ///           &[1, 2,
    ///             3, 4]);
    ///
    /// // Copy a 2x2 region from the source grid to the destination grid at position (1, 1),
    /// // scaling each cell by a factor of 2.
    /// dst.copy_rect_scaled(&src.copied(), Rect::from_ltwh(0, 0, 2, 2), Rect::from_ltwh(1, 1, 4, 4));
    ///
    /// assert_eq!(dst.iter_rect(Rect::from_ltwh(0, 0, 5, 5)).copied().collect::<Vec<_>>(),
    ///            &[0, 0, 0, 0, 0,
    ///              0, 1, 1, 2, 2,
    ///              0, 1, 1, 2, 2,
    ///              0, 3, 3, 4, 4,
    ///              0, 3, 3, 4, 4]);
    /// ```
    fn copy_rect_scaled<'src>(
        &mut self,
        src: &'src impl GridRead<Element<'src> = Self::Element>,
        src_rect: Rect,
        dst_rect: Rect,
    ) {
        if src_rect.is_empty() || dst_rect.is_empty() {
            return;
        }
        if src_rect.size() == dst_rect.size() {
            return self.copy_rect(src, src_rect, dst_rect.top_left());
        }
        for y in 0..dst_rect.height() {
            for x in 0..dst_rect.width() {
                let src_x = x * src_rect.width() / dst_rect.width();
                let src_y = y * src_rect.height() / dst_rect.height();

                let src_pos = src_rect.top_left() + Pos::new(src_x, src_y);
                let dst_pos = dst_rect.top_left() + Pos::new(x, y);

                if let Some(value) = src.get(src_pos) {
                    let _ = self.set(dst_pos, value);
                }
            }
        }
    }
}

impl<T> GridDraw for T where T: GridWrite {}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::test::NaiveGrid;
    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn copy_rect_within_bounds() {
        #[rustfmt::skip]
        let src = NaiveGrid::<i32>::with_cells(3, 3, [
            1, 1, 1,
            1, 1, 1,
            1, 1, 1,
        ]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        dst.copy_rect(&src.copied(), Rect::from_ltwh(0, 0, 3, 3), Pos::new(2, 2));

        #[rustfmt::skip]
        assert_eq!(dst.into_iter().collect::<Vec<_>>(),
        &[
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 1, 1, 1,
            0, 0, 1, 1, 1,
            0, 0, 1, 1, 1,
        ]);
    }

    #[test]
    fn copy_rect_partially_out_of_bounds() {
        #[rustfmt::skip]
        let src = NaiveGrid::<i32>::with_cells(3, 3, [
            1, 1, 1,
            1, 1, 1,
            1, 1, 1,
        ]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        dst.copy_rect(&src.copied(), Rect::from_ltwh(0, 0, 3, 3), Pos::new(4, 4));

        #[rustfmt::skip]
        assert_eq!(dst.into_iter().collect::<Vec<_>>(),
        &[
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 1,
        ]);
    }

    #[test]
    fn copy_rect_completely_outof_bounds() {
        #[rustfmt::skip]
        let src = NaiveGrid::<i32>::with_cells(3, 3, [
            1, 1, 1,
            1, 1, 1,
            1, 1, 1,
        ]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        dst.copy_rect(&src.copied(), Rect::from_ltwh(0, 0, 3, 3), Pos::new(6, 6));

        #[rustfmt::skip]
        assert_eq!(dst.into_iter().collect::<Vec<_>>(),
        &[
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0,
        ]);
    }

    #[test]
    fn copy_rect_scaled_0_noop() {
        let src = NaiveGrid::<i32>::with_cells(3, 3, [1, 1, 1, 1, 1, 1, 1, 1, 1]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        dst.copy_rect_scaled(
            &src.copied(),
            Rect::from_ltwh(0, 0, 3, 3),
            Rect::from_ltwh(2, 2, 0, 0),
        );

        #[rustfmt::skip]
        assert_eq!(dst.into_iter().collect::<Vec<_>>(),
        &[
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ]);
    }

    #[test]
    fn copy_rect_scaled_1_no_scale() {
        let src = NaiveGrid::<i32>::with_cells(3, 3, [1, 1, 1, 1, 1, 1, 1, 1, 1]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        dst.copy_rect_scaled(
            &src.copied(),
            Rect::from_ltwh(0, 0, 3, 3),
            Rect::from_ltwh(2, 2, 3, 3),
        );

        #[rustfmt::skip]
        assert_eq!(dst.into_iter().collect::<Vec<_>>(),
        &[
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 1, 1, 1,
            0, 0, 1, 1, 1,
            0, 0, 1, 1, 1,
        ]);
    }

    #[test]
    fn copy_rect_scaled_2x2_to_4x4() {
        let src = NaiveGrid::<i32>::with_cells(2, 2, [1, 2, 3, 4]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        dst.copy_rect_scaled(
            &src.copied(),
            Rect::from_ltwh(0, 0, 2, 2),
            Rect::from_ltwh(1, 1, 4, 4),
        );

        #[rustfmt::skip]
        assert_eq!(dst.into_iter().collect::<Vec<_>>(),
        &[
            0, 0, 0, 0, 0,
            0, 1, 1, 2, 2,
            0, 1, 1, 2, 2,
            0, 3, 3, 4, 4,
            0, 3, 3, 4, 4,
        ]);
    }

    #[test]
    fn copy_rect_scaled_2x2_to_4x4_partially_out_of_bounds() {
        let src = NaiveGrid::<i32>::with_cells(2, 2, [1, 2, 3, 4]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        dst.copy_rect_scaled(
            &src.copied(),
            Rect::from_ltwh(0, 0, 2, 2),
            Rect::from_ltwh(2, 2, 4, 4),
        );

        #[rustfmt::skip]
        assert_eq!(dst.into_iter().collect::<Vec<_>>(),
        &[
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 1, 1, 2,
            0, 0, 1, 1, 2,
            0, 0, 3, 3, 4,
        ]);
    }

    #[test]
    fn copy_rect_scaled_2x2_to_4x4_completely_out_of_bounds() {
        let src = NaiveGrid::<i32>::with_cells(2, 2, [1, 2, 3, 4]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        dst.copy_rect_scaled(
            &src.copied(),
            Rect::from_ltwh(0, 0, 2, 2),
            Rect::from_ltwh(6, 6, 4, 4),
        );

        #[rustfmt::skip]
        assert_eq!(dst.into_iter().collect::<Vec<_>>(),
        &[
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ]);
    }
}
