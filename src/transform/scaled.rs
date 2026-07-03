use core::ops::Index;

use crate::{
    core::{Pos, Size},
    ops::{ExactSizeGrid, GridBase, GridRead},
};

/// Scales the grid elements using a nearest-neighbor approach.
///
/// See [`GridConvertExt::scale`][] for usage.
///
/// [`GridConvertExt::scale`]: crate::transform::GridConvertExt::scale
pub struct Scaled<G> {
    pub(super) source: G,
    pub(super) scale: usize,
}

impl<G> GridBase for Scaled<G>
where
    G: GridBase,
{
    fn size_hint(&self) -> (Size, Option<Size>) {
        let (lo, hi) = self.source.size_hint();
        (lo * self.scale, hi.map(|s| s * self.scale))
    }
}

impl<G> ExactSizeGrid for Scaled<G>
where
    G: ExactSizeGrid,
{
    fn width(&self) -> usize {
        self.source.width() * self.scale
    }

    fn height(&self) -> usize {
        self.source.height() * self.scale
    }
}

impl<G> GridRead for Scaled<G>
where
    G: GridRead,
{
    type Element<'b>
        = G::Element<'b>
    where
        Self: 'b;

    /// The layout of the grid.
    type Layout = G::Layout;

    fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
        self.source.get(pos / self.scale)
    }
}

/// Indexes into the scaled grid by position, forwarding to the source grid.
///
/// # Panics
///
/// Panics if `pos` is out of bounds of the scaled grid. Use [`GridRead::get`] for a
/// non-panicking alternative.
impl<G, T> Index<Pos> for Scaled<G>
where
    G: for<'a> GridRead<Element<'a> = &'a T> + 'static,
{
    type Output = T;

    fn index(&self, pos: Pos) -> &T {
        self.get(pos).expect("position out of bounds")
    }
}
