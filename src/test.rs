//! Testing fixtures internal to the crate.

extern crate alloc;

use alloc::{vec, vec::Vec};

use crate::{
    core::{GridError, RowMajor},
    ops::{GridRead, GridWrite},
};

/// A grid implementation that does not optimize any operations.
pub struct NaiveGrid<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> NaiveGrid<T> {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Copy,
    {
        let cells = vec![T::default(); width * height];
        Self {
            cells,
            width,
            height,
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn with_cells(width: usize, height: usize, cells: impl IntoIterator<Item = T>) -> Self {
        let cells: Vec<T> = cells.into_iter().collect();
        assert_eq!(
            cells.len(),
            width * height,
            "Cells length does not match grid size"
        );
        Self {
            cells,
            width,
            height,
        }
    }
}

impl<T> GridRead for NaiveGrid<T> {
    type Element<'a>
        = &'a T
    where
        Self: 'a;

    type Layout = RowMajor;

    fn get(&self, pos: crate::core::Pos) -> Option<Self::Element<'_>> {
        if pos.x < self.width && pos.y < self.height {
            Some(&self.cells[pos.y * self.width + pos.x])
        } else {
            None
        }
    }
}

impl<T> GridWrite for NaiveGrid<T> {
    type Element = T;
    type Layout = RowMajor;

    fn set(&mut self, pos: crate::core::Pos, value: Self::Element) -> Result<(), GridError> {
        if pos.x < self.width && pos.y < self.height {
            self.cells[pos.y * self.width + pos.x] = value;
            Ok(())
        } else {
            Err(GridError)
        }
    }
}

impl<T> IntoIterator for NaiveGrid<T> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Cells length does not match grid size")]
    fn test_with_cells_panics_on_invalid_length() {
        let _grid = NaiveGrid::<u8>::with_cells(2, 2, vec![1, 2, 3]);
    }
}
