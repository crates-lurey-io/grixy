use core::marker::PhantomData;

use crate::{
    core::{Pos, Size},
    ops::{GridBase, GridRead, unchecked::TrustedSizeGrid},
};

/// Copies elements from another grid that returns copyable references.
///
/// See [`GridConvertExt::copied`][] for usage.
///
/// [`GridConvertExt::copied`]: crate::transform::GridConvertExt::copied
pub struct Copied<T, G> {
    pub(super) source: G,
    pub(super) _element: PhantomData<T>,
}

impl<T, G> GridBase for Copied<T, G>
where
    G: GridBase,
{
    fn size_hint(&self) -> (Size, Option<Size>) {
        self.source.size_hint()
    }
}

impl<T, G> GridRead for Copied<T, G>
where
    T: Copy,
    for<'a> G: GridRead<Element<'a> = &'a T> + 'a,
{
    type Element<'b>
        = T
    where
        Self: 'b;

    type Layout = G::Layout;

    fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
        self.source.get(pos).copied()
    }

    fn iter_rect(&self, bounds: crate::prelude::Rect) -> impl Iterator<Item = Self::Element<'_>> {
        self.source.iter_rect(bounds).copied()
    }
}

unsafe impl<T, G> TrustedSizeGrid for Copied<T, G>
where
    G: TrustedSizeGrid,
{
    fn width(&self) -> usize {
        self.source.width()
    }

    fn height(&self) -> usize {
        self.source.height()
    }
}
