use crate::{
    buf::GridBuf,
    core::{Layout, Pos},
    grid::{BoundedGrid, GridBase, GridReadUnchecked, GridWriteUnchecked},
};

impl<T, B, L> GridBase for GridBuf<T, B, L>
where
    L: Layout,
{
    type Element = T;
}

unsafe impl<T, B, L> BoundedGrid for GridBuf<T, B, L>
where
    L: Layout,
{
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<T, B, L> GridReadUnchecked for GridBuf<T, B, L>
where
    B: AsRef<[T]>,
    L: Layout,
{
    unsafe fn get_unchecked(&self, pos: Pos) -> &T {
        let index = L::to_1d(pos, self.width).index;
        unsafe { self.buffer.as_ref().get_unchecked(index) }
    }

    unsafe fn rect_iter_unchecked(
        &self,
        bounds: crate::core::Rect,
    ) -> impl Iterator<Item = &Self::Element> {
        let slice = self.buffer.as_ref();
        let width = self.width;
        (bounds.top()..bounds.bottom()).flat_map(move |y| {
            let row_start = L::to_1d(Pos::new(bounds.left(), y), width).index;
            slice[row_start..row_start + bounds.width()].iter()
        })
    }
}

impl<T, B, L> GridWriteUnchecked for GridBuf<T, B, L>
where
    B: AsMut<[T]>,
    L: Layout,
{
    unsafe fn set_unchecked(&mut self, pos: Pos, value: T) {
        let index = L::to_1d(pos, self.width).index;
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
            let x_xtart = L::to_1d(Pos::new(bounds.left(), y), width).index;
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
            let x_start = L::to_1d(Pos::new(bounds.left(), y), width).index;
            let x_end = x_start + bounds.width();
            slice[x_start..x_end].fill(value);
        }
    }
}
