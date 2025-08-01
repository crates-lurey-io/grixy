//! Grid operations for converting the representation of a grid.
//!
//! These types are provided as part of the public API, but all usage is through [`GridRead`].

use crate::{
    core::{Pos, Rect},
    ops::GridRead,
};

/// Copies elements from another grid that returns copyable references.
///
/// See [`GridRead::copied`][crate::ops::GridRead::copied] for usage.
pub struct Copied<'a, T, G>
where
    T: Copy,
    T: 'a,
    G: GridRead<Element<'a> = &'a T>,
{
    pub(super) source: &'a G,
}

impl<'a, T, G> GridRead for Copied<'a, T, G>
where
    T: 'a + Copy,
    G: GridRead<Element<'a> = &'a T>,
{
    type Element<'b>
        = T
    where
        Self: 'b;

    type Layout = G::Layout;

    fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
        self.source.get(pos).copied()
    }

    fn iter_rect(&self, bounds: crate::prelude::Rect) -> impl Iterator<Item = Self::Element<'_>> {
        self.source.iter_rect(bounds).copied()
    }
}

/// Transforms elements.
///
/// See [`GridRead::mapped`][crate::ops::GridRead::mapped] for usage.
pub struct Mapped<'a, S, F, G, T = S>
where
    S: 'a,
    T: 'a,
    F: Fn(S) -> T,
    G: GridRead<Element<'a> = S>,
{
    pub(super) source: &'a G,
    pub(super) map_fn: F,
}

impl<'a, S, F, G, T> GridRead for Mapped<'a, S, F, G, T>
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

    type Layout = G::Layout;

    fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
        self.source.get(pos).map(&self.map_fn)
    }

    fn iter_rect(&self, bounds: crate::prelude::Rect) -> impl Iterator<Item = Self::Element<'_>> {
        self.source.iter_rect(bounds).map(&self.map_fn)
    }
}

/// Views a sub-grid, allowing access to a specific rectangular area of the grid.
///
/// See [`GridRead::view`][crate::ops::GridRead::view] for usage.
pub struct Viewed<'a, G>
where
    G: GridRead,
{
    pub(super) source: &'a G,
    pub(super) bounds: Rect,
}

impl<G> GridRead for Viewed<'_, G>
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

/// Scales the grid elements using a nearest-neighbor approach.
///
/// See [`GridRead::scale`][crate::ops::GridRead::scale] for usage.
pub struct Scaled<'a, G>
where
    G: GridRead,
{
    pub(super) source: &'a G,
    pub(super) scale: usize,
}

impl<G> GridRead for Scaled<'_, G>
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

    #[test]
    fn grid_view_get() {
        let grid = VecGrid::new_filled_row_major(3, 3, 1);
        let view = grid.view(Rect::from_ltwh(0, 0, 2, 2));
        assert_eq!(view.get(Pos::new(1, 1)), Some(&1));
        assert_eq!(view.get(Pos::new(2, 2)), None);
    }

    #[test]
    fn grid_view_iter_rect() {
        let grid = VecGrid::new_filled_row_major(3, 3, 1);
        let view = grid.view(Rect::from_ltwh(0, 0, 2, 2));
        let elements: Vec<_> = view.iter_rect(Rect::from_ltwh(0, 0, 2, 2)).collect();
        assert_eq!(elements, &[&1, &1, &1, &1]);
    }

    #[test]
    fn grid_scaled_get() {
        let grid = VecGrid::with_buffer_row_major(2, 2, vec![1, 2, 3, 4]).unwrap();
        let scaled = grid.scale(2);
        assert_eq!(scaled.get(Pos::new(1, 1)), Some(&1));
        assert_eq!(scaled.get(Pos::new(2, 2)), Some(&4));
        assert_eq!(scaled.get(Pos::new(3, 3)), Some(&4));
        assert_eq!(scaled.get(Pos::new(4, 4)), None);
    }

    #[test]
    fn grid_scaled_iter_rect() {
        let grid = VecGrid::with_buffer_row_major(2, 2, vec![1, 2, 3, 4]).unwrap();
        let scaled = grid.scale(2);
        let elements: Vec<_> = scaled.iter_rect(Rect::from_ltwh(0, 0, 4, 4)).collect();

        #[rustfmt::skip]
        assert_eq!(elements, &[
            &1, &1, &2, &2,
            &1, &1, &2, &2,
            &3, &3, &4, &4,
            &3, &3, &4, &4,
        ]);
    }
}
