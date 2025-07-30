use crate::{
    core::{HasSize, Pos, Rect},
    ops::unchecked::{GridReadUnchecked, GridWriteUnchecked},
};

/// Copies a rectangular region from a source grid to a destination grid.
///
/// This function behaves similarly to [`copy_rect`], but bounds checking is skipped for
/// performance reasons.
///
/// [`copy_rect`]: crate::ops::copy_rect
///
/// ## Safety
///
/// The caller must ensure that the source and destination rectangles are valid and that the
/// source grid contains enough elements to fill the destination rectangle. If the rectangles are
/// not the same size, the caller must ensure that the source rectangle is large enough to fill
/// the destination rectangle.
pub unsafe fn copy_rect_unchecked<E: Clone>(
    src: &impl GridReadUnchecked<Element = E>,
    src_rect: Rect,
    dst: &mut impl GridWriteUnchecked<Element = E>,
    dst_rect: Rect,
) {
    // Optimization: If the source and destination rectangles are the same, we can just copy directly.
    if src_rect.size() == dst_rect.size() {
        unsafe {
            dst.fill_rect_iter_unchecked(dst_rect, src.iter_rect_unchecked(src_rect).cloned());
        }
    } else {
        for y in 0..src_rect.height() {
            for x in 0..src_rect.width() {
                let src_pos = src_rect.top_left() + Pos::new(x, y);
                let dst_pos = dst_rect.top_left() + Pos::new(x, y);
                unsafe {
                    let value = src.get_unchecked(src_pos);
                    dst.set_unchecked(dst_pos, value.clone());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::{
        buf::VecGrid,
        core::{Rect, RowMajor},
        ops::GridRead,
    };
    use alloc::{vec, vec::Vec};

    #[test]
    fn copy_rect_unchecked_same_size() {
        #[rustfmt::skip]
        let src = VecGrid::with_buffer_row_major(vec![
            1, 2, 3,
            4, 5, 6,
            7, 8, 9],
        3, 3).unwrap();

        let mut dst = VecGrid::<i32, RowMajor>::new(3, 3);

        unsafe {
            copy_rect_unchecked(
                &src,
                Rect::from_ltwh(0, 0, 3, 3),
                &mut dst,
                Rect::from_ltwh(0, 0, 3, 3),
            );
        }

        #[rustfmt::skip]
        assert_eq!(
            dst.iter_rect(Rect::from_ltwh(0, 0, 3, 3))
                .copied()
                .collect::<Vec<_>>(),
            vec![
                1, 2, 3,
                4, 5, 6,
                7, 8, 9
            ]
        );
    }
}
