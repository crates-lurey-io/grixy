//! Provides a single-bit, 2D grid data structure backed by a linear buffer.
//!
//! The main type, [`GridBits`], is highly generic, allowing it to act as a view over any buffer
//! that can be treated as a slice.
//!
//! ## Convenience Types
//!
//! For ease of use, several type aliases are provided for common use cases:
//! - [`VecBits`]: An owned grid backed by a `Vec<u8>`
//! - [`ArrayBits`]: An owned grid backed by a fixed-size array, `[u8; N]`.
//! - [`SliceBits`]: A read-only, borrowed view over an existing slice.
//! - [`SliceMutBits`]: A mutable, borrowed view over an existing slice.
//!
//! # Examples
//!
//! Creating an owned `VecGridBits` and accessing an element:
//! ```
//! use grixy::{core::{Pos, RowMajor}, buf::bits::VecBits};
//!
//! let grid = VecBits::<u8, RowMajor>::new(8, 1);
//! assert_eq!(grid.get(Pos::new(3, 0)), Some(false));
//! ```

mod ops;

use core::marker::PhantomData;

use ixy::index::{Layout, RowMajor};
pub use ops::BitOps;

use crate::{core::GridError, core::Pos};

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
    L: Layout,
{
    buffer: B,
    width: usize,
    height: usize,
    _element: PhantomData<T>,
    _layout: PhantomData<L>,
}

impl<T, B> GridBits<T, B, RowMajor>
where
    T: BitOps,
    B: AsRef<[T]>,
{
    /// Creates a `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// The maximum width that can be used is determined by [`BitOps::MAX_WIDTH`].
    ///
    /// The data buffer is expected to be in [`RowMajor`] order.
    ///
    /// ## Errors
    ///
    /// Returns an error if the buffer size does not match the expected size.
    pub fn with_buffer_row_major(
        buffer: B,
        width: usize,
        height: usize,
    ) -> Result<Self, GridError> {
        Self::with_buffer(buffer, width, height)
    }

    /// Creates a new `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// The data buffer is expected to be in [`RowMajor`] order.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that the buffer is large enough to hold `width * height` elements.
    pub unsafe fn with_buffer_row_major_unchecked(buffer: B, width: usize, height: usize) -> Self {
        unsafe { Self::with_buffer_unchecked(buffer, width, height) }
    }
}

impl<T, B, L> GridBits<T, B, L>
where
    T: BitOps,
    B: AsRef<[T]>,
    L: Layout,
{
    /// Creates a `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// The maximum width that can be used is determined by [`BitOps::MAX_WIDTH`].
    ///
    /// ## Errors
    ///
    /// Returns an error if the buffer size does not match the expected size.
    pub fn with_buffer(buffer: B, width: usize, height: usize) -> Result<Self, GridError> {
        let expected_size = width * height;
        if buffer.as_ref().len() * T::MAX_WIDTH < expected_size || width > T::MAX_WIDTH {
            return Err(GridError);
        }
        Ok(unsafe { Self::with_buffer_unchecked(buffer, width, height) })
    }

    /// Creates a new `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that the buffer is large enough to hold `width * height` elements.
    pub unsafe fn with_buffer_unchecked(buffer: B, width: usize, height: usize) -> Self {
        debug_assert_eq!(
            buffer.as_ref().len() * T::MAX_WIDTH,
            width * height,
            "Buffer size does not match grid dimensions"
        );
        Self {
            buffer,
            width,
            height,
            _element: PhantomData,
            _layout: PhantomData,
        }
    }

    /// Returns a reference of the element at the specified position.'
    ///
    /// If the position is out of bounds, returns `None`.
    pub fn get(&self, pos: Pos) -> Option<bool> {
        if pos.x < self.width && pos.y < self.height {
            let index = L::to_1d(pos, self.width).index;
            let (byte_index, bit_index) = (index / T::MAX_WIDTH, index % T::MAX_WIDTH);
            self.buffer
                .as_ref()
                .get(byte_index)
                .map(|byte| (byte.to_usize() >> bit_index) & 1 != 0)
        } else {
            None
        }
    }

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

/// A 2-dimensional grid implemented by a fixed-size array buffer of bytes.
///
/// This is a convenience type for using arrays as the underlying buffer.
///
/// ## Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
pub type ArrayBits<T, const N: usize, L> = GridBits<T, [T; N], L>;
impl<T, const N: usize, L> GridBits<T, [T; N], L>
where
    T: BitOps,
    L: Layout,
{
    /// Creates a new `GridBits` backed by a fixed-size array with the specified width and height.
    ///
    /// Each element is initialized to `false`.
    ///
    /// ## Panics
    ///
    /// Panics if the buffer size does not match the expected size.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_filled(width, height, false)
    }

    /// Creates a new `GridBits` backed by a fixed-size array with the specified width and height.
    ///
    /// Each element is initialized to the provided value.
    ///
    /// ## Panics
    ///
    /// Panics if the buffer size does not match the expected size.
    #[must_use]
    pub fn new_filled(width: usize, height: usize, value: bool) -> Self {
        let size = width * height;
        assert!(
            size.div_ceil(T::MAX_WIDTH) <= N,
            "Buffer size does not match grid dimensions"
        );
        let mut buffer = [T::from_usize(0); N];
        for i in 0..size {
            if value {
                buffer[i / T::MAX_WIDTH] |= T::from_usize(1 << (i % T::MAX_WIDTH));
            }
        }
        unsafe { Self::with_buffer_unchecked(buffer, width, height) }
    }
}

