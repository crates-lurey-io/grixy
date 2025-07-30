use crate::core::{Pos, Size};

/// A bounded grid type that provides methods to access its dimensions.
///
/// ## Safety
///
/// If the dimensions provide are not accurate, it may lead to _[undefined behavior][]_.
///
/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
pub unsafe trait TrustedSizeGrid {
    /// Returns the width of the grid, in columns.
    fn width(&self) -> usize;

    /// Returns the height of the grid, in rows.
    fn height(&self) -> usize;

    /// Returns the size of the grid.
    fn size(&self) -> Size {
        Size {
            width: self.width(),
            height: self.height(),
        }
    }

    /// Returns whether the given coordinates are valid for this grid.
    fn contains(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }

    /// Returns whether the given position is valid for this grid.
    fn contains_pos(&self, pos: Pos) -> bool {
        self.contains(pos.x, pos.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestGrid {
        width: usize,
        height: usize,
    }

    unsafe impl TrustedSizeGrid for TestGrid {
        fn width(&self) -> usize {
            self.width
        }

        fn height(&self) -> usize {
            self.height
        }
    }

    #[test]
    fn size() {
        let grid = TestGrid {
            width: 10,
            height: 5,
        };
        assert_eq!(
            grid.size(),
            Size {
                width: 10,
                height: 5
            }
        );
    }

    #[test]
    fn contains_true() {
        let grid = TestGrid {
            width: 10,
            height: 5,
        };
        assert!(grid.contains(5, 3));
        assert!(grid.contains_pos(Pos::new(5, 3)));
    }

    #[test]
    fn contains_false_x() {
        let grid = TestGrid {
            width: 10,
            height: 5,
        };
        assert!(!grid.contains(10, 3));
        assert!(!grid.contains_pos(Pos::new(10, 3)));
    }

    #[test]
    fn contains_false_y() {
        let grid = TestGrid {
            width: 10,
            height: 5,
        };
        assert!(!grid.contains(5, 5));
        assert!(!grid.contains_pos(Pos::new(5, 5)));
    }
}
