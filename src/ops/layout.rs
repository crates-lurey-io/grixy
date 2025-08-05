//! Defines how to traverse and optionally, store grids in linear memory.

use core::marker::PhantomData;

use crate::core::{Pos, Rect, Size};

use ixy::layout::{self, Linear as _, Traversal as _};

/// Crate-internal trait for mapping to [`ixy::layout`].
trait InternalLayout {
    type Traversal: layout::Traversal;
}

impl<T: InternalLayout> Layout for T {
    fn iter_pos(rect: Rect) -> impl Iterator<Item = Pos> {
        T::Traversal::iter_pos(rect)
    }
}

/// Defines the layout of a grid in memory.
pub trait Layout {
    /// Returns an iterator over positions in the given rectangle.
    ///
    /// The order of the positions is determined by the layout's traversal order.
    fn iter_pos(rect: Rect) -> impl Iterator<Item = Pos>;
}

/// Sparse layout for grids, where elements are not stored in a contiguous block of memory.
pub struct Sparse<T = RowMajor> {
    inner: PhantomData<T>,
}

impl<T: InternalLayout<Traversal: layout::Linear>> Linear for T {
    fn to_1d(pos: Pos, width: usize) -> usize {
        T::Traversal::pos_to_index(pos, width)
    }

    fn to_2d(index: usize, width: usize) -> Pos {
        T::Traversal::index_to_pos(index, width)
    }

    fn slice_rect_aligned<E>(slice: &[E], size: Size, rect: Rect) -> Option<&[E]> {
        T::Traversal::slice_rect_aligned(slice, size, rect)
    }

    fn slice_rect_aligned_mut<E>(slice: &mut [E], size: Size, rect: Rect) -> Option<&mut [E]> {
        T::Traversal::slice_rect_aligned_mut(slice, size, rect)
    }
}

/// Defines the layout of a grid in linear (contiguous) memory.
pub trait Linear: Layout {
    /// Converts a 2D position to a 1D index based on the grid's width.
    fn to_1d(pos: Pos, width: usize) -> usize;

    /// Converts a 1D index to a 2D position based on the grid's width.
    fn to_2d(index: usize, width: usize) -> Pos;

    /// Returns an aligned slice of the grid's elements within a rectangular region.
    ///
    /// If the rectangle is not aligned with the grid's layout, returns `None`.
    fn slice_rect_aligned<E>(slice: &[E], size: Size, rect: Rect) -> Option<&[E]>;

    /// Returns an aligned slice of the grid's elements within a rectangular region.
    ///
    /// If the rectangle is not aligned with the grid's layout, returns `None`.
    fn slice_rect_aligned_mut<E>(slice: &mut [E], size: Size, rect: Rect) -> Option<&mut [E]>;
}

/// Top-to-bottom, left-to-right traversal order for 2D layouts.
pub enum ColumnMajor {}

impl InternalLayout for ColumnMajor {
    type Traversal = layout::ColumnMajor;
}

/// Left-to-right, top-to-bottom traversal order for 2D layouts.
pub enum RowMajor {}

impl InternalLayout for RowMajor {
    type Traversal = layout::RowMajor;
}

/// 2D space divided into blocks, each containing a grid of cells.
///
/// For example, `Block<2, 2>` (a 2x2 block with row-major layout) would look like this:
///
/// ```txt
/// B0:   B1:
/// +----+----+
/// | 01 | 45 |
/// | 23 | 67 |
/// +----+----+
/// B2:   B3:
/// +----+----+
/// | 89 | CD |
/// | AB | EF |
/// +----+----+
/// ```
#[allow(private_bounds)]
pub struct Block<const W: usize, const H: usize, G = RowMajor, C = G>
where
    G: InternalLayout<Traversal: layout::Traversal>,
    C: InternalLayout<Traversal: layout::Traversal>,
{
    inner: PhantomData<(G, C)>,
}

impl<const W: usize, const H: usize, G, C> InternalLayout for Block<W, H, G, C>
where
    G: InternalLayout<Traversal: layout::Traversal>,
    C: InternalLayout<Traversal: layout::Traversal>,
{
    type Traversal = layout::Block<W, H, G::Traversal, C::Traversal>;
}
