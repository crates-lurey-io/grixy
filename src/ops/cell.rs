use crate::{
    core::{Pos, Rect},
    ops::{GridBase, GridWrite},
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

impl<T> GridBase for Cell<T>
where
    T: GridBase,
{
    fn size_hint(&self) -> (crate::core::Size, Option<crate::core::Size>) {
        // SAFETY: `size_hint` does not mutate the inner value, so it's safe to
        // get a shared reference from the `UnsafeCell` within `Cell`.
        unsafe { (*self.as_ptr()).size_hint() }
    }
}

impl<T> GridBase for RefCell<T>
where
    T: GridBase,
{
    fn size_hint(&self) -> (crate::core::Size, Option<crate::core::Size>) {
        self.borrow().size_hint()
    }
}

impl<T> GridBase for UnsafeCell<T>
where
    T: GridBase,
{
    fn size_hint(&self) -> (crate::core::Size, Option<crate::core::Size>) {
        // SAFETY: `size_hint` does not mutate the inner value, so it's safe to
        // get a shared reference from the `UnsafeCell`.
        unsafe { (*self.get()).size_hint() }
    }
}

impl_grid_write!(Cell<T>);
impl_grid_write!(RefCell<T>);
impl_grid_write!(UnsafeCell<T>);

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test::NaiveGrid;

    fn test_grid_write<'a>(grid: &mut (impl GridWrite<Element = u8> + 'a)) {
        grid.set(Pos::new(1, 1), 42).unwrap();
        grid.fill_rect(Rect::from_ltwh(0, 0, 3, 3), |_| 0);
        grid.fill_rect_iter(Rect::from_ltwh(0, 0, 3, 3), [1, 2, 3]);
        grid.fill_rect_solid(Rect::from_ltwh(0, 0, 3, 3), 99);
    }

    // #[test]
    // fn test_cell_grid_write() {
    //     let mut grid = Cell::new(NaiveGrid::new(3, 3));
    //     test_grid_write(&mut grid);
    // }

    #[test]
    fn test_refcell_grid_write() {
        let mut grid = RefCell::new(NaiveGrid::new(3, 3));
        test_grid_write(&mut grid);
    }

    #[test]
    fn test_unsafecell_grid_write() {
        let mut grid = UnsafeCell::new(NaiveGrid::new(3, 3));
        test_grid_write(&mut grid);
    }
}
