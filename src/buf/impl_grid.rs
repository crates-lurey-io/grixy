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
        let index = L::to_1d(pos, self.width);
        unsafe { self.buffer.as_ref().get_unchecked(index) }
    }

    unsafe fn iter_rect_unchecked(
        &self,
        bounds: crate::core::Rect,
    ) -> impl Iterator<Item = Self::Element<'_>> {
        if let Some(aligned) = L::slice_rect_aligned(self.as_ref(), self.size(), bounds) {
            internal::IterRect::Aligned(aligned.iter())
        } else {
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
        let index = L::to_1d(pos, self.width);
        unsafe { *self.buffer.as_mut().get_unchecked_mut(index) = value }
    }

    unsafe fn fill_rect_iter_unchecked(
        &mut self,
        bounds: crate::core::Rect,
        iter: impl IntoIterator<Item = Self::Element>,
    ) {
        let slice = self.buffer.as_mut();
        let width = self.width;
        let mut iter = iter.into_iter();
        for y in bounds.top()..bounds.bottom() {
            let x_xtart = L::to_1d(Pos::new(bounds.left(), y), width);
            let x_end = x_xtart + bounds.width();
            slice[x_xtart..x_end]
                .iter_mut()
                .zip(&mut iter)
                .for_each(|(cell, value)| *cell = value);
        }
    }

    unsafe fn fill_rect_solid_unchecked(&mut self, bounds: crate::core::Rect, value: Self::Element)
    where
        Self::Element: Copy,
    {
        let slice = self.buffer.as_mut();
        let width = self.width;
        for y in bounds.top()..bounds.bottom() {
            let x_start = L::to_1d(Pos::new(bounds.left(), y), width);
            let x_end = x_start + bounds.width();
            slice[x_start..x_end].fill(value);
        }
    }
}
