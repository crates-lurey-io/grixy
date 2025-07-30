# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - WIP (Alpha)

### Added

- Added `{copy_|blit}_[scaled]_rect` to `grixy::ops` (and _unchecked_ variants)
- Added `grixy::prelude` module for common imports

### Changed

- Renamed `fill_rect_from` to `fill_rect_iter`
- Renamed `grixy::grid` to `grixy::ops`
- Moved all unchecked operations to `grixy::ops::unchecked`

### Removed

- `GridBase`; now every Grid trait has it's own `Element` type

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
