use crate::{
    core::{GridError, Pos, Rect, Size},
    ops::{ExactSizeGrid, GridBase, GridRead, GridWrite},
};

/// Blends write operations to a grid.
///
/// See [`GridConvertExt::blend`][] for usage.
///
/// [`GridConvertExt::blend`]: crate::transform::GridConvertExt::blend
pub struct Blended<'a, G, F> {
    pub(super) source: &'a mut G,
    pub(super) blend_fn: F,
}

impl<G, F> GridBase for Blended<'_, G, F>
where
    G: GridBase,
{
    fn size_hint(&self) -> (Size, Option<Size>) {
        self.source.size_hint()
    }
}

impl<G, F> ExactSizeGrid for Blended<'_, G, F>
where
    G: ExactSizeGrid,
{
    fn width(&self) -> usize {
        self.source.width()
    }

    fn height(&self) -> usize {
        self.source.height()
    }
}

impl<G, F> GridWrite for Blended<'_, G, F>
where
    G: GridRead + GridWrite,
    F: Fn(<G as GridRead>::Element<'_>, <G as GridWrite>::Element) -> <G as GridWrite>::Element,
    <G as GridWrite>::Element: Copy,
{
    type Element = <G as GridWrite>::Element;
    type Layout = <G as GridWrite>::Layout;

    fn set(&mut self, pos: Pos, value: Self::Element) -> Result<(), GridError> {
        let current = self.source.get(pos).ok_or(GridError)?;
        self.source.set(pos, (self.blend_fn)(current, value))
    }
}

impl<G, F> GridRead for Blended<'_, G, F>
where
    G: GridRead,
{
    type Element<'b>
        = <G as GridRead>::Element<'b>
    where
        Self: 'b;

    type Layout = <G as GridRead>::Layout;

    fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
        self.source.get(pos)
    }

    fn iter_rect(&self, bounds: Rect) -> impl Iterator<Item = Self::Element<'_>> {
        self.source.iter_rect(bounds)
    }
}
