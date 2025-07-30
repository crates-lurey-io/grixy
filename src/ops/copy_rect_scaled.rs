use crate::{
    core::{HasSize, Pos, Rect},
    ops::{GridRead, GridWrite, copy_rect},
};

/// Copies and scales a rectangular region from a source grid to a destination grid.
///
/// This function behaves similarly to `copy_rect`, but it allows for scaling the source rectangle
/// to fit the destination rectangle. If the source rectangle is smaller than the destination,
/// the source elements are stretched to fill the destination rectangle, with an aspect ratio
/// maintained.
///
/// For example, if the source rectangle is 2x2 and the destination rectangle is 3x3, the source
/// rectangle will be doubled in size to fill the destination rectangle, but truncated to fit the
/// destination's dimensions.
///
/// ## Examples
///
/// ```rust
/// use grixy::{core::{Pos, Rect}, buf::VecGrid, ops::{copy_rect_scaled, GridRead, GridWrite}};
///
/// let src = VecGrid::new_filled_row_major(3, 3, 1);
/// let mut dst = VecGrid::new_row_major(4, 4);
///
/// copy_rect_scaled(
///    &src,
///    Rect::from_ltwh(0, 0, 2, 2),
///    &mut dst,
///    Rect::from_ltwh(0, 0, 4, 4),
/// );
///
/// let actual = dst.iter_rect(Rect::from_ltwh(0, 0, 4, 4))
///    .copied()
///    .collect::<Vec<_>>();
///
/// assert_eq!(actual, vec![
///   1, 1, 1, 1,
///   1, 1, 1, 1,
///   1, 1, 1, 1,
///   1, 1, 1, 1,
/// ]);
/// ```
pub fn copy_rect_scaled<E: Clone>(
    src: &impl GridRead<Element = E>,
    src_rect: Rect,
    dst: &mut impl GridWrite<Element = E>,
    dst_rect: Rect,
) {
    // Optimize: If the sizes are the same, we can just copy directly.
    if src_rect.size() == dst_rect.size() {
        copy_rect(src, src_rect, dst, dst_rect);
        return;
    }

    // Iterate over each pixel of the destination rectangle
    for y in 0..dst_rect.height() {
        for x in 0..dst_rect.width() {
            // Map the destination coordinate back to the source coordinate.
            // Multiplying *before* dividing is the key to preserving precision with integer math.
            let src_x = x * src_rect.width() / dst_rect.width();
            let src_y = y * src_rect.height() / dst_rect.height();

            // Calculate absolute positions
            let src_pos = src_rect.top_left() + Pos::new(src_x, src_y);
            let dst_pos = dst_rect.top_left() + Pos::new(x, y);

            // Get the source pixel and set it in the destination
            if let Some(value) = src.get(src_pos) {
                let _ = dst.set(dst_pos, value.clone());
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
    };
    use alloc::{vec, vec::Vec};

    #[test]
    fn copy_rect_scaled_2x2_to_4x4() {
        #[rustfmt::skip]
        let src = VecGrid::with_buffer_row_major(vec![
          1, 2, 3,
          4, 5, 6,
          7, 8, 9],
        3, 3).unwrap();

        let mut dst = VecGrid::<i32, RowMajor>::new(4, 4);
        copy_rect_scaled(
            &src,
            Rect::from_ltwh(0, 0, 2, 2),
            &mut dst,
            Rect::from_ltwh(0, 0, 4, 4),
        );

        #[rustfmt::skip]
        assert_eq!(
            dst.iter_rect(Rect::from_ltwh(0, 0, 4, 4))
                .copied()
                .collect::<Vec<_>>(),
            vec![
              1, 1, 2, 2,
              1, 1, 2, 2,
              4, 4, 5, 5,
              4, 4, 5, 5
            ]
        );
    }

    #[test]
    fn copy_rect_scaled_2x2_to_3x3() {
        #[rustfmt::skip]
        let src = VecGrid::with_buffer_row_major(vec![
          1, 2, 3,
          4, 5, 6,
          7, 8, 9],
        3, 3).unwrap();

        let mut dst = VecGrid::<i32, RowMajor>::new(3, 3);
        copy_rect_scaled(
            &src,
            Rect::from_ltwh(0, 0, 2, 2),
            &mut dst,
            Rect::from_ltwh(0, 0, 3, 3),
        );

        #[rustfmt::skip]
        assert_eq!(
            dst.iter_rect(Rect::from_ltwh(0, 0, 3, 3))
                .copied()
                .collect::<Vec<_>>(),
            vec![
              1, 1, 2,
              1, 1, 2,
              4, 4, 5
            ]
        );
    }

    #[test]
    fn copy_rect_scaled_same_size_opt() {
        #[rustfmt::skip]
        let src = VecGrid::with_buffer_row_major(vec![
          1, 2, 3,
          4, 5, 6,
          7, 8, 9],
        3, 3).unwrap();

        let mut dst = VecGrid::<i32, RowMajor>::new(3, 3);
        copy_rect_scaled(
            &src,
            Rect::from_ltwh(0, 0, 3, 3),
            &mut dst,
            Rect::from_ltwh(0, 0, 3, 3),
        );

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
