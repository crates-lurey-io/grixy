use crate::{
    core::{GridError, Pos},
    grid::{BoundedGrid, GridBase},
};

/// Write elements to a 2-dimensional grid position.
pub trait GridWrite: GridBase {
    /// Sets the element at a specified position.
    ///
    /// ## Errors
    ///
    /// Returns an error if the position is out of bounds.
    fn set(&mut self, pos: Pos, value: Self::Element) -> Result<(), GridError>;
}

/// Write elements to a 2-dimensional grid position without bounds checking.
pub trait GridWriteUnchecked: GridBase {
    /// Sets the element at a specified position without bounds checking.
    ///
    /// ## Safety
    ///
    /// Calling this method with an out-of-bounds position is _[undefined behavior][]_.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    unsafe fn set_unchecked(&mut self, pos: Pos, value: Self::Element);
}

/// Automatically implement `GridWrite` when `GridWriteUnchecked` + `BoundedGrid` are implemented.
impl<T: GridWriteUnchecked + BoundedGrid> GridWrite for T {
    fn set(&mut self, pos: Pos, value: Self::Element) -> Result<(), GridError> {
        if self.contains_pos(pos) {
            unsafe {
                self.set_unchecked(pos, value);
                Ok(())
            }
        } else {
            Err(GridError)
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;

    struct TestGrid {
        grid: [[u8; 3]; 3],
    }

    impl GridBase for TestGrid {
        type Element = u8;
    }

    unsafe impl BoundedGrid for TestGrid {
        fn width(&self) -> usize {
            3
        }

        fn height(&self) -> usize {
            3
        }
    }

    impl GridWriteUnchecked for TestGrid {
        unsafe fn set_unchecked(&mut self, pos: Pos, value: Self::Element) {
            self.grid[pos.y][pos.x] = value;
        }
    }

    #[test]
    fn test_set_ok() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 1, y: 1 };
        grid.set(pos, 42).unwrap();
        assert_eq!(grid.grid[1][1], 42);
    }

    #[test]
    fn test_set_out_of_bounds_x() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 3, y: 1 };
        assert!(grid.set(pos, 42).is_err());
    }

    #[test]
    fn test_set_out_of_bounds_y() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 1, y: 3 };
        assert!(grid.set(pos, 42).is_err());
    }

    #[test]
    fn test_set_unchecked_in_bounds() {
        let mut grid = TestGrid { grid: [[0; 3]; 3] };
        let pos = Pos { x: 2, y: 2 };
        unsafe {
            grid.set_unchecked(pos, 99);
        }
        assert_eq!(grid.grid[2][2], 99);
    }
}
