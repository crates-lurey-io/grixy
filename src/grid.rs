//! Grid traits.

mod base;
pub use base::GridBase;

mod bounded;
pub use bounded::BoundedGrid;

mod read;
pub use read::{GridRead, GridReadUnchecked};

mod write;
pub use write::{GridWrite, GridWriteUnchecked};

/// A 2-dimensional grid that supports both reading and writing at specific positions.
pub trait Grid: GridRead + GridWrite {}

/// Automatically implement `Grid` when `GridRead` + `GridWrite` are implemented.
impl<T: GridRead + GridWrite> Grid for T {}
