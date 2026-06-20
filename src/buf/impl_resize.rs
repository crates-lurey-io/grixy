#[cfg(feature = "alloc")]
extern crate alloc;

use crate::{buf::GridBuf, core::Pos, ops::layout};

#[cfg(feature = "alloc")]
impl<T, L> GridBuf<T, alloc::vec::Vec<T>, L>
where
    T: Copy + Default,
    L: layout::Linear,
{
    /// Resizes the grid to the new dimensions.
    ///
    /// Content in the overlap between the old and new dimensions is preserved.
    /// New cells are initialized to `T::default()`.
    ///
    /// If both dimensions shrink, excess cells are dropped.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use grixy::prelude::*;
    ///
    /// let mut grid = GridBuf::new_filled(4, 4, 1u8);
    /// grid[Pos::new(0, 0)] = 99;
    /// grid.resize(6, 6);
    ///
    /// assert_eq!(grid.get(Pos::new(0, 0)), Some(&99)); // preserved
    /// assert_eq!(grid.get(Pos::new(5, 5)), Some(&0));  // new, default
    /// ```
    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        if new_width == self.width && new_height == self.height {
            return;
        }

        let copy_w = self.width.min(new_width);
        let copy_h = self.height.min(new_height);
        let mut new_buf = alloc::vec![T::default(); new_width * new_height];

        for row in 0..copy_h {
            for col in 0..copy_w {
                let src_pos = Pos::new(col, row);
                let dst_idx = L::pos_to_index(src_pos, new_width);
                let src_idx = L::pos_to_index(src_pos, self.width);
                new_buf[dst_idx] = self.buffer[src_idx];
            }
        }

        self.buffer = new_buf;
        self.width = new_width;
        self.height = new_height;
    }

    /// Resizes the grid, filling new cells with `value` instead of `Default`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use grixy::prelude::*;
    ///
    /// let mut grid = GridBuf::new_filled(2, 2, 1u8);
    /// grid.resize_filled(4, 4, 42);
    ///
    /// assert_eq!(grid.get(Pos::new(0, 0)), Some(&1));  // preserved
    /// assert_eq!(grid.get(Pos::new(3, 3)), Some(&42)); // new, filled with 42
    /// ```
    pub fn resize_filled(&mut self, new_width: usize, new_height: usize, value: T) {
        if new_width == self.width && new_height == self.height {
            return;
        }

        let copy_w = self.width.min(new_width);
        let copy_h = self.height.min(new_height);
        let mut new_buf = alloc::vec![value; new_width * new_height];

        for row in 0..copy_h {
            for col in 0..copy_w {
                let src_pos = Pos::new(col, row);
                let dst_idx = L::pos_to_index(src_pos, new_width);
                let src_idx = L::pos_to_index(src_pos, self.width);
                new_buf[dst_idx] = self.buffer[src_idx];
            }
        }

        self.buffer = new_buf;
        self.width = new_width;
        self.height = new_height;
    }
}

#[cfg(test)]
#[cfg(feature = "alloc")]
mod tests {
    extern crate alloc;

    use crate::{
        buf::GridBuf,
        core::Pos,
        ops::{ExactSizeGrid as _, GridRead as _, layout::RowMajor},
    };

    #[test]
    fn resize_grow() {
        let mut grid = GridBuf::<_, _, RowMajor>::new_filled(2, 2, 1u8);
        grid[Pos::new(0, 0)] = 99;
        grid.resize(4, 4);

        assert_eq!(grid.width(), 4);
        assert_eq!(grid.height(), 4);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&99)); // preserved
        assert_eq!(grid.get(Pos::new(1, 1)), Some(&1)); // preserved
        assert_eq!(grid.get(Pos::new(3, 3)), Some(&0)); // new, default
    }

    #[test]
    fn resize_shrink() {
        let mut grid = GridBuf::<_, _, RowMajor>::new_filled(4, 4, 1u8);
        grid[Pos::new(3, 3)] = 99;
        grid.resize(2, 2);

        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&1)); // preserved
        assert_eq!(grid.get(Pos::new(1, 1)), Some(&1)); // preserved
        assert_eq!(grid.get(Pos::new(3, 3)), None); // out of bounds
    }

    #[test]
    fn resize_same_size() {
        let mut grid = GridBuf::<_, _, RowMajor>::new_filled(3, 3, 1u8);
        grid[Pos::new(0, 0)] = 99;
        grid.resize(3, 3);

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&99)); // unchanged
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
    }

    #[test]
    fn resize_grow_width_shrink_height() {
        let mut grid = GridBuf::<_, _, RowMajor>::new_filled(3, 3, 1u8);
        grid[Pos::new(2, 2)] = 99;
        grid.resize(5, 2);

        assert_eq!(grid.width(), 5);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.get(Pos::new(2, 2)), None); // out of bounds (height shrunk)
        assert_eq!(grid.get(Pos::new(4, 1)), Some(&0)); // new, default
    }

    #[test]
    fn resize_filled_grow() {
        let mut grid = GridBuf::<_, _, RowMajor>::new_filled(2, 2, 1u8);
        grid.resize_filled(4, 4, 42);

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&1)); // preserved
        assert_eq!(grid.get(Pos::new(3, 3)), Some(&42)); // new, filled with 42
    }
}
