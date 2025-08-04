use crate::{
    core::Pos,
    ops::{GridRead, unchecked::TrustedSizeGrid},
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

    fn iter_rect(&self, bounds: crate::prelude::Rect) -> impl Iterator<Item = Self::Element<'_>> {
        core::iter::empty()
    }
}

unsafe impl<G> TrustedSizeGrid for Scaled<G>
where
    G: TrustedSizeGrid,
{
    fn width(&self) -> usize {
        self.source.width() * self.scale
    }

    fn height(&self) -> usize {
        self.source.height() * self.scale
    }
}