impl<T, B, L> GridBits<T, B, L>
where
    T: BitOps,
    B: AsMut<[T]>,
    L: Layout,
{
    /// Sets the bit at the specified position, if it is within bounds.
    pub fn set(&mut self, pos: Pos, value: bool) -> Option<()> {
        if pos.x < self.width && pos.y < self.height {
            let index = L::to_1d(pos, self.width).index;
            let (byte_index, bit_index) = (index / T::MAX_WIDTH, index % T::MAX_WIDTH);
            let byte = self.buffer.as_mut().get_mut(byte_index)?;
            if value {
                *byte |= T::from_usize(1 << bit_index);
            } else {
                *byte &= !T::from_usize(1 << bit_index);
            }
            Some(())
        } else {
            None
        }
    }
}

/// A 2-dimensional grid implemented by a slice buffer of bytes.
///
/// This is a convenience type for using slices as the underlying buffer.
///
/// ## Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
pub type SliceBits<'a, T, L> = GridBits<T, &'a [T], L>;

/// A 2-dimensional grid implemented by a mutable slice buffer of bytes.
///
/// This is a convenience type for using mutable slices as the underlying buffer.
///
/// ## Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
pub type SliceMutBits<'a, T, L> = GridBits<T, &'a mut [T], L>;

/// A 2-dimensional grid implemented by a vector buffer of bytes.
///
/// This is a convenience type for using `Vec` as the underlying buffer.
///
/// ## Layout
///
/// The grid is stored in a linear buffer, with elements accessed in an order defined by [`Layout`].
#[cfg(feature = "alloc")]
pub type VecBits<T, L> = GridBits<T, alloc::vec::Vec<T>, L>;

