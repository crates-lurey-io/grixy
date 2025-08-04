# Todos

## Immediate

_Nothing as of now_.

## 0.6.0 (Beta)

- [ ] Make more descriptive `GridError` enum variants
- [ ] Add rotate
- [ ] Add flip
- [ ] Add transpose
- [ ] Implement some nice debug string representations/formats
- [ ] Implement `Index|IndexMut<Pos>`
- [ ] Implement `Clone` when buffer is clonable
- [ ] Rename `collect` to `flatten`
- [ ] Add auto-trait with `clear`, `fill`, and related
- [ ] Add an example of noise generation (`examples/noise.rs`)
- [ ] Add an example of pathfinding (`examples/pathfinding.rs`)
- [ ] Add an example of a grid with a custom layout (`examples/z-order.rs`)
- [ ] Benchmark heavily and adjust as needed

## 0.6.0 (Stable)

_Nothing as of now_.

## Future

- [ ] Ranges?
- [ ] Optional support for `defmt`
- [ ] Add `size_hint` (untrusted)
- [ ] Add `GridMut` trait for mutable grid buffer operations
  - [ ] `resize`
  - [ ] `push_{row,col,block}`
  - [ ] `pop_*`
  - [ ] `insert_*`
  - [ ] `remove_*`
  - [ ] `expand_*`
  - [ ] `shrink_*`
- [ ] Add `grixy::buf::macros` module with macros for creating grids
- [ ] Add iterators for rows and columns
- [ ] Add a sparse (`BTree`-backed) grid buffer
- [ ] Noise-generation crate that uses `grixy` to represent a grid

## Speculative

- [ ] Support `Into/TryInto<usize>` to support arbitrary dimensions for Grids
- [ ] Support a `::diff` module for comparing grids
- [ ] Investigate generic parallelism for grid operations
- [ ] Path operations (in `Ixy`), potentially including flood-fill
- [ ] Directions and octants (in `Ixy`)
- [ ] Geometry (in `Ixy`)
