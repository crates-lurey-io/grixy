use crate::{
    core::{Pos, Rect},
    ops::{GridBase, GridRead},
};

#[cfg(not(feature = "alloc"))]
compile_error!("The `alloc` feature must be enabled to use this module.");

extern crate alloc;

macro_rules! impl_grid_read {
    ($rc:ident) => {
        impl<T> GridBase for $rc<T>
        where
            T: GridBase,
        {
            fn size_hint(&self) -> (crate::core::Size, Option<crate::core::Size>) {
                self.as_ref().size_hint()
            }
        }

        impl<T> GridRead for $rc<T>
        where
            T: GridRead,
        {
            type Element<'a>
                = T::Element<'a>
            where
                T: 'a;
            type Layout = T::Layout;

            fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
                self.as_ref().get(pos)
            }

            fn iter_rect(&self, bounds: Rect) -> impl Iterator<Item = Self::Element<'_>> {
                self.as_ref().iter_rect(bounds)
            }
        }
    };
}

use alloc::{rc::Rc, sync::Arc};

impl_grid_read!(Arc);
impl_grid_read!(Rc);

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test::NaiveGrid;

    fn test_grid_read<'a>(grid: &'a impl GridRead<Element<'a> = &'a u8>) {
        assert_eq!(grid.get(Pos::new(1, 1)), Some(&0));
        assert_eq!(grid.iter_rect(Rect::from_ltwh(0, 0, 3, 3)).count(), 9);
    }

    #[test]
    fn test_arc_grid_read() {
        let grid = Arc::new(NaiveGrid::new(3, 3));
        test_grid_read(&grid);
    }

    #[test]
    fn test_rc_grid_read() {
        let grid = Rc::new(NaiveGrid::new(3, 3));
        test_grid_read(&grid);
    }
}
