# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `GridRead` has default implementations of `{row|col|rect}_iter`.
- `GridWrite` has default implementations of `set_{row|col|rect}`.

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
