//! Grid operations for converting the representation of a grid.
//!
//! These types are provided as part of the public API, but all usage is through [`GridRead`].

use core::marker::PhantomData;

use crate::{
    core::{GridError, Pos, Rect},
    ops::{GridRead, GridWrite, unchecked::TrustedSizeGrid},
};

/// Copies elements from another grid that returns copyable references.
///
/// See [`GridRead::copied`] for usage.
pub struct Copied<T, G> {
    pub(super) source: G,
    pub(super) _element: PhantomData<T>,
}

impl<T, G> GridRead for Copied<T, G>
where
    T: Copy,
    for<'a> G: GridRead<Element<'a> = &'a T> + 'a,
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

unsafe impl<T, G> TrustedSizeGrid for Copied<T, G>
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

/// Transforms elements.
///
/// See [`GridRead::map`] for usage.
pub struct Mapped<F, G, T> {
    pub(super) source: G,
    pub(super) map_fn: F,
    pub(super) _element: PhantomData<T>,
}

impl<F, G, T> GridRead for Mapped<F, G, T>
where
    F: Fn(G::Element<'_>) -> T,
    G: GridRead,
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

unsafe impl<F, G, T> TrustedSizeGrid for Mapped<F, G, T>
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

/// Views a sub-grid, allowing access to a specific rectangular area of the grid.
///
/// See [`GridRead::view`] for usage.
pub struct Viewed<'a, G> {
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

unsafe impl<G> TrustedSizeGrid for Viewed<'_, G>
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

/// Scales the grid elements using a nearest-neighbor approach.
///
/// See [`GridRead::scale`] for usage.
pub struct Scaled<'a, G> {
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

unsafe impl<G> TrustedSizeGrid for Scaled<'_, G>
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

/// Blends write operations to a grid.
///
/// See [`GridWrite::blend`] for usage.
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

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::{buf::GridBuf, core::Rect};
    use alloc::{vec, vec::Vec};

    use super::*;

    #[test]
    fn grid_copied_size() {
        let grid = GridBuf::<u8, _, _>::new(10, 10).copied();
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 10);
    }

    #[test]
    fn grid_copied_get() {
        let grid = GridBuf::new_filled(3, 3, 1);
        let copied = grid.copied();
        assert_eq!(copied.get(Pos::new(1, 1)), Some(1));
        assert_eq!(copied.get(Pos::new(3, 3)), None);
    }

    #[test]
    fn grid_copied_iter_rect() {
        let grid = GridBuf::new_filled(3, 3, 1);
        let copied = grid.copied();
        let elements: Vec<_> = copied.iter_rect(Rect::from_ltwh(0, 0, 2, 2)).collect();
        assert_eq!(elements, vec![1, 1, 1, 1]);
    }

    #[test]
    fn grid_mapped_size() {
        let grid = GridBuf::<u8, _, _>::new(10, 10);
        let mapped = grid.map(|x| x * 2);
        assert_eq!(mapped.width(), 10);
        assert_eq!(mapped.height(), 10);
    }

    #[test]
    fn grid_mapped_get() {
        let grid = GridBuf::new_filled(3, 3, 1);
        let mapped = grid.map(|x| x * 2);
        assert_eq!(mapped.get(Pos::new(1, 1)), Some(2));
        assert_eq!(mapped.get(Pos::new(3, 3)), None);
    }

    #[test]
    fn grid_mapped_iter_rect() {
        let grid = GridBuf::new_filled(3, 3, 1);
        let mapped = grid.map(|x| x * 2);
        let elements: Vec<_> = mapped.iter_rect(Rect::from_ltwh(0, 0, 2, 2)).collect();
        assert_eq!(elements, vec![2, 2, 2, 2]);
    }

    #[test]
    fn grid_view_size() {
        let grid = GridBuf::<u8, _, _>::new(10, 10);
        let view = grid.view(Rect::from_ltwh(0, 0, 5, 5));
        assert_eq!(view.width(), 5);
        assert_eq!(view.height(), 5);
    }

    #[test]
    fn grid_view_get() {
        let grid = GridBuf::new_filled(3, 3, 1);
        let view = grid.view(Rect::from_ltwh(0, 0, 2, 2));
        assert_eq!(view.get(Pos::new(1, 1)), Some(&1));
        assert_eq!(view.get(Pos::new(2, 2)), None);
    }

    #[test]
    fn grid_view_iter_rect() {
        let grid = GridBuf::new_filled(3, 3, 1);
        let view = grid.view(Rect::from_ltwh(0, 0, 2, 2));
        let elements: Vec<_> = view.iter_rect(Rect::from_ltwh(0, 0, 2, 2)).collect();
        assert_eq!(elements, &[&1, &1, &1, &1]);
    }

    #[test]
    fn grid_scaled_size() {
        let grid = GridBuf::<u8, _, _>::new(10, 10);
        let scaled = grid.scale(2);
        assert_eq!(scaled.width(), 20);
        assert_eq!(scaled.height(), 20);
    }

    #[test]
    fn grid_scaled_get() {
        let grid = GridBuf::<_, _>::from_buffer(vec![1, 2, 3, 4], 2);
        let scaled = grid.scale(2);
        assert_eq!(scaled.get(Pos::new(1, 1)), Some(&1));
        assert_eq!(scaled.get(Pos::new(2, 2)), Some(&4));
        assert_eq!(scaled.get(Pos::new(3, 3)), Some(&4));
        assert_eq!(scaled.get(Pos::new(4, 4)), None);
    }

    #[test]
    fn grid_scaled_iter_rect() {
        let grid = GridBuf::<_, _>::from_buffer(vec![1, 2, 3, 4], 2);
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

    #[test]
    fn grid_blended_size() {
        let mut grid = GridBuf::<u8, _, _>::new(10, 10);
        let mut blended = grid.blend(|current, new| current + new);
        blended.set(Pos::new(1, 1), 5).unwrap();
        assert_eq!(blended.width(), 10);
        assert_eq!(blended.height(), 10);
    }

    #[test]
    fn grid_write_blended_set() {
        let mut grid = GridBuf::new_filled(3, 3, 0);
        let mut blended = grid.blend(|current, new| current + new);
        blended.set(Pos::new(1, 1), 5).unwrap();
        assert_eq!(blended.get(Pos::new(1, 1)), Some(&5));
        blended.set(Pos::new(1, 1), 3).unwrap();
        assert_eq!(blended.get(Pos::new(1, 1)), Some(&8));
    }

    #[test]
    fn grid_write_blended_iter_rect() {
        let mut grid = GridBuf::new_filled(3, 3, 0);
        let mut blended = grid.blend(|current, new| current + new);
        blended.set(Pos::new(1, 1), 5).unwrap();
        blended.set(Pos::new(2, 2), 3).unwrap();
        let elements: Vec<_> = blended.iter_rect(Rect::from_ltwh(0, 0, 3, 3)).collect();
        assert_eq!(elements, vec![&0, &0, &0, &0, &5, &0, &0, &0, &3]);
    }
}
