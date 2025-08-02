//! Blending operations for drawing elements in a grid.

/// Clears the destination element, returning a default value.
///
/// ## Examples
///
/// ```rust
/// use grixy::convert::blend;
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
/// use grixy::convert::blend;
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
/// use grixy::convert::blend;
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
