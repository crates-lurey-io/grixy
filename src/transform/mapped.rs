use core::marker::PhantomData;

use crate::{
    core::{Pos, Size},
    ops::{GridBase, GridRead},
};

/// Transforms elements.
///
/// See [`GridConvertExt::map`][] for usage.
///
/// [`GridConvertExt::map`]: crate::transform::GridConvertExt::map
pub struct Mapped<F, G, T> {
    pub(super) source: G,
    pub(super) map_fn: F,
    pub(super) _element: PhantomData<T>,
}

impl<F, G, T> GridBase for Mapped<F, G, T>
where
    G: GridBase,
{
    fn size_hint(&self) -> (Size, Option<Size>) {
        self.source.size_hint()
    }
}

impl<F, G, T> GridRead for Mapped<F, G, T>
where
    F: Fn(G::Element<'_>) -> T,
    G: GridRead,
{
    type Element<'b>
        = T
    where
        Self: 'b;

    type Layout = G::Layout;

    fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
        self.source.get(pos).map(&self.map_fn)
    }

    fn iter_rect(&self, bounds: crate::prelude::Rect) -> impl Iterator<Item = Self::Element<'_>> {
        self.source.iter_rect(bounds).map(&self.map_fn)
    }
}
