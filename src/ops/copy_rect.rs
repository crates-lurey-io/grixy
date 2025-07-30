use crate::{
    core::{HasSize, Pos, Rect},
    ops::{GridRead, GridWrite},
};

/// Copies a rectangular region from a source grid to a destination grid.
///
/// Given a source grid and a destination grid, along with their respective rectangles, this
/// function copies elements from the source rectangle to the destination rectangle. If the
/// rectangles are the same size, it performs a fast bulk copy. Otherwise, it copies elements one by
/// one, matching positions from the source to the destination.
///
/// If the source is smaller than the destination, the rest of the destination remains unchanged.
///
/// See [`copy_rect_scaled`] for an alternative that scales the source rectangle if necessary.
///
/// [`copy_rect_scaled`]: crate::ops::copy_rect_scaled
///
/// ## Examples
///
/// ```rust
/// use grixy::{core::{Pos, Rect}, buf::VecGrid, ops::{copy_rect, GridRead, GridWrite}};
///
/// let src = VecGrid::new_filled_row_major(3, 3, 1);
/// let mut dst = VecGrid::new_row_major(4, 4);
///
/// copy_rect(
///     &src,
///     Rect::from_ltwh(0, 0, 3, 3),
///     &mut dst,
///     Rect::from_ltwh(0, 0, 4, 4),
/// );
///
/// let actual = dst.iter_rect(Rect::from_ltwh(0, 0, 4, 4))
///     .copied()
///     .collect::<Vec<_>>();
///
/// assert_eq!(actual, vec![
///   1, 1, 1, 0,
///   1, 1, 1, 0,
///   1, 1, 1, 0,
///   0, 0, 0, 0,
/// ]);
/// ```
pub fn copy_rect<E: Clone>(
    src: &impl GridRead<Element = E>,
    src_rect: Rect,
    dst: &mut impl GridWrite<Element = E>,
    dst_rect: Rect,
) {
    // Optimization: If the source and destination rectangles are the same, we can just copy directly.
    if src_rect.size() == dst_rect.size() {
        dst.fill_rect_iter(dst_rect, src.iter_rect(src_rect).cloned());
    } else {
        for y in 0..src_rect.height() {
            for x in 0..src_rect.width() {
                let src_pos = src_rect.top_left() + Pos::new(x, y);
                let dst_pos = dst_rect.top_left() + Pos::new(x, y);
                if let Some(value) = src.get(src_pos) {
                    let _ = dst.set(dst_pos, value.clone());
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
        ops::copy_rect,
    };
    use alloc::{vec, vec::Vec};

    #[test]
    fn copy_rect_same_size_3x3() {
        #[rustfmt::skip]
        let src = VecGrid::with_buffer_row_major(vec![
          1, 2, 3, 
          4, 5, 6, 
          7, 8, 9], 
        3, 3).unwrap();

        let mut dst = VecGrid::<i32, RowMajor>::new(3, 3);

        copy_rect(
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

    #[test]
    fn copy_rect_same_size_subset_2x2() {
        #[rustfmt::skip]
        let src = VecGrid::with_buffer_row_major(vec![
          1, 2, 3, 
          4, 5, 6, 
          7, 8, 9], 
        3, 3).unwrap();

        let mut dst = VecGrid::<i32, RowMajor>::new(3, 3);

        copy_rect(
            &src,
            Rect::from_ltwh(0, 0, 2, 2),
            &mut dst,
            Rect::from_ltwh(0, 0, 2, 2),
        );

        #[rustfmt::skip]
        assert_eq!(
            dst.iter_rect(Rect::from_ltwh(0, 0, 3, 3))
                .copied()
                .collect::<Vec<_>>(),
            vec![
              1, 2, 0,
              4, 5, 0,
              0, 0, 0
            ]
        );
    }

    #[test]
    fn copy_rect_different_size() {
        #[rustfmt::skip]
        let src = VecGrid::with_buffer_row_major(vec![
          1, 2, 3,
          4, 5, 6,
          7, 8, 9],
        3, 3).unwrap();

        let mut dst = VecGrid::<i32, RowMajor>::new(3, 3);

        copy_rect(
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
              1, 2, 0,
              4, 5, 0,
              0, 0, 0
            ]
        );
    }
}
