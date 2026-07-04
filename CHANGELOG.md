# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2026-07-04

Pre-1.0 stable release. No API breaking changes vs. alpha.8.

### Dependencies

- Bumped `ixy` from the exact-pinned prerelease `=0.6.0-alpha.9` to the published stable
  `=0.6.0`. No source changes needed; ixy's own `0.6.0` release was lint/doc/Justfile polish only.

### Added

- `PartialEq`/`Eq` derives on `GridBuf<T, B, L>` (bounded on `T: PartialEq`/`Eq`, `B: PartialEq`/
  `Eq`), so grids can be compared directly instead of manually iterating - matches what a stable
  owning container is expected to support.

### Fixed

- Doc examples for `GridWrite::set()` in `ops.rs` and `prelude.rs` ignored the returned
  `Result<(), GridError>`; they now `.unwrap()` it so the published docs model correct usage.
- `transform` module's test suite (`Rc`/`Arc`/`GridBuf`-based) was not feature-gated and failed to
  compile under `--no-default-features` or partial feature combinations (e.g. `--features cell`
  alone); now gated on `feature = "buffer"` and `feature = "alloc"`, which is what it actually
  exercises.

### Chores (pre-stable pass)

- Lint config aligned with the rest of the ecosystem (ixy/gem/hexal/framepace): `[lints.rust]`
  now denies `missing_docs`/`unreachable_pub`/`unused_qualifications` (previously only
  `missing_docs = "warn"`), and `[lints.clippy]` now denies `all`/`nursery`/`must_use_candidate`/
  `missing_errors_doc`/`missing_panics_doc` in addition to `pedantic`. (`unsafe_code` is not
  forbidden here, unlike sibling crates, since `grixy` legitimately uses `unsafe` for its
  `*_unchecked` accessors.) Turning these on surfaced and fixed ~20 real findings: unnecessary
  qualifications, `if let`/`else` that should be `Option::map_or(_else)`, redundant trait bounds,
  `needless_collect` in tests, and one visibility lint conflict (`redundant_pub_crate` vs.
  `unreachable_pub` disagreeing on `internal::IterRect`'s visibility - kept `pub(crate)` since
  that's the true reachability, with a scoped `#[allow]` and comment explaining why).

### Known follow-up (not blocking this release)

- Non-default feature combinations still don't fully build/test cleanly beyond the one fix above
  (e.g. `--features buffer` alone hits missing `GridBuf::new_filled` in test code elsewhere). CI
  and this crate's own `Justfile` have only ever exercised `--all-features`, so this is pre-existing
  and undetected, not a regression. Worth a `cargo-hack --feature-powerset` pass in a follow-up.

## [0.6.0-alpha.8] - 2026-07-03

### Dependencies

- Bumped `ixy` from `0.6.0-alpha.5` to `0.6.0-alpha.9`. Mechanical rename to follow ixy's own
  breaking rename in that range:
  - `layout::Traversal` → `layout::Layout`
  - `layout::Linear` → `layout::LinearLayout`
  - `LinearLayout::pos_to_index()`/`index_to_pos()`'s second parameter is now named `stride`
    (was `width`); semantics unchanged
  - No grixy-visible behavior change; `core::Size`/`core::Rect`/`HasSize` stay source-compatible
    since ixy's new generic `Int` parameters default to `usize`, matching grixy's existing usage
- `ixy` is now pinned with an exact requirement (`=0.6.0-alpha.9`) instead of a caret requirement.
  Previously published `grixy` versions used a loose `ixy = "0.6.0-alpha.5"` requirement, which,
  because both are pre-1.0 versions on the same `0.6.0` track, let Cargo silently resolve to any
  newer `0.6.0-alpha.N` release of `ixy` - including ones with breaking renames. Exact-pinning
  avoids repeating this for future `ixy` prereleases.

### Fixed

- `just semver-checks` no longer hardcodes an old baseline version (previously
  `0.6.0-alpha.4`). That baseline's own loose `ixy` requirement made it re-resolve to newer,
  incompatible `ixy` prereleases and fail to build entirely, unrelated to any actual API change.
  The recipe now lets `cargo-semver-checks` auto-select the baseline (the latest true stable
  release), which isn't affected by `ixy`'s alpha-track churn.

## [0.6.0-alpha.6] - 2026-06-19

### Added

- `GridDiff` trait with `diff()` method for comparing two grids
- `resize()` and `resize_filled()` on `GridBuf<T, Vec<T>, L>`
- `iter_rect_with_pos()` on `GridRead` — position-paired rect iteration
- `iter_with_pos()` and `cells()` on `GridIter`
- `get_mut()` on `GridBuf` — mutable checked accessor
- `Display` for `GridBuf<T: Display + Default + PartialEq, ...>`
- Optional `serde` feature for `Serialize`/`Deserialize` on `GridBuf` and `GridError`
- `missing_docs = "warn"` lint and docs for all public items

### Changed

- `L: layout::Linear` removed from `GridBuf` struct definition (C-STRUCT-BOUNDS)
- License changed from `MIT` to `MIT OR Apache-2.0`
- Updated `ixy` dependency from `0.6.0-alpha.2` to `0.6.0-alpha.5`
- Fixed `feature(doc_auto_cfg)` → `feature(doc_cfg)` for docs.rs compatibility

### Removed

- `transform::blend` submodule (`clear`, `source`, `destination` functions)
  — trivially expressible as closures
- Bridge functions `pos_from_u16`, `pos_to_u16`, `rect_from_u16`
  — unnecessary; rg can cast inline with `usize::from()`

## [0.6.0] - Unreleased

### Added

- Re-added `grid::ops::blend`, which was mistakenly omitted
- Implements `GridRead` and `GridWrite` for smart pointers (Box, Rc, Arc) that
  contain a grid

### Changed

- No features are enabled by default anymore
- All conversions are now exposed through the `convert` module
- Conversions now consume the source grid, rather than borrowing it. This allows
  more fluent chaining of operations, at the cost of needing to use a wrapper
  like `Rc<Grid>` to keep the original grid around
- Generic bounds for various conversion structs have been relaxed. While it
  technically could have changed the API, it was not used in any particularly
  useful way

## [0.5.0] - 2025-08-01

Major changes to the API, including new traits and methods for grid operations.

### Added

- Added `GridDraw` to `grixy::ops`
- `map` and `copied` to `GridRead`
- Added `grid::ops::blend` for example blend functions
- Added `grixy::prelude` module for common imports
- Added `VecGrid::new_generate` method to create a grid with a function

### Changed

- `GridRead` and `GridReadUnchecked` use generic associated types (GATs) for
  `Element`, which allows more flexiblity in how either references or owned
  values are returned. This capability is used to allow a `.map` method to
  lazily transform the grid's elements on read.
- Renamed `fill_rect_from` to `fill_rect_iter`
- Renamed `grixy::grid` to `grixy::ops`
- Moved all unchecked operations to `grixy::ops::unchecked`
- The `buffer` feature can be enabled to include `GridBuf` and related types

### Removed

- Most of the convenience types and constructors from `GridBuf`
- `GridBase`; now every Grid trait has it's own `Element` type
- `Rect::contains_pos` method; use `Rect::contains` instead
- The `bytemuck` feature; `GridBuf` already supports `AsRef<[T]>` which is
  sufficient to use `bytemuck` when `T` is `Pod`.

## [0.4.0] - 2025-07-19

### Changed

- Renamed various methods like `rect_iter` to `iter_rect` for consistency
- Renamed `fill_rect_iter` to `fill_rect_from`

### Removed

- No longer exporting `core::Layout`; use `ixy::index::Layout` as needed.

## [0.3.0] - 2025-07-19

### Added

- `GridBuf<T, B>` is now `AsRef<[T]>` when `B: AsRef<[T]>`
- `GridBuf<T, B>` is now `AsMut<[T]>` when `B: AsMut<[T]>`
- Optional feature `bytemuck` to implement `bytemuck::Pod` on eligible `GridBuf`
- Reduced constraints on `AsRef<[T]>` where able

### Changed

- The `alloc` feature is now disabled by default, and can be enabled manually

## [0.2.1] - 2025-07-18

### Changed

- Dependency on `ixy` widened to `>=0.3.0, <0.5.0`.

## [0.2.0] - 2025-07-18

### Added

- Added (with default implementations) to `GridRead`, `GridReadUnchecked`
  - `rect_iter`, `rect_iter_unchecked`
- Added (with default implementations) to `GridWrite`, `GridWriteUnchecked`
  - `fill_rect`, `fill_rect_unchecked`
  - `fill_rect_iter`, `fill_rect_iter_unchecked`
  - `fill_rect_solid`, `fill_rect_solid_unchecked`

### Removed

- Removed unused `impls` module.

## [0.1.0] - 2025-07-14

### Added

The feature `alloc` (enabled by default) controls use of `alloc::vec::Vec`.

- `buf`: Added type aliases (i.e. `VecGrid`) and `::bits` (for compact bit-grids)
- `core`: Exported additional types from `ixy`
- `grid`: Various traits for reading and writing to grid-like types

### Changed

- Moved top-level types to the `core` module

## [0.0.0] - 2025-07-12

### Added

- Initial release
