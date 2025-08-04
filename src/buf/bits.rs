//! Provides a single-bit, 2D grid data structure backed by a linear buffer.
//!
//! The main type, [`GridBits`], is highly generic, allowing it to act as a view over any buffer
//! that can be treated as a slice of bytes.
//!
//! # Examples
//!
//! Creating an owned `GridBits` and accessing an element:
//! ```
//! use grixy::{core::{Pos, RowMajor}, buf::bits::GridBits, ops::GridRead};
//!
//! let grid = GridBits::<u8, _, RowMajor>::new(8, 1);
//! assert_eq!(grid.get(Pos::new(3, 0)), Some(false));
//! ```

use core::marker::PhantomData;

mod ops;
pub use ops::BitOps;

use crate::{
    core::Pos,
    internal,
    ops::{
        layout,
        unchecked::{GridReadUnchecked, GridWriteUnchecked, TrustedSizeGrid},
    },
};

#[cfg(feature = "alloc")]
extern crate alloc;

/// A 2-dimensional grid where every individual bit is treated as either `true` or `false`.
///
/// ## Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
#[derive(Debug, Clone)]
pub struct GridBits<T, B, L>
where
    T: BitOps,
    L: layout::Linear,
{
    buffer: B,
    width: usize,
    height: usize,
    _layout: PhantomData<L>,
    _element: PhantomData<T>,
}

impl<T, B, L> GridBits<T, B, L>
where
    T: BitOps,
    B: AsRef<[T]>,
    L: layout::Linear,
{
    /// Returns a grid from an existing buffer with a given width in columns.
    ///
    /// The height is inferred from the buffer length and width.
    ///
    /// Any data type that can be represented as a slice can be used as the buffer type, including
    /// arrays, slices, and vectors.
    ///
    /// ## Panics
    ///
    /// This panics if the buffer length is not a multiple of the width.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use grixy::{core::Pos, buf::bits::GridBits, ops::GridRead};
    ///
    /// let grid = GridBits::<_, Vec<u8>>::from_buffer(vec![1, 2, 3, 4], 2);
    /// assert_eq!(grid.get(Pos::new(0, 0)), Some(true));
    ///
    /// assert_eq!(grid.get(Pos::new(1, 0)), Some(false));
    /// assert_eq!(grid.get(Pos::new(2, 0)), None);
    /// ```
    #[must_use]
    pub fn from_buffer(buffer: B, width: usize) -> Self {
        let height = buffer.as_ref().len() * T::MAX_WIDTH / width;
        assert!(
            height * width == buffer.as_ref().len() * T::MAX_WIDTH,
            "Buffer length must be a multiple of width"
        );
        Self {
            buffer,
            width,
            height,
            _layout: PhantomData,
            _element: PhantomData,
        }
    }
}

#[cfg(feature = "alloc")]
impl<T> GridBits<T, alloc::vec::Vec<T>, layout::RowMajor>
where
    T: BitOps + Default,
{
    /// Creates a new grid with the specified width and height, filled with a default value.
    ///
    /// This creates a grid with a row-major layout; see [`new_with_layout`][] to customize.
    ///
    /// [`new_with_layout`]: GridBits::new_with_layout
    ///
    /// ## Example
    ///
    /// ```rust
    /// use grixy::{core::Pos, buf::bits::GridBits, ops::GridRead};
    ///
    /// let grid = GridBits::<u8, _>::new(8, 1);
    /// assert_eq!(grid.get(Pos::new(0, 0)), Some(false));
    /// assert_eq!(grid.get(Pos::new(7, 0)), Some(false));
    /// assert_eq!(grid.get(Pos::new(8, 0)), None);
    /// assert_eq!(grid.get(Pos::new(0, 1)), None);
    /// ```
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = alloc::vec![T::default(); (width * height).div_ceil(T::MAX_WIDTH)];
        Self::from_buffer(buffer, width)
    }
}

#[cfg(feature = "alloc")]
impl<T, L> GridBits<T, alloc::vec::Vec<T>, L>
where
    T: BitOps,
    L: layout::Linear,
{
    /// Creates a new grid with the specified width, height, and layout, filled with the default value.
    ///
    /// # Example
    /// ```
    /// use grixy::{core::{Pos, RowMajor}, buf::bits::GridBits, ops::GridRead};
    ///
    /// let grid = GridBits::<u8, _, RowMajor>::new_with_layout(8, 1);
    /// assert_eq!(grid.get(Pos::new(0, 0)), Some(false));
    /// ```
    #[must_use]
    pub fn new_with_layout(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        let buffer = alloc::vec![T::default(); (width * height).div_ceil(T::MAX_WIDTH)];
        Self::from_buffer(buffer, width)
    }
}

