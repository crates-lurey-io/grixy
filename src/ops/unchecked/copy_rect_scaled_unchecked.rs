use crate::{
    core::{Pos, Rect},
    ops::unchecked::{GridReadUnchecked, GridWriteUnchecked},
};

/// Copies and scales a rectangular region from a source grid to a destination grid.
///
/// This function behaves similarly to `copy_rect_scaled`, but bounds checking is skipped for
/// performance reasons.
///
/// ## Safety
///
/// The caller must ensure that the source and destination rectangles are valid and that the
/// source grid contains enough elements to fill the destination rectangle. If the rectangles are
/// not the same size, the caller must ensure that the source rectangle is large enough to fill
/// the destination rectangle.
pub unsafe fn copy_rect_scaled_unchecked<E: Clone>(
    src: &impl GridReadUnchecked<Element = E>,
    src_rect: Rect,
    dst: &mut (impl GridWriteUnchecked<Element = E> + GridReadUnchecked<Element = E>),
    dst_rect: Rect,
) {
    // Iterate over each pixel of the destination rectangle
    for y in 0..dst_rect.height() {
        for x in 0..dst_rect.width() {
            // Map the destination coordinate back to the source coordinate.
            let src_x = x * src_rect.width() / dst_rect.width();
            let src_y = y * src_rect.height() / dst_rect.height();

            // Calculate absolute positions
            let src_pos = src_rect.top_left() + Pos::new(src_x, src_y);
            let dst_pos = dst_rect.top_left() + Pos::new(x, y);

            // Get the source pixel and set it in the destination
            let src_value = unsafe { src.get_unchecked(src_pos) };
            unsafe { dst.set_unchecked(dst_pos, src_value.clone()) };
        }
    }
}
