use crate::core::{Pos, Rect};

use ixy::layout::{self, Traversal as _};

/// Crate-internal trait for mapping to [`ixy::layout`].
trait InternalLayout {
    type Traversal: layout::Traversal;

    /// Returns the layout traversal for this grid layout.
    fn as_traversal(&self) -> &Self::Traversal;
}

impl<T: InternalLayout> Layout for T {
    fn iter_pos(&self, rect: Rect) -> impl Iterator<Item = Pos> {
        self.as_traversal().iter_pos(rect)
    }
}

/// Defines the layout of a grid in memory.
pub trait Layout {
    /// Returns an iterator over positions in the given rectangle.
    ///
    /// The order of the positions is determined by the layout's traversal order.
    fn iter_pos(&self, rect: Rect) -> impl Iterator<Item = Pos>;
}

/// Sparse layout for grids, where elements are not stored in a contiguous block of memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sparse<T = RowMajor>
where
    T: Layout,
{
    inner: T,
}

/// Defines the layout of a grid in linear (contiguous) memory.
pub trait Linear: Layout {}

/// Top-to-bottom, left-to-right traversal order for 2D layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnMajor;

impl InternalLayout for ColumnMajor {
    type Traversal = layout::ColumnMajor;

    fn as_traversal(&self) -> &Self::Traversal {
        &layout::ColumnMajor
    }
}

impl Linear for ColumnMajor {}

/// Left-to-right, top-to-bottom traversal order for 2D layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowMajor;

impl InternalLayout for RowMajor {
    type Traversal = layout::RowMajor;

    fn as_traversal(&self) -> &Self::Traversal {
        &layout::RowMajor
    }
}

impl Linear for RowMajor {}

/// 2D space divided into blocks, each containing a grid of cells.
///
/// Each block has a fixed size (that may be defined at runtime), and is traversed using layout `G`
/// for each block, and layout `C` for each cell within the block; by default, both are `RowMajor`
/// but can be customized using the [`with_grid`] and [`with_cell`] methods.
///
/// [`with_grid`]: Block::with_grid
/// [`with_cell`]: Block::with_cell
///
/// For example, `Block<RowMajor, RowMajor>` with a block-size of 2x2:
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block<I> {
    inner: I,
}

impl<G, C> InternalLayout for Block<layout::Block<G, C>>
where
    G: layout::Traversal,
    C: layout::Traversal,
{
    type Traversal = layout::Block<G, C>;

    fn as_traversal(&self) -> &Self::Traversal {
        &self.inner
    }
}

impl<G, C> Linear for Block<layout::Block<G, C>>
where
    G: layout::Traversal,
    C: layout::Traversal,
{
}
