use crate::{
    core::{Pos, Rect},
    ops::{ExactSizeGrid, GridRead, layout::Traversal as _},
};

/// Extension trait for comparing two grids.
///
/// Automatically implemented for all types that implement [`GridRead`] and [`ExactSizeGrid`].
///
/// # Examples
///
/// ```rust
/// use grixy::prelude::*;
///
/// let a = GridBuf::new_filled(3, 3, 0u8);
/// let mut b = GridBuf::new_filled(3, 3, 0u8);
/// b[Pos::new(1, 1)] = 42;
///
/// let changed: Vec<_> = a.diff(&b).collect();
/// assert_eq!(changed, [(Pos::new(1, 1), &0u8)]);
/// ```
pub trait GridDiff: GridRead + ExactSizeGrid {
    /// Returns an iterator over positions where `self` differs from `other`.
    ///
    /// Elements are compared with [`PartialEq`]. Positions are yielded in the
    /// traversal order defined by `Self::Layout`.
    ///
    /// If the grids have different dimensions, all positions in `self` are
    /// considered changed. This matches the rg double-buffering contract: resize
    /// both buffers before diffing.
    fn diff<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = (Pos, Self::Element<'a>)> + 'a
    where
        Self::Element<'a>: PartialEq;
}

impl<G> GridDiff for G
where
    G: GridRead + ExactSizeGrid,
{
    fn diff<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = (Pos, Self::Element<'a>)> + 'a
    where
        Self::Element<'a>: PartialEq,
    {
        let same_size = self.width() == other.width() && self.height() == other.height();
        let full_rect = Rect::from_ltwh(0, 0, self.width(), self.height());

        Self::Layout::iter_pos(full_rect).filter_map(move |pos| {
            let current = self.get(pos)?;
            if same_size {
                let previous = other.get(pos)?;
                if current == previous {
                    return None;
                }
            }
            Some((pos, current))
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::{
        buf::GridBuf,
        core::Pos,
        ops::{ExactSizeGrid as _, GridDiff as _, GridRead as _},
    };
    use alloc::vec::Vec;

    #[test]
    fn diff_same_grid() {
        let a = GridBuf::new_filled(3, 3, 0u8);
        let b = GridBuf::new_filled(3, 3, 0u8);
        let changed: Vec<_> = a.diff(&b).collect();
        assert!(changed.is_empty());
    }

    #[test]
    fn diff_one_changed_cell() {
        let a = GridBuf::new_filled(3, 3, 0u8);
        let mut b = GridBuf::new_filled(3, 3, 0u8);
        b[Pos::new(1, 1)] = 42;

        let changed: Vec<_> = a.diff(&b).collect();
        assert_eq!(changed, [(Pos::new(1, 1), &0u8)]);
    }

    #[test]
    fn diff_all_changed() {
        let a = GridBuf::new_filled(3, 3, 0u8);
        let b = GridBuf::new_filled(3, 3, 1u8);

        let changed: Vec<_> = a.diff(&b).collect();
        assert_eq!(changed.len(), 9);
        assert!(changed.iter().all(|&(_, v)| *v == 0));
    }

    #[test]
    fn diff_different_sizes() {
        let a = GridBuf::new_filled(2, 2, 0u8);
        let b = GridBuf::new_filled(3, 3, 0u8);

        // All positions in self (2x2) are considered changed
        let changed: Vec<_> = a.diff(&b).collect();
        assert_eq!(changed.len(), 4);
    }
}
