use core::ops::Index;

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
    fn size_hint(&self) -> (Size, Option<crate::core::Size>) {
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

/// Indexes into the view by position, forwarding to the source grid.
///
/// # Panics
///
/// Panics if `pos` is out of bounds of the view. Use [`GridRead::get`] for a non-panicking
/// alternative.
impl<G, T> Index<Pos> for Viewed<G>
where
    G: for<'a> GridRead<Element<'a> = &'a T> + 'static,
{
    type Output = T;

    fn index(&self, pos: Pos) -> &T {
        self.get(pos).expect("position out of bounds")
    }
}
