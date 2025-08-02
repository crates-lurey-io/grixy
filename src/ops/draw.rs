use crate::{
    core::{Pos, Rect},
    ops::{GridRead, GridWrite},
};

/// Copies a rectangular region from a source grid to a destination grid.
///
/// The operation starts by copying the top-left corner to the specified offset; if there is
/// insufficient space in the current grid, or the rectangle is out of bounds of the source grid,
/// those individual cells are ignored and not copied to/from.
///
/// ## Examples
///
/// ```rust
/// use grixy::{core::{Pos, Rect}, convert::GridConvertExt as _, ops::{copy_rect, GridRead, GridWrite}, buf::GridBuf};
///
/// let src = GridBuf::new_filled(3, 3, 1);
/// let mut dst = GridBuf::new(5, 5);
/// copy_rect(&src.copied(), &mut dst, Rect::from_ltwh(0, 0, 3, 3), Pos::new(2, 2));
///
/// assert_eq!(dst.get(Pos::new(2, 2)), Some(&1));
/// assert_eq!(dst.get(Pos::new(4, 4)), Some(&1));
/// assert_eq!(dst.get(Pos::new(5, 5)), None);
/// ```
pub fn copy_rect<'s, E>(
    src: &'s impl GridRead<Element<'s> = E>,
    dst: &mut impl GridWrite<Element = E>,
    from: Rect,
    to: Pos,
) {
    dst.fill_rect_iter(
        Rect::from_ltwh(to.x, to.y, from.width(), from.height()),
        src.iter_rect(from),
    );
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::{convert::GridConvertExt as _, test::NaiveGrid};
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
        copy_rect(
            &src.copied(),
            &mut dst,
            Rect::from_ltwh(0, 0, 3, 3),
            Pos::new(2, 2),
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
    fn copy_rect_partially_out_of_bounds() {
        #[rustfmt::skip]
        let src = NaiveGrid::<i32>::with_cells(3, 3, [
            1, 1, 1,
            1, 1, 1,
            1, 1, 1,
        ]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        copy_rect(
            &src.copied(),
            &mut dst,
            Rect::from_ltwh(0, 0, 3, 3),
            Pos::new(4, 4),
        );

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
        copy_rect(
            &src.copied(),
            &mut dst,
            Rect::from_ltwh(0, 0, 3, 3),
            Pos::new(6, 6),
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
