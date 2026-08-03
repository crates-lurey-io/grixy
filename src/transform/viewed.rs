use crate::{
    core::{Pos, Rect, Size},
    ops::{ExactSizeGrid, GridBase, GridRead},
};

/// Views a sub-grid, allowing access to a specific rectangular area of the grid.
///
/// See [`GridConvertExt::view`][] for usage.
///
/// [`GridConvertExt::view`]: crate::transform::GridConvertExt::view
pub struct Viewed<G> {
    pub(super) source: G,
    pub(super) bounds: Rect,
}

impl<G> GridBase for Viewed<G>
where
    G: GridBase,
{
    fn size_hint(&self) -> (Size, Option<Size>) {
        let size = Size::new(self.bounds.width(), self.bounds.height());
        (size, Some(size))
    }
}

impl<G> ExactSizeGrid for Viewed<G>
where
    G: ExactSizeGrid,
{
    fn width(&self) -> usize {
        self.bounds.width()
    }

    fn height(&self) -> usize {
        self.bounds.height()
    }
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
        // `pos` is in view-local coordinates (0..width, 0..height); check it against the view's
        // own (zero-based) size *before* translating, then add `bounds.top_left()` to reach the
        // source's coordinate space. Checking the already-translated `pos` against `self.bounds`
        // (which is itself in source coordinates) double-offsets, and for a `pos` smaller than
        // `bounds.top_left()` underflows.
        if pos.x >= self.bounds.width() || pos.y >= self.bounds.height() {
            return None;
        }
        self.source.get(pos + self.bounds.top_left())
    }

    fn iter_rect(&self, bounds: Rect) -> impl Iterator<Item = Self::Element<'_>> {
        // Clamp the (view-local) query to the view's own size first, so an oversized `bounds`
        // can't leak cells from outside the view, then translate into source coordinates.
        let bounds = self.trim_rect(bounds) + self.bounds.top_left();
        self.source.iter_rect(bounds)
    }
}
