//! Defines a trait for generating 2-dimensional patterns, such as for noise generation.

use grixy::prelude::*;

pub trait Pattern2D {
    /// Generates a 2-dimensional pattern value at the given position.
    fn generate(&self, pos: Pos) -> f32;
}

pub struct Checkerboard {
    pub even: f32,
    pub odd: f32,
}

impl Pattern2D for Checkerboard {
    fn generate(&self, pos: Pos) -> f32 {
        if (pos.x + pos.y) % 2 == 0 {
            self.even
        } else {
            self.odd
        }
    }
}

/// Wrapper struct for types implementing [`Pattern2D`].
pub struct Pattern2DGrid<T> {
    pattern: T,
}

impl<T> GridBase for Pattern2DGrid<T> where T: Pattern2D {}

impl<T> GridRead for Pattern2DGrid<T>
where
    T: Pattern2D,
{
    type Element<'a>
        = f32
    where
        Self: 'a;

    type Layout = RowMajor;

    fn get(&self, pos: Pos) -> Option<Self::Element<'_>> {
        Some(self.pattern.generate(pos))
    }
}

fn main() {
    let pattern = Checkerboard {
        even: 1.0,
        odd: 0.0,
    };

    let grid = Pattern2DGrid { pattern };

    // Example usage: print the generated values in a 4x4 grid.
    for y in 0..4 {
        for x in 0..4 {
            let pos = Pos::new(x, y);
            print!("{:.1} ", grid.get(pos).unwrap());
        }
        println!();
    }
}
