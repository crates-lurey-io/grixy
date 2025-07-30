use crate::{
    core::{Pos, Rect},
    ops::{GridRead, GridWrite},
};

/// Blends and scales a rectangular region from a source grid to a destination grid.
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
/// use grixy::{core::{Pos, Rect}, buf::VecGrid, ops::{blit_rect_scaled, GridRead, GridWrite}};
///
/// let src = VecGrid::new_filled_row_major(3, 3, 1);
/// let mut dst = VecGrid::new_filled_row_major(4, 4, 10);
///
/// blit_rect_scaled(
///     &src,
///     Rect::from_ltwh(0, 0, 2, 2),
///     &mut dst,
///     Rect::from_ltwh(0, 0, 4, 4),    
///     &|src, dst| src + dst,
/// );
///
/// let actual = dst.iter_rect(Rect::from_ltwh(0, 0, 4, 4))
///     .copied()
///     .collect::<Vec<_>>();
///
/// assert_eq!(actual, vec![
///   11, 11, 11, 11,
///   11, 11, 11, 11,
///   11, 11, 11, 11,
///   11, 11, 11, 11,
/// ]);
/// ```
pub fn blit_rect_scaled<S: Clone, D>(
    src: &impl GridRead<Element = S>,
    src_rect: Rect,
    dst: &mut (impl GridWrite<Element = D> + GridRead<Element = D>),
    dst_rect: Rect,
    blend: &impl Fn(&S, &D) -> D,
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

            // Get the source pixel and blend it with the destination
            if let Some(src_value) = src.get(src_pos) {
                if let Some(dst_value) = dst.get(dst_pos) {
                    let blended_value = blend(src_value, dst_value);
                    let _ = dst.set(dst_pos, blended_value);
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
    };
    use alloc::{vec, vec::Vec};

    struct AddBlend;
    impl AddBlend {
        #[allow(clippy::unused_self)]
        fn blend(&self, src: i32, dst: i32) -> i32 {
            src + dst
        }
    }

    #[test]
    fn blit_rect_scaled_test() {
        #[rustfmt::skip]
        let src = VecGrid::with_buffer_row_major(vec![
          1, 2, 3,
          4, 5, 6,
          7, 8, 9],
        3, 3).unwrap();

        let blend = AddBlend;
        let mut dst = VecGrid::<i32, RowMajor>::new_filled(4, 4, 10);

        blit_rect_scaled(
            &src,
            Rect::from_ltwh(0, 0, 2, 2),
            &mut dst,
            Rect::from_ltwh(0, 0, 4, 4),
            &|src, dst| blend.blend(*src, *dst),
        );

        #[rustfmt::skip]
        assert_eq!(
            dst.iter_rect(Rect::from_ltwh(0, 0, 4, 4))
                .copied()
                .collect::<Vec<_>>(),
            vec![
              11, 11, 12, 12,
              11, 11, 12, 12,
              14, 14, 15, 15,
              14, 14, 15, 15
            ]
        );
    }
}
