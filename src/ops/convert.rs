use crate::{core::Pos, ops::GridRead};

/// A grid reader that copies elements from another grid that returns copyable references.
///
/// See [`GridRead::copied`][crate::ops::GridRead::copied] for usage.
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

/// A grid reader that transforms elements.
///
/// See [`GridRead::mapped`][crate::ops::GridRead::mapped] for usage.
pub struct GridMapped<'a, S, F, G, T = S>
where
    S: 'a,
    T: 'a,
    F: Fn(S) -> T,
    G: GridRead<Element<'a> = S>,
{
    pub(super) source: &'a G,
    pub(super) map_fn: F,
}

impl<'a, S, F, G, T> GridRead for GridMapped<'a, S, F, G, T>
where
    S: 'a,
    T: 'a,
    F: Fn(S) -> T,
    G: GridRead<Element<'a> = S>,
{
    type Element<'b>
        = T
    where
        Self: 'b;

    fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
        self.source.get(pos).map(&self.map_fn)
    }

    fn iter_rect(&self, bounds: crate::prelude::Rect) -> impl Iterator<Item = Self::Element<'_>> {
        self.source.iter_rect(bounds).map(&self.map_fn)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::{buf::VecGrid, core::Rect};
    use alloc::{vec, vec::Vec};

    use super::*;

    #[test]
    fn grid_copied_get() {
        let grid = VecGrid::new_filled_row_major(3, 3, 1);
        let copied = grid.copied();
        assert_eq!(copied.get(Pos::new(1, 1)), Some(1));
        assert_eq!(copied.get(Pos::new(3, 3)), None);
    }

    #[test]
    fn grid_copied_iter_rect() {
        let grid = VecGrid::new_filled_row_major(3, 3, 1);
        let copied = grid.copied();
        let elements: Vec<_> = copied.iter_rect(Rect::from_ltwh(0, 0, 2, 2)).collect();
        assert_eq!(elements, vec![1, 1, 1, 1]);
    }

    #[test]
    fn grid_mapped_get() {
        let grid = VecGrid::new_filled_row_major(3, 3, 1);
        let mapped = grid.map(|x| x * 2);
        assert_eq!(mapped.get(Pos::new(1, 1)), Some(2));
        assert_eq!(mapped.get(Pos::new(3, 3)), None);
    }

    #[test]
    fn grid_mapped_iter_rect() {
        let grid = VecGrid::new_filled_row_major(3, 3, 1);
        let mapped = grid.map(|x| x * 2);
        let elements: Vec<_> = mapped.iter_rect(Rect::from_ltwh(0, 0, 2, 2)).collect();
        assert_eq!(elements, vec![2, 2, 2, 2]);
    }
}
