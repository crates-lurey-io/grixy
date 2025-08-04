//! Transformation operations for grids.
//!
//! [`GridConvertExt`] is automatically implemented for all types that implement reading, or in the
//! case of [`blend`][GridConvertExt::blend], reading _and_ writing from a grid, and provides
//! additional methods for lazily transforming grids (leaving the original grid unchanged, and
//! without any allocations).
//!
//! Operations include:
//!
//! - [`blend`](GridConvertExt::blend): Creates a blended version of the grid, applying a blend function when setting elements.
//! - [`collect`](GridConvertExt::collect): Collects the elements of the grid into a new buffer.
//! - [`copied`](GridConvertExt::copied): Creates a grid that copies all of its elements.
//! - [`map`](GridConvertExt::map): Creates a grid that applies a mapping function to its elements.
//! - [`scale`](GridConvertExt::scale): Creates a scaled version of the grid.
//! - [`view`](GridConvertExt::view): Creates a view of the grid over a specified rectangular region.
//!
//! ## Chaining transformations
//!
//! Methods on [`GridConvertExt`] can be chained together to create complex transformations, as
//! they consume the grid and return a new one. This allows for a functional style of programming
//! where each transformation is applied in sequence, without modifying the original grid:
//!
//! ```rust
//! use grixy::prelude::*;
//!
//! let grid = GridBuf::new_filled(3, 3, 1)
//!   .copied()
//!   .map(|x| x * 2)
//!   .view(Rect::from_ltwh(0, 0, 2, 2))
//!   .scale(2);
//!
//! assert_eq!(grid.get(Pos::new(1, 1)), Some(2));
//! ```
//!
//! ## Sharing a grid
//!
//! To share the original grid, you can use `Rc` or `Arc` to wrap it:
//!
//! ```rust
//! // Or alloc::rc::Rc;
//! use std::rc::Rc;
//! use grixy::prelude::*;
//!
//! let rc = Rc::new(GridBuf::new_filled(3, 3, 1));
//!
//! let rf = Rc::clone(&rc);
//! let chained = rf
//!   .copied()
//!   .map(|x| x * 2)
//!   .view(Rect::from_ltwh(0, 0, 2, 2))
//!   .scale(2);
//! assert_eq!(chained.get(Pos::new(1, 1)), Some(2));
//!
//! // Original grid is still accessible
//! assert_eq!(rc.get(Pos::new(1, 1)), Some(&1));
//! ```

use core::marker::PhantomData;

#[cfg(feature = "buffer")]
use crate::ops::layout;
use crate::{
    core::Rect,
    ops::{GridRead, GridWrite, unchecked::TrustedSizeGrid},
};

mod blended;
pub use blended::Blended;

mod copied;
pub use copied::Copied;

mod mapped;
pub use mapped::Mapped;

mod scaled;
pub use scaled::Scaled;

mod viewed;
pub use viewed::Viewed;

pub mod blend;

/// Extension trait for converting grids into different forms.
pub trait GridConvertExt: GridRead {
    /// Creates a grid that copies all of its elements.
    ///
    /// This is useful when you have a `GridRead<&T>`, but need a `GridRead<T>`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use grixy::prelude::*;
    ///
    /// // By default, `GridBuf` returns references to its elements (similar to `Vec`).
    /// let grid = GridBuf::new_filled(3, 3, 1);
    /// assert_eq!(grid.get(Pos::new(1, 1)), Some(&1));
    ///
    /// // We can create a `GridRead` that returns owned copies of the elements.
    /// let copied = grid.copied();
    /// assert_eq!(copied.get(Pos::new(1, 1)), Some(1));
    /// ```
    fn copied<'a, T>(self) -> Copied<T, Self>
    where
        Self: Sized + GridRead<Element<'a> = &'a T> + 'a,
        T: Copy + 'a,
    {
        Copied {
            source: self,
            _element: PhantomData,
        }
    }

