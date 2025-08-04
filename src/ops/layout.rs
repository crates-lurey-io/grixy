use core::marker::PhantomData;

use crate::core::{Pos, Rect};

use ixy::layout::{self, Traversal as _};

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

/// Defines the layout of a grid in linear (contiguous) memory.
pub trait Linear: Layout {}

/// Top-to-bottom, left-to-right traversal order for 2D layouts.
pub enum ColumnMajor {}

impl InternalLayout for ColumnMajor {
    type Traversal = layout::ColumnMajor;
}

impl Linear for ColumnMajor {}

/// Left-to-right, top-to-bottom traversal order for 2D layouts.
pub enum RowMajor {}

impl InternalLayout for RowMajor {
    type Traversal = layout::RowMajor;
}

impl Linear for RowMajor {}

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
pub struct Block<const W: usize, const H: usize, I> {
    inner: PhantomData<I>,
}

impl<const W: usize, const H: usize, G, C> InternalLayout for Block<W, H, layout::Block<W, H, G, C>>
where
    G: layout::Traversal,
    C: layout::Traversal,
{
    type Traversal = layout::Block<W, H, G, C>;
}

impl<const W: usize, const H: usize, G, C> Linear for Block<W, H, layout::Block<W, H, G, C>>
where
    G: layout::Traversal,
    C: layout::Traversal,
{
}
