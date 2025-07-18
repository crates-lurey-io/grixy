# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
