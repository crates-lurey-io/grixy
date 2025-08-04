/// Defines the layout of a grid in memory.
pub trait Layout {}

/// Sparse layout for grids, where elements are not stored in a contiguous block of memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sparse;

impl Layout for Sparse {}

/// Defines the layout of a grid in linear (contiguous) memory.
pub trait Linear: Layout {}

/// Top-to-bottom, left-to-right traversal order for 2D layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnMajor;

impl Layout for ColumnMajor {}
impl Linear for ColumnMajor {}

/// Left-to-right, top-to-bottom traversal order for 2D layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowMajor;

impl Layout for RowMajor {}
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
pub struct Block<G, C> {
    grid: G,
    cell: C,
}

impl<G, C> Layout for Block<G, C>
where
    G: Layout,
    C: Layout,
{
}

impl<G, C> Linear for Block<G, C>
where
    G: Linear,
    C: Linear,
{
}
