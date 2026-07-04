/// Prevents a trait from being implemented outside this crate.
#[allow(dead_code)]
pub trait Sealed {}

/// The result of iterating over a rectangular region of a grid.
#[allow(dead_code)]
// `redundant_pub_crate` and `unreachable_pub` disagree on the right visibility for an item in a
// `pub(crate)` module: the former wants `pub` (since the enclosing module is already crate-
// private), the latter flags `pub` as unreachable outside the crate and wants `pub(crate)`.
// `unreachable_pub` reflects the truth (nothing outside this crate can see `IterRect`), so keep
// `pub(crate)` and allow the redundancy lint here.
#[allow(clippy::redundant_pub_crate)]
pub(crate) enum IterRect<T, A, U>
where
    A: Iterator<Item = T>,
    U: Iterator<Item = T>,
{
    /// The region is aligned, meaning the grid's layout matches the region's layout.
    Aligned(A),

    /// The region is unaligned, meaning the grid's layout does not match the region's layout.
    Unaligned(U),
}

impl<T, A, U> Iterator for IterRect<T, A, U>
where
    A: Iterator<Item = T>,
    U: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Aligned(iter) => iter.next(),
            Self::Unaligned(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Aligned(iter) => iter.size_hint(),
            Self::Unaligned(iter) => iter.size_hint(),
        }
    }
}
