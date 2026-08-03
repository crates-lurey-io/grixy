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
/// use grixy::{core::{Pos, Rect}, transform::GridConvertExt as _, ops::{copy_rect, GridRead, GridWrite}, buf::GridBuf};
///
/// let src = GridBuf::new_filled(3, 3, 1);
/// let mut dst = GridBuf::new(5, 5);
/// copy_rect(&src.copied(), &mut dst, Rect::from_ltwh(0, 0, 3, 3), Pos::new(2, 2));
///
/// assert_eq!(dst.get(Pos::new(2, 2)), Some(&1));
/// assert_eq!(dst.get(Pos::new(4, 4)), Some(&1));
/// assert_eq!(dst.get(Pos::new(5, 5)), None);
/// ```
#[inline]
pub fn copy_rect<'a, E>(
    src: &'a impl GridRead<Element<'a> = E>,
    dst: &mut impl GridWrite<Element = E>,
    from: Rect,
    to: Pos,
) {
    // Pair up each source position with its destination position by offset and copy them one at
    // a time, rather than zipping a (possibly shorter, clip-filtered) value stream against a
    // separate position stream: if either side drops a cell, a zip desyncs and shifts every
    // subsequent cell in the row. `trim_rect` only narrows the candidate area as an optimization;
    // out-of-bounds cells are still skipped individually via `get`/`set`.
    let from = src.trim_rect(from);
    for src_pos in from.pos_iter() {
        let Some(value) = src.get(src_pos) else {
            continue;
        };
        let dst_pos = to + (src_pos - from.top_left());
        let _ = dst.set(dst_pos, value);
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::{ops::GridBase, test::NaiveGrid, transform::GridConvertExt as _};
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
    fn copy_rect_row_alignment_when_destination_clips() {
        // Exact reproduction from https://github.com/crates-lurey-io/grixy/issues/15: a 3x3
        // source copied to (3, 3) of a 5x5 destination clips on the right/bottom. Previously this
        // shifted every row after the first because the destination `set` calls that landed
        // out-of-bounds silently desynced a zipped value stream instead of being skipped in place.
        #[rustfmt::skip]
        let src = NaiveGrid::<i32>::with_cells(3, 3, [
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        copy_rect(
            &src.copied(),
            &mut dst,
            Rect::from_ltwh(0, 0, 3, 3),
            Pos::new(3, 3),
        );

        #[rustfmt::skip]
        assert_eq!(dst.into_iter().collect::<Vec<_>>(),
        &[
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 1, 2,
            0, 0, 0, 4, 5,
        ]);
    }

    /// A grid with no size hint (uses [`GridBase`]'s default), so `trim_rect` never narrows
    /// `from` ahead of time. Used to exercise `copy_rect`'s per-position `get`-returns-`None`
    /// handling directly, rather than relying on `trim_rect` to clamp `from` to the source's real
    /// bounds before iteration ever reaches an out-of-bounds position.
    struct UnhintedGrid<'a>(&'a NaiveGrid<i32>);

    impl GridBase for UnhintedGrid<'_> {}

    impl GridRead for UnhintedGrid<'_> {
        type Element<'b>
            = i32
        where
            Self: 'b;

        type Layout = crate::ops::layout::RowMajor;

        fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
            self.0.get(pos).copied()
        }
    }

    #[test]
    fn copy_rect_row_alignment_when_source_clips_mid_row() {
        #[rustfmt::skip]
        let src = NaiveGrid::<i32>::with_cells(3, 3, [
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ]);
        let unhinted = UnhintedGrid(&src);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        // `from` extends past the (unhinted) source's real 3x3 bounds on the right, and
        // `trim_rect` can't narrow it away (no size hint), so `get` returns `None` for the
        // clipped column on every row.
        copy_rect(
            &unhinted,
            &mut dst,
            Rect::from_ltwh(0, 0, 4, 3),
            Pos::new(0, 0),
        );

        #[rustfmt::skip]
        assert_eq!(dst.into_iter().collect::<Vec<_>>(),
        &[
            1, 2, 3, 0, 0,
            4, 5, 6, 0, 0,
            7, 8, 9, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
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
