use crate::ops::ExactSizeGrid;

/// A grid that reports an accuate size using `size_hint()`.
///
/// ## Safety
///
/// If the dimensions provide are not accurate, it may lead to _[undefined behavior][]_.
///
/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
pub unsafe trait TrustedSizeGrid: ExactSizeGrid {}

#[cfg(test)]
mod tests {
    use crate::core::{Pos, Size};

    use super::*;

    struct TestGrid {
        width: usize,
        height: usize,
    }

    impl ExactSizeGrid for TestGrid {
        fn width(&self) -> usize {
            self.width
        }

        fn height(&self) -> usize {
            self.height
        }
    }

    unsafe impl TrustedSizeGrid for TestGrid {}

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
        assert!(grid.contains(Pos::new(5, 3)));
    }

    #[test]
    fn contains_false_x() {
        let grid = TestGrid {
            width: 10,
            height: 5,
        };
        assert!(!grid.contains(Pos::new(10, 3)));
    }

    #[test]
    fn contains_false_y() {
        let grid = TestGrid {
            width: 10,
            height: 5,
        };
        assert!(!grid.contains(Pos::new(5, 5)));
    }
}
