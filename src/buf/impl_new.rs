use core::marker::PhantomData;

use ixy::index::Layout;

use crate::{
    buf::GridBuf,
    core::{ColMajor, GridError, RowMajor},
};

impl<T, B, L> GridBuf<T, B, L>
where
    B: AsRef<[T]>,
    L: Layout,
{
    /// Creates a `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// ## Errors
    ///
    /// Returns an error if the buffer size does not match the expected size.
    pub fn with_buffer(buffer: B, width: usize, height: usize) -> Result<Self, GridError> {
        let expected_size = width * height;
        if buffer.as_ref().len() != expected_size {
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
            buffer.as_ref().len(),
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
}

impl<T, B> GridBuf<T, B, RowMajor>
where
    B: AsRef<[T]>,
{
    /// Creates a `GridBuf` using an existing data buffer, specifying the grid dimensions.
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

impl<T, B> GridBuf<T, B, ColMajor>
where
    B: AsRef<[T]>,
{
    /// Creates a `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// The data buffer is expected to be in [`ColMajor`] order.
    ///
    /// ## Errors
    ///
    /// Returns an error if the buffer size does not match the expected size.
    pub fn with_buffer_col_major(
        buffer: B,
        width: usize,
        height: usize,
    ) -> Result<Self, GridError> {
        Self::with_buffer(buffer, width, height)
    }

    /// Creates a new `GridBuf` using an existing data buffer, specifying the grid dimensions.
    ///
    /// The data buffer is expected to be in [`ColMajor`] order.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that the buffer is large enough to hold `width * height` elements.
    pub unsafe fn with_buffer_col_major_unchecked(buffer: B, width: usize, height: usize) -> Self {
        unsafe { Self::with_buffer_unchecked(buffer, width, height) }
    }
}
