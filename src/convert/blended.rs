use crate::{
    core::{GridError, Pos, Rect},
    ops::{GridRead, GridWrite, unchecked::TrustedSizeGrid},
};

/// Blends write operations to a grid.
///
/// See [`GridConvertExt::blend`][] for usage.
///
/// [`GridConvertExt::blend`]: crate::convert::GridConvertExt::blend
pub struct Blended<'a, G, F> {
    pub(super) source: &'a mut G,
    pub(super) blend_fn: F,
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

unsafe impl<G, F> TrustedSizeGrid for Blended<'_, G, F>
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