    /// Creates a grid that applies a mapping function to its elements.
    ///
    /// This is useful when you want to transform the elements of a grid lazily.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use grixy::prelude::*;
    ///
    /// let grid = GridBuf::new_filled(3, 3, 1);
    /// let mapped = grid.map(|&x| x * 2);
    /// assert_eq!(mapped.get(Pos::new(1, 1)), Some(2));
    /// ```
    fn map<F, T>(self, map_fn: F) -> Mapped<F, Self, T>
    where
        Self: Sized,
        F: Fn(Self::Element<'_>) -> T,
    {
        Mapped {
            source: self,
            map_fn,
            _element: PhantomData,
        }
    }

    /// Creates a view of the grid over a specified rectangular region.
    ///
    /// The view is a lightweight wrapper that allows access to a subset of the grid's elements.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use grixy::prelude::*;
    ///
    /// let grid = GridBuf::new_filled(3, 3, 1);
    /// let view = grid.view(Rect::from_ltwh(0, 0, 2, 2));
    /// assert_eq!(view.get(Pos::new(1, 1)), Some(&1));
    /// assert_eq!(view.get(Pos::new(2, 2)), None);
    /// ```
    fn view(self, bounds: Rect) -> Viewed<Self>
    where
        Self: Sized,
    {
        Viewed {
            source: self,
            bounds,
        }
    }

    /// Creates a scaled version of the grid.
    ///
    /// The `scale` factor determines how many cells in the original grid correspond to one cell
    /// in the scaled grid. For example, a scale factor of 2 means that each cell in the scaled grid
    /// corresponds to a 2x2 block of cells in the original grid.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use grixy::prelude::*;
    ///
    /// let grid = GridBuf::new_filled(2, 2, 1);
    /// let scaled = grid.scale(2);
    /// assert_eq!(scaled.get(Pos::new(0, 0)), Some(&1));
    /// assert_eq!(scaled.get(Pos::new(1, 1)), Some(&1));
    /// assert_eq!(scaled.get(Pos::new(2, 2)), Some(&1));
    /// assert_eq!(scaled.get(Pos::new(3, 3)), Some(&1));
    /// assert_eq!(scaled.get(Pos::new(4, 4)), None);
    /// ```
    fn scale(self, factor: usize) -> Scaled<Self>
    where
        Self: Sized,
    {
        Scaled {
            source: self,
            scale: factor,
        }
    }

    /// Collects the elements of the grid into a new buffer.
    ///
    /// This method is only available when the `buffer` feature is enabled.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use grixy::prelude::*;
    ///
    /// let grid = GridBuf::new_filled(3, 3, 1);
    /// let collected = grid.copied().collect::<Vec<_>>(RowMajor);
    /// assert_eq!(collected.get(Pos::new(1, 1)), Some(&1));
    /// assert_eq!(collected.get(Pos::new(3, 3)), None);
    /// ```
    #[cfg(feature = "buffer")]
    fn collect<'a, B, L>(&'a self) -> crate::buf::GridBuf<Self::Element<'a>, B, L>
    where
        B: FromIterator<Self::Element<'a>> + AsRef<[Self::Element<'a>]>,
        L: layout::Linear,
        Self: Sized,
        Self: TrustedSizeGrid,
        Self::Element<'a>: Copy,
    {
        use crate::core::Rect;

        let iter = self.iter_rect(Rect::from_ltwh(0, 0, self.width(), self.height()));
        let elem = iter.collect::<B>();
        crate::buf::GridBuf::from_buffer(elem, self.width())
    }

    /// Creates a blended version of this grid, applying a blend function when setting elements.
    ///
    /// This is useful for operations like blending colors or combining values.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use grixy::prelude::*;
    ///
    /// let mut grid = GridBuf::new_filled(3, 3, 1);
    /// let blend_fn = |current: &i32, new: i32| current + new;
    /// let mut blended = grid.blend(blend_fn);
    ///
    /// blended.set(Pos::new(1, 1), 5).unwrap();
    /// assert_eq!(blended.get(Pos::new(1, 1)), Some(&6));
    /// ```
    fn blend<F>(&mut self, blend_fn: F) -> Blended<'_, Self, F>
    where
        Self: Sized + GridRead + GridWrite,
        F: Fn(
            <Self as GridRead>::Element<'_>,
            <Self as GridWrite>::Element,
        ) -> <Self as GridWrite>::Element,
    {
        Blended {
            source: self,
            blend_fn,
        }
    }
}

impl<T> GridConvertExt for T where T: GridRead {}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::{
        buf::GridBuf,
        core::{Pos, Rect},
        ops::layout::RowMajor,
    };
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
        let grid = GridBuf::<_, _, RowMajor>::from_buffer(vec![1, 2, 3, 4], 2);
        let scaled = grid.scale(2);
        assert_eq!(scaled.get(Pos::new(1, 1)), Some(&1));
        assert_eq!(scaled.get(Pos::new(2, 2)), Some(&4));
        assert_eq!(scaled.get(Pos::new(3, 3)), Some(&4));
        assert_eq!(scaled.get(Pos::new(4, 4)), None);
    }

    #[test]
    fn grid_scaled_iter_rect() {
        let grid = GridBuf::<_, _, RowMajor>::from_buffer(vec![1, 2, 3, 4], 2);
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

    #[test]
    fn grid_chained_operations() {
        let grid = GridBuf::new_filled(3, 3, 1)
            .copied()
            .map(|x| x * 2)
            .view(Rect::from_ltwh(0, 0, 2, 2))
            .scale(2);

        assert_eq!(grid.get(Pos::new(1, 1)), Some(2));
    }

    #[test]
    fn grid_rc() {
        use alloc::rc::Rc;

        let rc = Rc::new(GridBuf::new_filled(3, 3, 1));
        let rf = Rc::clone(&rc);
        let chained = rf
            .copied()
            .map(|x| x * 2)
            .view(Rect::from_ltwh(0, 0, 2, 2))
            .scale(2);
        assert_eq!(chained.get(Pos::new(1, 1)), Some(2));
    }

    #[test]
    fn grid_arc() {
        use alloc::sync::Arc;

        let arc = Arc::new(GridBuf::new_filled(3, 3, 1));
        let af = Arc::clone(&arc);
        let chained = af
            .copied()
            .map(|x| x * 2)
            .view(Rect::from_ltwh(0, 0, 2, 2))
            .scale(2);
        assert_eq!(chained.get(Pos::new(1, 1)), Some(2));
    }
}