impl<T, B, L> AsRef<[T]> for GridBits<T, B, L>
where
    T: BitOps,
    B: AsRef<[T]>,
    L: layout::Linear,
{
    fn as_ref(&self) -> &[T] {
        self.buffer.as_ref()
    }
}

impl<T, B, L> AsMut<[T]> for GridBits<T, B, L>
where
    T: BitOps,
    B: AsMut<[T]>,
    L: layout::Linear,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.buffer.as_mut()
    }
}

impl<T, B, L> GridBits<T, B, L>
where
    T: BitOps,
    B: AsRef<[T]>,
    L: layout::Linear,
{
    /// Consumes the `GridBits`, returning the underlying buffer, width, and height.
    #[must_use]
    pub fn into_inner(self) -> (B, usize, usize) {
        (self.buffer, self.width, self.height)
    }

    /// Returns an iterator over the bits of the grid.
    ///
    /// The iterator yields all items in the grid in the order defined by the layout.
    pub fn iter(&self) -> impl Iterator<Item = bool> + '_ {
        self.buffer.as_ref().iter().flat_map(|byte| {
            (0..T::MAX_WIDTH).map(move |bit_index| (byte.to_usize() >> bit_index) & 1 != 0)
        })
    }
}

impl<T, B, L> GridReadUnchecked for GridBits<T, B, L>
where
    T: BitOps,
    B: AsRef<[T]>,
    L: layout::Linear,
{
    type Element<'a>
        = bool
    where
        Self: 'a;

    type Layout = L;

    unsafe fn get_unchecked(&self, pos: Pos) -> Self::Element<'_> {
        let index = L::to_1d(pos, self.width);
        let (byte_index, bit_index) = (index / T::MAX_WIDTH, index % T::MAX_WIDTH);
        let byte = unsafe { self.buffer.as_ref().get_unchecked(byte_index) };
        (byte.to_usize() >> bit_index) & 1 != 0
    }

    unsafe fn iter_rect_unchecked(
        &self,
        bounds: crate::prelude::Rect,
    ) -> impl Iterator<Item = Self::Element<'_>> {
        if let Some(aligned) = L::slice_rect_aligned(self.as_ref(), self.size(), bounds) {
            let iter = aligned.iter().flat_map(|byte| {
                (0..T::MAX_WIDTH).map(move |bit_index| (byte.to_usize() >> bit_index) & 1 != 0)
            });
            internal::IterRect::Aligned(iter)
        } else {
            let iter = {
                let pos = Self::Layout::iter_pos(bounds);
                pos.map(move |pos| unsafe { self.get_unchecked(pos) })
            };
            internal::IterRect::Unaligned(iter)
        }
    }
}

impl<T, B, L> GridWriteUnchecked for GridBits<T, B, L>
where
    T: BitOps,
    B: AsMut<[T]>,
    L: layout::Linear,
{
    type Element = bool;
    type Layout = L;

    unsafe fn set_unchecked(&mut self, pos: Pos, value: bool) {
        let index = L::to_1d(pos, self.width);
        let (byte_index, bit_index) = (index / T::MAX_WIDTH, index % T::MAX_WIDTH);
        let byte = unsafe { self.buffer.as_mut().get_unchecked_mut(byte_index) };
        if value {
            *byte |= T::from_usize(1 << bit_index);
        } else {
            *byte &= !T::from_usize(1 << bit_index);
        }
    }
}

