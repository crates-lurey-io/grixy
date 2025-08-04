/// Prevents a trait from being implemented outside this crate.
#[allow(dead_code)]
pub trait Sealed {}

/// The result of iterating over a rectangular region of a grid.
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
            IterRect::Aligned(iter) => iter.next(),
            IterRect::Unaligned(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            IterRect::Aligned(iter) => iter.size_hint(),
            IterRect::Unaligned(iter) => iter.size_hint(),
        }
    }
}
