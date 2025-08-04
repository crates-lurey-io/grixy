# Todos

## Immediate

_Nothing as of now_.

## 0.6.0 (Beta)

- [x] Add `size_hint` (untrusted)
- [x] Make more descriptive `GridError` enum variants
- [x] Implement `Index|IndexMut<Pos>`
- [x] Rename `collect` to `flatten`
- [x] Add auto-trait with `clear`, `fill`, and related
- [x] Add an example of noise generation (`examples/noise.rs`)
- [x] Add an example of pathfinding (`examples/pathfinding.rs`)
- [x] Add an example of a grid with a custom layout (`examples/z-order.rs`)
- [ ] Benchmark heavily and adjust as needed

## 0.6.0 (Stable)

- [ ] Implement some nice debug string representations/formats
- [ ] Test coverage back to 100%, fix bugs as they come up

## Future

- [ ] Directions (in `Ixy`)
- [ ] Add rotate
- [ ] Add flip
- [ ] Add transpose
- [ ] Ranges?
- [ ] Optional support for `defmt`
- [ ] Add `GridMut` trait for mutable grid buffer operations
  - [ ] `resize`
  - [ ] `push_{row,col,block}`
  - [ ] `pop_*`
  - [ ] `insert_*`
  - [ ] `remove_*`
  - [ ] `expand_*`
  - [ ] `shrink_*`
  - [ ] `map`-like (mut transform) operations
- [ ] Add `grixy::buf::macros` module with macros for creating grids
- [ ] Add iterators for rows and columns
- [ ] Add a sparse (`BTree`-backed) grid buffer
- [ ] Noise-generation crate that uses `grixy` to represent a grid

## Speculative

- [ ] Support `Into/TryInto<usize>` to support arbitrary dimensions for Grids
- [ ] Support a `::diff` module for comparing grids
- [ ] Investigate generic parallelism for grid operations
- [ ] Path operations (in `Ixy`), potentially including flood-fill
- [ ] Octants (in `Ixy`), then Bersham's line algorithm
- [ ] Geometry (in `Ixy`)
