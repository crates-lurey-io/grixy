use crate::{core::Pos, ops::GridRead};

/// A grid reader that copies elements from another grid that returns copyable references.
pub struct GridCopied<'a, T, G>
where
    T: Copy,
    T: 'a,
    G: GridRead<Element<'a> = &'a T>,
{
    pub(super) source: &'a G,
}

impl<'a, T, G> GridRead for GridCopied<'a, T, G>
where
    T: 'a + Copy,
    G: GridRead<Element<'a> = &'a T>,
{
    type Element<'b>
        = T
    where
        Self: 'b;

    fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
        self.source.get(pos).copied()
    }

    fn iter_rect(&self, bounds: crate::prelude::Rect) -> impl Iterator<Item = Self::Element<'_>> {
        self.source.iter_rect(bounds).copied()
    }
}
