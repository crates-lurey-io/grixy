use crate::{
    core::{Pos, Rect},
    ops::unchecked::{GridReadUnchecked, GridWriteUnchecked},
};

/// Blends a rectangular region from a source grid to a destination grid.
///
/// This function behaves similarly to [`blit_rect`], but bounds checking is skipped for
/// performance reasons.
///
/// [`blit_rect`]: crate::ops::blit_rect
///
/// ## Safety
///
/// The caller must ensure that the source and destination rectangles are valid and that the
/// source grid contains enough elements to fill the destination rectangle. If the rectangles are
/// not the same size, the caller must ensure that the source rectangle is large enough to fill
/// the destination rectangle.
pub unsafe fn blit_rect_unchecked<E: Clone>(
    src: &impl GridReadUnchecked<Element = E>,
    src_rect: Rect,
    dst: &mut (impl GridWriteUnchecked<Element = E> + GridReadUnchecked<Element = E>),
    dst_rect: Rect,
    blend: &impl Fn(&E, &E) -> E,
) {
    for y in 0..src_rect.height() {
        for x in 0..src_rect.width() {
            let src_pos = src_rect.top_left() + Pos::new(x, y);
            let dst_pos = dst_rect.top_left() + Pos::new(x, y);
            let src_value = unsafe { src.get_unchecked(src_pos) };
            let dst_value = unsafe { dst.get_unchecked(dst_pos) };
            let blended_value = blend(src_value, dst_value);
            unsafe { dst.set_unchecked(dst_pos, blended_value) };
        }
    }
}

#[cfg(test)]
mod tests {}
