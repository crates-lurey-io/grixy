//! Blending operations for drawing elements in a grid.

use crate::{
    core::{GridError, Layout as _, Pos, Rect},
    ops::{GridRead, GridWrite},
};

/// Perform blending operations on a grid.
pub trait GridBlend: GridRead + GridWrite
where
    for<'a> <Self as GridRead>::Element<'a>: Copy,
    Self: Sized,
{
    /// Sets the element at a specified position, applying a blend function.
    ///
    /// The current element at the destination position is blended with the provided value.
    ///
    /// ## Errors
    ///
    /// If the position is out of bounds, this method returns an error.
    fn blend_set(
        &mut self,
        dst: Pos,
        value: <Self as GridWrite>::Element,
        blend: &impl Fn(
            <Self as GridRead>::Element<'_>,
            <Self as GridWrite>::Element,
        ) -> <Self as GridWrite>::Element,
    ) -> Result<(), GridError> {
        let current = self.get(dst).ok_or(GridError)?;
        self.set(dst, blend(current, value))
    }

    /// Blends a rectangular region from a source grid to the destination grid.
    ///
    /// The operation starts by blending the top-left corner to the specified `dst`, blending each
    /// cell from the source grid with corresponding cell in the destination grid. If there is
    /// insufficient space in the current grid or the `rect` is out of bounds, those individual
    /// cells are ignored and not copied to/from.
    fn blend_rect<'src, S>(
        &mut self,
        dst: Pos,
        src: &'src S,
        rect: Rect,
        blend: &impl Fn(
            <Self as GridRead>::Element<'_>,
            <Self as GridWrite>::Element,
        ) -> <Self as GridWrite>::Element,
    ) where
        S: GridRead<Element<'src> = <Self as GridWrite>::Element>,
    {
        for pos in <Self as GridRead>::Layout::iter_pos(rect) {
            if let Some(value) = src.get(pos) {
                let _ = self.blend_set(dst + pos, value, blend);
            }
        }
    }
}

impl<T> GridBlend for T
where
    T: GridRead + GridWrite,
    for<'a> <T as GridRead>::Element<'a>: Copy,
{
}

/// Clears the destination element, returning a default value.
///
/// ## Examples
///
/// ```rust
/// use grixy::ops::blend;
///
/// let src = 42;
/// let dst: i32 = 24;
/// let result = blend::clear(src, dst);
/// assert_eq!(result, 0);
/// ```
pub fn clear<S, D>(_src: S, _dst: D) -> D
where
    D: Default,
{
    D::default()
}

/// Replaces the destination element with the source element.
///
/// ```rust
/// use grixy::ops::blend;
///
/// let src = 42;
/// let dst: i32 = 24;
/// let result = blend::source(src, dst);
/// assert_eq!(result, 42);
/// ```
pub fn source<S, D>(src: S, _dst: D) -> S {
    src
}

/// Retains the destination element, ignoring the source element.
///
/// ```rust
/// use grixy::ops::blend;
///
/// let src = 42;
/// let dst: i32 = 24;
/// let result = blend::destination(src, dst);
/// assert_eq!(result, 24);
/// ```
pub fn destination<S, D>(_src: S, dst: D) -> D {
    dst
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blend_clear_test() {
        let src = 42;
        let dst: i32 = 24;
        let result = clear(src, dst);
        assert_eq!(result, 0);
    }

    #[test]
    fn blend_source_test() {
        let src = 42;
        let dst: i32 = 24;
        let result = source(src, dst);
        assert_eq!(result, 42);
    }

    #[test]
    fn blend_destination_test() {
        let src = 42;
        let dst: i32 = 24;
        let result = destination(src, dst);
        assert_eq!(result, 24);
    }
}
