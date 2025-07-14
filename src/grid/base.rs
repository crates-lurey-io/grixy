/// Marker type for a 2-dimensional grid.
///
/// No functionality is provided by this trait other than an associated type, [`GridBase::Element`].
pub trait GridBase {
    /// The element stored in the grid at a specific position.
    type Element;
}