#[cfg(feature = "alloc")]
impl<T, L> GridBits<T, alloc::vec::Vec<T>, L>
where
    T: BitOps,
    L: Layout,
{
    /// Creates a new `GridBits` backed by a `Vec` with the specified width and height.
    ///
    /// Each element is initialized to `false`.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_filled(width, height, false)
    }

    /// Creates a new `GridBits` backed by a `Vec` with the specified width and height.
    ///
    /// Each element is initialized to the provided value.
    #[must_use]
    pub fn new_filled(width: usize, height: usize, value: bool) -> Self {
        let size = width * height;
        let mut buffer = alloc::vec![T::from_usize(0); size.div_ceil(T::MAX_WIDTH)];
        for i in 0..size {
            if value {
                buffer[i / T::MAX_WIDTH] |= T::from_usize(1 << (i % T::MAX_WIDTH));
            }
        }
        unsafe { Self::with_buffer_unchecked(buffer, width, height) }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use ixy::index::RowMajor;

    use crate::{
        buf::bits::{ArrayBits, SliceBits, SliceMutBits, VecBits},
        core::Pos,
    };

    #[test]
    fn impl_arr() {
        let data: [u8; 1] = [0b0000_0001];
        let grid = ArrayBits::<_, 1, RowMajor>::with_buffer(data, 8, 1).unwrap();

        assert_eq!(grid.get(Pos::new(0, 0)), Some(true));
        assert_eq!(grid.get(Pos::new(1, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(8, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 1)), None);
    }

    #[test]
    fn arr_new() {
        let grid = ArrayBits::<u8, 1, RowMajor>::new(8, 1);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(7, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(8, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 1)), None);
    }

    #[test]
    fn arr_new_filled() {
        let grid = ArrayBits::<u8, 1, RowMajor>::new_filled(8, 1, true);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(true));
        assert_eq!(grid.get(Pos::new(7, 0)), Some(true));
        assert_eq!(grid.get(Pos::new(8, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 1)), None);
    }

    #[test]
    #[should_panic(expected = "Buffer size does not match grid dimensions")]
    fn arr_new_panics() {
        let _ = ArrayBits::<u8, 1, RowMajor>::new(9, 1);
    }

    #[test]
    fn impl_slice() {
        let data: [u8; 1] = [0b0000_0001];
        let grid = SliceBits::with_buffer_row_major(&data, 8, 1).unwrap();

        assert_eq!(grid.get(Pos::new(0, 0)), Some(true));
        assert_eq!(grid.get(Pos::new(1, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(8, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 1)), None);
    }

    #[test]
    fn impl_slice_mut() {
        let mut data: [u8; 1] = [0b0000_0001];
        let mut grid = SliceMutBits::with_buffer_row_major(&mut data, 8, 1).unwrap();

        assert_eq!(grid.get(Pos::new(0, 0)), Some(true));
        assert_eq!(grid.get(Pos::new(1, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(8, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 1)), None);

        grid.set(Pos::new(1, 0), true).unwrap();
        assert_eq!(grid.get(Pos::new(1, 0)), Some(true));
        grid.set(Pos::new(0, 0), false).unwrap();
        assert_eq!(grid.get(Pos::new(0, 0)), Some(false));

        assert_eq!(grid.set(Pos::new(8, 0), true), None);
    }

    #[test]
    fn vec_new() {
        let grid = VecBits::<u8, RowMajor>::new(8, 1);
        assert_eq!(grid.get(Pos::new(4, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(7, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(8, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 1)), None);
    }

    #[test]
    fn vec_new_filled() {
        let grid = VecBits::<u8, RowMajor>::new_filled(8, 1, true);
        assert_eq!(grid.get(Pos::new(4, 0)), Some(true));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "Buffer size does not match grid dimensions")]
    fn with_buffer_unchecked_panics() {
        let data: alloc::vec::Vec<u8> = alloc::vec![0b0001_0001];
        // width * height = 6
        // but data.len() = 5
        let _ = unsafe { VecBits::<_, RowMajor>::with_buffer_unchecked(data, 2, 3) };
    }

    #[test]
    fn out_of_bounds() {
        let data: alloc::vec::Vec<u8> = alloc::vec![0b0001_0001];
        let grid = VecBits::with_buffer_row_major(data, 8, 2);
        assert!(grid.is_err());
    }

    #[test]
    fn into_inner() {
        let data: alloc::vec::Vec<u8> = alloc::vec![0b0001_0001];
        let grid = VecBits::with_buffer_row_major(data, 8, 1).unwrap();
        let (buffer, width, height) = grid.into_inner();
        assert_eq!(width, 8);
        assert_eq!(height, 1);
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer[0], 0b0001_0001);
    }

    #[test]
    fn iter() {
        let data: alloc::vec::Vec<u8> = alloc::vec![0b0001_0001];
        let grid = VecBits::with_buffer_row_major(data, 8, 1).unwrap();
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
    fn with_buffer_row_major_unchecked() {
        let data: alloc::vec::Vec<u8> = alloc::vec![0b0001_0001];
        let grid = unsafe { VecBits::<u8, RowMajor>::with_buffer_row_major_unchecked(data, 8, 1) };
        assert_eq!(grid.get(Pos::new(0, 0)), Some(true));
        assert_eq!(grid.get(Pos::new(1, 0)), Some(false));
        assert_eq!(grid.get(Pos::new(8, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 1)), None);
    }

    #[test]
    fn with_buffer_width_exceeded_err() {
        let data: alloc::vec::Vec<u8> = alloc::vec![0b0001_0001];
        let grid = VecBits::with_buffer_row_major(data, 8, 2);
        assert!(grid.is_err());
    }
}
