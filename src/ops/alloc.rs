use crate::{
    core::{Pos, Rect},
    ops::GridRead,
};

#[cfg(not(feature = "alloc"))]
compile_error!("The `alloc` feature must be enabled to use this module.");

extern crate alloc;

macro_rules! impl_grid_read {
    ($rc:ident) => {
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
