use crate::{
    buf::GridBuf,
    core::{Pos, Size},
    internal,
    ops::{
        ExactSizeGrid, GridBase, layout,
        unchecked::{GridReadUnchecked, GridWriteUnchecked, TrustedSizeGrid},
    },
};

impl<T, B, L> GridBase for GridBuf<T, B, L>
where
    L: layout::Linear,
{
    fn size_hint(&self) -> (crate::prelude::Size, Option<crate::prelude::Size>) {
        let size = Size::new(self.width, self.height);
        (size, Some(size))
    }
}

impl<T, B, L> ExactSizeGrid for GridBuf<T, B, L>
where
    L: layout::Linear,
{
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

// SAFETY: `GridBuf` always reports its exact dimensions from `size_hint()` (see `GridBase` impl),
// and those dimensions match `ExactSizeGrid::width()`/`height()`. The buffer length is always
// `width * height` (enforced by `from_buffer` and constructors), so unchecked indexing into
// the buffer at `L::pos_to_index(pos, width)` for any pos within `(0..width, 0..height)` is safe.
unsafe impl<T, B, L> TrustedSizeGrid for GridBuf<T, B, L> where L: layout::Linear {}

impl<T, B, L> GridReadUnchecked for GridBuf<T, B, L>
where
    B: AsRef<[T]>,
    L: layout::Linear,
{
    type Element<'a>
        = &'a T
    where
        Self: 'a;

    type Layout = L;

    unsafe fn get_unchecked(&self, pos: Pos) -> Self::Element<'_> {
        let index = L::pos_to_index(pos, self.width);
        // SAFETY: The caller guarantees `pos` is in bounds, and `TrustedSizeGrid` guarantees
        // `index < self.buffer.len()`. The buffer is at least `width * height` elements long.
        unsafe { self.buffer.as_ref().get_unchecked(index) }
    }

    unsafe fn iter_rect_unchecked(
        &self,
        bounds: crate::core::Rect,
    ) -> impl Iterator<Item = Self::Element<'_>> {
        if let Some(aligned) = L::slice_rect_aligned(self.as_ref(), self.size(), bounds) {
            // SAFETY: `slice_rect_aligned` returns `None` when the bounds are not contiguous in
            // the layout's storage order. When it returns `Some`, the returned slice covers
            // exactly the positions in `bounds`. The caller guarantees every position is valid,
            // so the slice is within the allocated buffer.
            internal::IterRect::Aligned(aligned.iter())
        } else {
            // SAFETY: For non-contiguous rects, iterate position-by-position. Each
            // `self.get_unchecked(pos)` call is sound because the caller guarantees all
            // positions in `bounds` are valid.
            let iter = {
                let pos = L::iter_pos(bounds);
                pos.map(move |pos| unsafe { self.get_unchecked(pos) })
            };
            internal::IterRect::Unaligned(iter)
        }
    }
}

impl<T, B, L> GridWriteUnchecked for GridBuf<T, B, L>
where
    B: AsMut<[T]>,
    L: layout::Linear,
{
    type Element = T;
    type Layout = L;

    unsafe fn set_unchecked(&mut self, pos: Pos, value: Self::Element) {
        let index = L::pos_to_index(pos, self.width);
        // SAFETY: The caller guarantees `pos` is in bounds, and `TrustedSizeGrid` guarantees
        // `index < self.buffer.len()`. The buffer is at least `width * height` elements long.
        unsafe { *self.buffer.as_mut().get_unchecked_mut(index) = value }
    }

    unsafe fn fill_rect_iter_unchecked(
        &mut self,
        bounds: crate::core::Rect,
        iter: impl IntoIterator<Item = Self::Element>,
    ) {
        let size = self.size();
        if let Some(aligned) = L::slice_rect_aligned_mut(self.as_mut(), size, bounds) {
            // SAFETY: `slice_rect_aligned_mut` returns `None` when the bounds are not contiguous.
            // When it returns `Some`, the mutable slice covers exactly the positions in `bounds`.
            // The caller guarantees every position is valid, so the slice is within the buffer.
            aligned
                .iter_mut()
                .zip(iter)
                .for_each(|(cell, value)| *cell = value);
        } else {
            let mut iter = iter.into_iter();
            for pos in L::iter_pos(bounds) {
                if let Some(value) = iter.next() {
                    // SAFETY: The caller guarantees every position in `bounds` is valid.
                    unsafe { self.set_unchecked(pos, value) }
                } else {
                    break;
                }
            }
        }
    }

    unsafe fn fill_rect_solid_unchecked(&mut self, bounds: crate::core::Rect, value: Self::Element)
    where
        Self::Element: Copy,
    {
        let size = self.size();
        if let Some(aligned) = L::slice_rect_aligned_mut(self.as_mut(), size, bounds) {
            // SAFETY: `slice_rect_aligned_mut` returns `None` when the bounds are not contiguous.
            // When `Some`, the mutable slice covers exactly the positions in `bounds`.
            // `slice.fill(value)` is safe because the slice is within the allocated buffer.
            aligned.fill(value);
        } else {
            for pos in L::iter_pos(bounds) {
                // SAFETY: The caller guarantees every position in `bounds` is valid.
                unsafe { self.set_unchecked(pos, value) }
            }
        }
    }
}
