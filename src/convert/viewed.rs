use crate::{
    core::{Pos, Rect},
    ops::{GridRead, unchecked::TrustedSizeGrid},
};

/// Views a sub-grid, allowing access to a specific rectangular area of the grid.
///
/// See [`GridConvertExt::view`][] for usage.
///
/// [`GridConvertExt::view`]: crate::convert::GridConvertExt::view
pub struct Viewed<G> {
    pub(super) source: G,
    pub(super) bounds: Rect,
}

impl<G> GridRead for Viewed<G>
where
    G: GridRead,
{
    type Element<'b>
        = G::Element<'b>
    where
        Self: 'b;

    type Layout = G::Layout;

    fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
        let pos = pos - self.bounds.top_left();
        if !self.bounds.contains_pos(pos) {
            return None;
        }
        self.source.get(pos)
    }

    fn iter_rect(&self, bounds: Rect) -> impl Iterator<Item = Self::Element<'_>> {
        let bounds = bounds - self.bounds.top_left();
        self.source.iter_rect(bounds)
    }
}

unsafe impl<G> TrustedSizeGrid for Viewed<G>
where
    G: TrustedSizeGrid,
{
    fn width(&self) -> usize {
        self.bounds.width()
    }

    fn height(&self) -> usize {
        self.bounds.height()
    }
}