unsafe impl<T, B, L> TrustedSizeGrid for GridBits<T, B, L>
where
    T: BitOps,
    B: AsRef<[T]>,
    L: layout::Linear,
{
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::{
        buf::bits::GridBits,
        core::{GridError, Pos},
        ops::{GridRead, GridWrite, layout::RowMajor, unchecked::GridReadUnchecked as _},
    };

    #[test]
    fn impl_arr() {
        let data: [u8; 1] = [0b0000_0001];
        let grid = GridBits::<_, _, RowMajor>::from_buffer(data, 8);

        assert_eq!(grid.get(Pos::new(0, 0)), Some(true));
        assert_eq!(grid.get(Pos::new(1, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(8, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 1)), None);
    }

    #[test]
    fn arr_new() {
        let grid = GridBits::<u8, _, RowMajor>::new(8, 1);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(7, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(8, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 1)), None);
    }

    #[test]
    #[should_panic(expected = "Buffer length must be a multiple of width")]
    fn arr_new_panics() {
        let _ = GridBits::<u8, _, RowMajor>::new(9, 1);
    }

    #[test]
    fn impl_slice() {
        let data: [u8; 1] = [0b0000_0001];
        let grid = GridBits::<_, _, RowMajor>::from_buffer(data, 8);

        assert_eq!(grid.get(Pos::new(0, 0)), Some(true));
        assert_eq!(grid.get(Pos::new(1, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(8, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 1)), None);
    }

    #[test]
    fn new_with_layout() {
        let grid = GridBits::<u8, _, RowMajor>::new_with_layout(8, 1);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(7, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(8, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 1)), None);
    }

    #[test]
    fn impl_slice_mut() {
        let mut data: [u8; 1] = [0b0000_0001];
        let mut grid = GridBits::<_, _, RowMajor>::from_buffer(&mut data, 8);

        assert_eq!(grid.get(Pos::new(0, 0)), Some(true));
        assert_eq!(grid.get(Pos::new(1, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(8, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 1)), None);

        grid.set(Pos::new(1, 0), true).unwrap();
        assert_eq!(grid.get(Pos::new(1, 0)), Some(true));
        grid.set(Pos::new(0, 0), false).unwrap();
        assert_eq!(grid.get(Pos::new(0, 0)), Some(false));

        assert_eq!(grid.set(Pos::new(8, 0), true), Err(GridError));
    }

    #[test]
    fn vec_new() {
        let grid = GridBits::<_, alloc::vec::Vec<u8>, RowMajor>::new(8, 1);
        assert_eq!(grid.get(Pos::new(4, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(7, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(8, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 1)), None);
    }

    #[test]
    #[should_panic(expected = "Buffer length must be a multiple of width")]
    fn out_of_bounds() {
        let data: alloc::vec::Vec<u8> = alloc::vec![0b0001_0001];
        let _ = GridBits::<_, _, RowMajor>::from_buffer(data, 9);
    }

    #[test]
    fn into_inner() {
        let data: alloc::vec::Vec<u8> = alloc::vec![0b0001_0001];
        let grid = GridBits::<_, _, RowMajor>::from_buffer(data, 8);
        let (buffer, width, height) = grid.into_inner();
        assert_eq!(width, 8);
        assert_eq!(height, 1);
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer[0], 0b0001_0001);
    }

    #[test]
    fn iter() {
        let data: alloc::vec::Vec<u8> = alloc::vec![0b0001_0001];
        let grid = GridBits::<_, _, RowMajor>::from_buffer(data, 8);
        let mut iter = grid.iter();
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_ref() {
        let data: alloc::vec::Vec<u8> = alloc::vec![0b0001_0001];
        let grid = GridBits::<_, _, RowMajor>::from_buffer(data, 8);
        let slice: &[u8] = grid.as_ref();
        assert_eq!(slice.len(), 1);
        assert_eq!(slice[0], 0b0001_0001);
    }

    #[test]
    fn as_mut() {
        let data: alloc::vec::Vec<u8> = alloc::vec![0b0001_0001];
        let mut grid = GridBits::<_, _, RowMajor>::from_buffer(data, 8);
        let slice: &mut [u8] = grid.as_mut();
        assert_eq!(slice.len(), 1);
        assert_eq!(slice[0], 0b0001_0001);
        slice[0] = 0b1111_1011;
        assert_eq!(grid.get(Pos::new(0, 0)), Some(true));
        assert_eq!(grid.get(Pos::new(1, 0)), Some(true));
        assert_eq!(grid.get(Pos::new(2, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(3, 0)), Some(true));
    }

    #[test]
    fn bits_from_slice_is_grid_read() {
        let data = &[0b0000_0001u8];
        let grid = GridBits::<_, _, RowMajor>::from_buffer(data, 8);
        assert!(unsafe { grid.get_unchecked(Pos::new(0, 0)) });
        assert_eq!(grid.get(Pos::new(0, 0)), Some(true));
    }
}
