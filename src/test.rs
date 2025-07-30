//! Testing fixtures internal to the crate.

extern crate alloc;

use core::ops::Add;

use alloc::{vec, vec::Vec};

use crate::{
    core::GridError,
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
    type Element = T;

    fn get(&self, pos: crate::core::Pos) -> Option<&Self::Element> {
        if pos.x < self.width && pos.y < self.height {
            Some(&self.cells[pos.y * self.width + pos.x])
        } else {
            None
        }
    }
}

impl<T> GridWrite for NaiveGrid<T> {
    type Element = T;

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

/// A blend function that adds two elements together.
pub fn blend_add<S, T>(src: S, dst: T) -> T
where
    S: Add<T, Output = T>,
{
    src + dst
}
