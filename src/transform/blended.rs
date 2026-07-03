use core::ops::Index;

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
        let current = self.source.get(pos).ok_or(GridError::OutOfBounds { pos })?;
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

/// Indexes into the underlying grid by position, forwarding to the source grid's read side.
///
/// # Panics
///
/// Panics if `pos` is out of bounds. Use [`GridRead::get`] for a non-panicking alternative.
impl<G, F, T> Index<Pos> for Blended<'_, G, F>
where
    G: for<'a> GridRead<Element<'a> = &'a T> + 'static,
{
    type Output = T;

    fn index(&self, pos: Pos) -> &T {
        self.get(pos).expect("position out of bounds")
    }
}
