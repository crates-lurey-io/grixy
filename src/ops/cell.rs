use crate::{
    core::{Pos, Rect},
    ops::GridWrite,
};

macro_rules! impl_grid_write {
    ($cell:ident<$t:ident>) => {
        impl<T> GridWrite for $cell<T>
        where
            T: GridWrite,
        {
            type Element = T::Element;
            type Layout = T::Layout;

            fn set(
                &mut self,
                pos: Pos,
                value: Self::Element,
            ) -> Result<(), crate::core::GridError> {
                self.get_mut().set(pos, value)
            }

            fn fill_rect(&mut self, bounds: Rect, f: impl FnMut(Pos) -> Self::Element) {
                self.get_mut().fill_rect(bounds, f);
            }

            fn fill_rect_iter(&mut self, dst: Rect, iter: impl IntoIterator<Item = Self::Element>) {
                self.get_mut().fill_rect_iter(dst, iter);
            }

            fn fill_rect_solid(&mut self, dst: Rect, value: Self::Element)
            where
                Self::Element: Copy,
            {
                self.get_mut().fill_rect_solid(dst, value);
            }
        }
    };
}

use core::cell::{Cell, RefCell, UnsafeCell};

impl_grid_write!(Cell<T>);
impl_grid_write!(RefCell<T>);
impl_grid_write!(UnsafeCell<T>);
