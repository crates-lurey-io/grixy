use crate::{
    core::{Pos, Rect},
    ops::{GridRead, GridWrite},
};

/// Blends a rectangular region from a source grid to a destination grid.
///
/// This function copies elements from the source rectangle to the destination rectangle, blending
/// the source and destination elements using a provided blending function. If the source rectangle
/// is smaller than the destination rectangle, it only copies the overlapping area.
///
/// See [`blit_rect_scaled`] for an alternative that scales the source rectangle if necessary.
///
/// [`blit_rect_scaled`]: crate::ops::blit_rect_scaled
///
/// ## Examples
///
/// ```rust
/// use grixy::{core::{Pos, Rect}, buf::VecGrid, ops::{blit_rect, GridRead, GridWrite}};
///
/// let src = VecGrid::new_filled_row_major(3, 3, 1);
/// let mut dst = VecGrid::new_filled_row_major(4, 4, 10);
///
/// blit_rect(
///     &src,
///     Rect::from_ltwh(0, 0, 3, 3),
///     &mut dst,
///     Rect::from_ltwh(0, 0, 4, 4),
///     &|src, dst| src + dst,
/// );
///
/// let actual = dst.iter_rect(Rect::from_ltwh(0, 0, 4, 4))
///    .copied()
///    .collect::<Vec<_>>();
///
/// assert_eq!(actual, vec![
///   11, 11, 11, 10,
///   11, 11, 11, 10,
///   11, 11, 11, 10,
///   10, 10, 10, 10,
/// ]);
/// ```
pub fn blit_rect<S: Clone, D>(
    src: &impl GridRead<Element = S>,
    src_rect: Rect,
    dst: &mut (impl GridWrite<Element = D> + GridRead<Element = D>),
    dst_rect: Rect,
    blend: &impl Fn(&S, &D) -> D,
) {
    for y in 0..src_rect.height() {
        for x in 0..src_rect.width() {
            let src_pos = src_rect.top_left() + Pos::new(x, y);
            let dst_pos = dst_rect.top_left() + Pos::new(x, y);
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
    fn blit_rect_test() {
        #[rustfmt::skip]
        let src = VecGrid::with_buffer_row_major(vec![
          1, 2, 3,
          4, 5, 6,
          7, 8, 9],
        3, 3).unwrap();

        let blend = AddBlend;
        let mut dst = VecGrid::<i32, RowMajor>::new_filled(3, 3, 10);

        blit_rect(
            &src,
            Rect::from_ltwh(0, 0, 2, 2),
            &mut dst,
            Rect::from_ltwh(0, 0, 3, 3),
            &|src, dst| blend.blend(*src, *dst),
        );

        #[rustfmt::skip]
        assert_eq!(
            dst.iter_rect(Rect::from_ltwh(0, 0, 3, 3))
                .copied()
                .collect::<Vec<_>>(),
            vec![
              11, 12, 10,
              14, 15, 10,
              10, 10, 10
            ]
        );
    }
}
