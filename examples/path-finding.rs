//! Implements a simple pathfinding algorithm using a grid buffer.

use std::collections::VecDeque;

use grixy::{ops::ExactSizeGrid, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Walkable,
    Blocked,
}

fn find_path<'a>(
    grid: &'a (impl GridRead<Element<'a> = Tile> + ExactSizeGrid),
    start: Pos,
    end: Pos,
) -> Option<impl Iterator<Item = Pos>> {
    // A simple breadth-first search (BFS) algorithm to find a path.
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut visited = GridBuf::new_filled(grid.size().width, grid.size().height, false);

    let _ = visited.set(start, true);
    let mut came_from = GridBuf::new_filled(grid.size().width, grid.size().height, None::<Pos>);

    while let Some(current) = queue.pop_front() {
        if current == end {
            // Reconstruct the path.
            let mut path = Vec::new();
            let mut pos = end;

            while pos != start {
                path.push(pos);
                pos = came_from.get(pos).unwrap().unwrap();
            }
            path.push(start);
            path.reverse();
            return Some(path.into_iter());
        }

        // Compute neighbors manually (up, down, left, right)
        let directions: [(i32, i32); 4] = [
            (0, -1), // up
            (0, 1),  // down
            (-1, 0), // left
            (1, 0),  // right
        ];
        for (dx, dy) in &directions {
            let new_x = i32::try_from(current.x).unwrap() + dx;
            let new_y = i32::try_from(current.y).unwrap() + dy;
            if new_x >= 0 && new_y >= 0 {
                let neighbor = Pos {
                    x: usize::try_from(new_x).unwrap(),
                    y: usize::try_from(new_y).unwrap(),
                };
                // Check bounds
                if neighbor.x < grid.size().width
                    && neighbor.y < grid.size().height
                    && grid.get(neighbor) == Some(Tile::Walkable)
                    && !visited.get(neighbor).unwrap()
                {
                    let _ = visited.set(neighbor, true);
                    let _ = came_from.set(neighbor, Some(current));
                    queue.push_back(neighbor);
                }
            }
        }
    }

    None
}

fn main() {
    // Example of an entire grid filled with walkable tiles.
    let grid = GridBuf::new_filled(10, 10, Tile::Walkable);
    let start = Pos::new(0, 0);
    let end = Pos::new(9, 9);
    if let Some(path) = find_path(&grid.copied(), start, end) {
        println!("Path found from {start:?} to {end:?}:");
        for pos in path {
            println!("Path step: {pos:?}");
        }
    } else {
        println!("No path found");
    }

    println!("--------------------------------------------------");

    // Example of a grid with some blocked tiles.
    #[rustfmt::skip]
    let grid_with_blocks = GridBuf::<_, _, RowMajor>::from_buffer(&[
        0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
        0, 1, 1, 1, 1, 1, 0, 0, 1, 0,
        0, 0, 0, 1, 0, 0, 0, 1, 1, 0,
        0, 1, 1, 1, 0, 1, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 1, 0, 1, 1, 0,
        0, 1, 1, 1, 0, 1, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 1, 0, 1, 1, 0,
        0, 1, 1, 1, 0, 1, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 1, 1, 1, 1, 0,
        0, 1, 1, 1, 0, 1, 0, 0, 0, 0,
    ], 10).map(|tile| {
        if tile == &1 {
            Tile::Blocked
        } else {
            Tile::Walkable
        }
    });
    let start = Pos::new(0, 0);
    let end = Pos::new(9, 9);
    if let Some(path) = find_path(&grid_with_blocks, start, end) {
        println!("Path found from {start:?} to {end:?}:");
        for pos in path {
            println!("Path step: {pos:?}");
        }
    } else {
        println!("No path found");
    }
}
