use crate::{
    core::{Pos, Rect},
    ops::{ExactSizeGrid, GridRead, layout::Layout as _},
};

/// Describes how a single cell differs between two grids, as yielded by [`GridDiff::diff_from`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridChange<T> {
    /// The cell exists in the grid being diffed, but has no counterpart in `other`.
    ///
    /// This happens when `other` is smaller than `self`, so a position that's valid in `self`
    /// falls outside `other`'s bounds.
    Added(T),

    /// The cell exists in both grids, but the values differ.
    ///
    /// `from` is `other`'s value (the previous/baseline state), `to` is `self`'s value (the
    /// current state).
    Modified {
        /// The value in `other` (the previous state).
        from: T,
        /// The value in `self` (the current state).
        to: T,
    },
}

impl<T> GridChange<T> {
    /// Returns the "current" value of this change, i.e. `self`'s value.
    ///
    /// For [`Added`](GridChange::Added) this is the added value; for
    /// [`Modified`](GridChange::Modified) this is `to`.
    #[must_use]
    pub fn current(&self) -> &T {
        match self {
            GridChange::Added(value) | GridChange::Modified { to: value, .. } => value,
        }
    }
}

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
/// let changed: Vec<_> = b.diff_from(&a).collect();
/// assert_eq!(
///     changed,
///     [(Pos::new(1, 1), GridChange::Modified { from: &0u8, to: &42u8 })]
/// );
/// ```
pub trait GridDiff: GridRead + ExactSizeGrid {
    /// Returns an iterator over cells where `self` differs from `other`.
    ///
    /// `self` is the current/new state, `other` is the previous/baseline state — matching the
    /// common double-buffering pattern in game loops, where you diff this frame's grid against
    /// last frame's to figure out what changed.
    ///
    /// Elements are compared with [`PartialEq`]. Positions are yielded in the traversal order
    /// defined by `Self::Layout`.
    ///
    /// If `other` is smaller than `self`, positions that fall outside `other`'s bounds are
    /// yielded as [`GridChange::Added`]. Positions inside both grids are yielded as
    /// [`GridChange::Modified`] when they differ, and omitted when they're equal.
    fn diff_from<'a>(
        &'a self,
        other: &'a Self,
    ) -> impl Iterator<Item = (Pos, GridChange<Self::Element<'a>>)> + 'a
    where
        Self::Element<'a>: PartialEq;
}

impl<G> GridDiff for G
where
    G: GridRead + ExactSizeGrid,
{
    fn diff_from<'a>(
        &'a self,
        other: &'a Self,
    ) -> impl Iterator<Item = (Pos, GridChange<Self::Element<'a>>)> + 'a
    where
        Self::Element<'a>: PartialEq,
    {
        let full_rect = Rect::from_ltwh(0, 0, self.width(), self.height());

        Self::Layout::iter_pos(full_rect).filter_map(move |pos| {
            let current = self.get(pos)?;
            match other.get(pos) {
                Some(previous) if previous == current => None,
                Some(previous) => Some((
                    pos,
                    GridChange::Modified {
                        from: previous,
                        to: current,
                    },
                )),
                None => Some((pos, GridChange::Added(current))),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::{
        buf::GridBuf,
        core::Pos,
        ops::{GridChange, GridDiff as _},
    };
    use alloc::vec::Vec;

    #[test]
    fn diff_same_grid() {
        let a = GridBuf::new_filled(3, 3, 0u8);
        let b = GridBuf::new_filled(3, 3, 0u8);
        let changed: Vec<_> = a.diff_from(&b).collect();
        assert!(changed.is_empty());
    }

    #[test]
    fn diff_one_changed_cell() {
        let mut b = GridBuf::new_filled(3, 3, 0u8);
        let a = GridBuf::new_filled(3, 3, 0u8);
        b[Pos::new(1, 1)] = 42;

        let changed: Vec<_> = b.diff_from(&a).collect();
        assert_eq!(
            changed,
            [(
                Pos::new(1, 1),
                GridChange::Modified {
                    from: &0u8,
                    to: &42u8
                }
            )]
        );
    }

    #[test]
    fn diff_all_changed() {
        let a = GridBuf::new_filled(3, 3, 1u8);
        let b = GridBuf::new_filled(3, 3, 0u8);

        let changed: Vec<_> = a.diff_from(&b).collect();
        assert_eq!(changed.len(), 9);
        assert!(changed.iter().all(|(_, c)| **c.current() == 1));
    }

    #[test]
    fn diff_larger_than_other() {
        let a = GridBuf::new_filled(3, 3, 0u8);
        let b = GridBuf::new_filled(2, 2, 0u8);

        // Positions outside `b`'s bounds are reported as `Added`.
        let changed: Vec<_> = a.diff_from(&b).collect();
        assert_eq!(changed.len(), 5);
        assert!(
            changed
                .iter()
                .all(|(_, c)| matches!(c, GridChange::Added(_)))
        );
    }
}
