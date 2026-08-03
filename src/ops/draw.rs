use crate::{
    core::{Pos, Rect},
    ops::{ExactSizeGrid, GridRead, GridWrite},
};

/// Copies a rectangular region from a source grid to a destination grid.
///
/// The operation starts by copying the top-left corner to the specified offset; if there is
/// insufficient space in the current grid, or the rectangle is out of bounds of the source grid,
/// those individual cells are ignored and not copied to/from.
///
/// See [`copy_rect_clamped`] for a variant that's more efficient when `src`/`dst` implement
/// [`ExactSizeGrid`], since it doesn't depend on
/// [`GridBase::trim_rect`][crate::ops::GridBase::trim_rect]'s best-effort hint to bound its work.
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

/// Copies a rectangular region from a source grid to a destination grid, clamping both
/// rectangles to their respective grid bounds before copying.
///
/// `from` is first clamped to `src`'s bounds, the equivalent destination rectangle is translated
/// from that clamp and clamped again to `dst`'s bounds, and the resulting rectangles - always the
/// same size as each other, since the tighter of the two clamps wins on both sides - are copied
/// position-for-position.
///
/// This requires [`ExactSizeGrid`] (in addition to [`GridRead`]/[`GridWrite`]) so both clamps can
/// use the grid's authoritative size, rather than [`copy_rect`]'s best-effort
/// [`trim_rect`][crate::ops::GridBase::trim_rect] hint: the copy only ever visits positions in the
/// overlap of `from`, `src`, the translated destination rectangle, and `dst`, so it stays
/// efficient even when `from` is much larger than `src`'s actual bounds.
///
/// If the clamped region ends up empty (`from` doesn't overlap `src` at all, or the translated
/// destination rectangle doesn't overlap `dst` at all), this is a no-op.
///
/// ## Examples
///
/// ```rust
/// use grixy::{core::{Pos, Rect}, transform::GridConvertExt as _, ops::{copy_rect_clamped, GridRead, GridWrite}, buf::GridBuf};
///
/// let src = GridBuf::new_filled(3, 3, 1);
/// let mut dst = GridBuf::new(5, 5);
/// copy_rect_clamped(&src.copied(), &mut dst, Rect::from_ltwh(0, 0, 3, 3), Pos::new(2, 2));
///
/// assert_eq!(dst.get(Pos::new(2, 2)), Some(&1));
/// assert_eq!(dst.get(Pos::new(4, 4)), Some(&1));
/// assert_eq!(dst.get(Pos::new(5, 5)), None);
/// ```
#[inline]
pub fn copy_rect_clamped<'a, E>(
    src: &'a (impl ExactSizeGrid + GridRead<Element<'a> = E>),
    dst: &mut (impl ExactSizeGrid + GridWrite<Element = E>),
    from: Rect,
    to: Pos,
) {
    let from = from.intersect(Rect::from_ltwh(0, 0, src.width(), src.height()));
    if from.is_empty() {
        return;
    }

    let to = Rect::from_ltwh(to.x, to.y, from.width(), from.height()).intersect(Rect::from_ltwh(
        0,
        0,
        dst.width(),
        dst.height(),
    ));
    if to.is_empty() {
        return;
    }

    // `to` may have been clamped tighter than `from` (e.g. if `to` sits near the destination's
    // edge); shrink `from` to match before copying, so both rectangles are the same size and
    // `pos_iter` visits them in lockstep.
    let from = Rect::from_ltwh(from.left(), from.top(), to.width(), to.height());
    for (src_pos, dst_pos) in from.pos_iter().zip(to.pos_iter()) {
        if let Some(value) = src.get(src_pos) {
            let _ = dst.set(dst_pos, value);
        }
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

    #[test]
    fn copy_rect_clamped_within_bounds() {
        #[rustfmt::skip]
        let src = NaiveGrid::<i32>::with_cells(3, 3, [
            1, 1, 1,
            1, 1, 1,
            1, 1, 1,
        ]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        copy_rect_clamped(
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
    fn copy_rect_clamped_source_side_clip() {
        #[rustfmt::skip]
        let src = NaiveGrid::<i32>::with_cells(3, 3, [
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        // `from` is far larger than `src`'s actual 3x3 bounds; only the overlap is copied.
        copy_rect_clamped(
            &src.copied(),
            &mut dst,
            Rect::from_ltwh(0, 0, 10, 10),
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
    fn copy_rect_clamped_destination_side_clip() {
        // Same reproduction as `copy_rect`'s https://github.com/crates-lurey-io/grixy/issues/15.
        #[rustfmt::skip]
        let src = NaiveGrid::<i32>::with_cells(3, 3, [
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        copy_rect_clamped(
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

    #[test]
    fn copy_rect_clamped_both_sides_clip_takes_the_tighter_bound() {
        #[rustfmt::skip]
        let src = NaiveGrid::<i32>::with_cells(4, 4, [
             1,  2,  3,  4,
             5,  6,  7,  8,
             9, 10, 11, 12,
            13, 14, 15, 16,
        ]);

        let mut dst = NaiveGrid::<i32>::new(6, 6);
        // Source-side clamp narrows `from` from 10x10 to 4x4 (src's real size); destination-side
        // clamp then narrows it further to 3x3 (dst only has 3 columns/rows left from `to`). The
        // final copy must use the tighter (3x3) bound on *both* sides, taking the top-left 3x3 of
        // the already source-clamped region - not the top-left 3x3 of the original 10x10 `from`.
        copy_rect_clamped(
            &src.copied(),
            &mut dst,
            Rect::from_ltwh(0, 0, 10, 10),
            Pos::new(3, 3),
        );

        #[rustfmt::skip]
        assert_eq!(dst.into_iter().collect::<Vec<_>>(),
        &[
            0, 0, 0,  0,  0,  0,
            0, 0, 0,  0,  0,  0,
            0, 0, 0,  0,  0,  0,
            0, 0, 0,  1,  2,  3,
            0, 0, 0,  5,  6,  7,
            0, 0, 0,  9, 10, 11,
        ]);
    }

    #[test]
    fn copy_rect_clamped_source_completely_out_of_bounds() {
        #[rustfmt::skip]
        let src = NaiveGrid::<i32>::with_cells(3, 3, [
            1, 1, 1,
            1, 1, 1,
            1, 1, 1,
        ]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        copy_rect_clamped(
            &src.copied(),
            &mut dst,
            Rect::from_ltwh(5, 5, 2, 2),
            Pos::new(0, 0),
        );

        assert!(dst.into_iter().all(|cell| cell == 0));
    }

    #[test]
    fn copy_rect_clamped_destination_completely_out_of_bounds() {
        #[rustfmt::skip]
        let src = NaiveGrid::<i32>::with_cells(3, 3, [
            1, 1, 1,
            1, 1, 1,
            1, 1, 1,
        ]);

        let mut dst = NaiveGrid::<i32>::new(5, 5);
        copy_rect_clamped(
            &src.copied(),
            &mut dst,
            Rect::from_ltwh(0, 0, 3, 3),
            Pos::new(10, 10),
        );

        assert!(dst.into_iter().all(|cell| cell == 0));
    }
}
