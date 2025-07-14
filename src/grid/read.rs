use crate::{
    core::Pos,
    grid::{BoundedGrid, GridBase},
};

/// Read elements from a 2-dimensional grid position.
pub trait GridRead: GridBase {
    /// Returns a reference to an element at a specified position.
    ///
    /// If the position is out of bounds, it returns `None`.
    fn get(&self, pos: Pos) -> Option<&Self::Element>;
}

/// Read elements from a 2-dimensional grid position without bounds checking.
pub trait GridReadUnchecked: GridBase {
    /// Returns a reference to an element, without doing bounds checking.
    ///
    /// ## Safety
    ///
    /// Calling this method with an out-of-bounds position is _[undefined behavior][]_.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    unsafe fn get_unchecked(&self, pos: Pos) -> &Self::Element;
}

/// Automatically implement `GridRead` when `GridReadUnchecked` + `BoundedGrid` are implemented.
impl<T: GridReadUnchecked + BoundedGrid> GridRead for T {
    fn get(&self, pos: Pos) -> Option<&Self::Element> {
        if self.contains_pos(pos) {
            Some(unsafe { self.get_unchecked(pos) })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
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

    impl GridReadUnchecked for TestGrid {
        unsafe fn get_unchecked(&self, pos: Pos) -> &Self::Element {
            &self.grid[pos.y][pos.x]
        }
    }

    #[test]
    fn test_get_ok() {
        let grid = TestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        assert_eq!(grid.get(Pos::new(1, 1)), Some(&5));
    }

    #[test]
    fn test_get_out_of_bounds_x() {
        let grid = TestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        assert_eq!(grid.get(Pos::new(3, 1)), None);
    }

    #[test]
    fn test_get_out_of_bounds_y() {
        let grid = TestGrid {
            grid: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        assert_eq!(grid.get(Pos::new(1, 3)), None);
    }
}
