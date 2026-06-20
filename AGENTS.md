# Agent Guidelines for grixy

## Rust API Guidelines

Prefer following the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html).

Key points:
- No trait bounds on struct definitions (`C-STRUCT-BOUNDS`)
- Sealed traits for downstream-safe abstractions (`C-SEALED`)
- `no_std` compatible, verified in CI
- Unsafe is permitted in unchecked trait impls, but every `unsafe fn` must have a `# Safety` doc section and every `unsafe {}` block must have a `// SAFETY:` comment
- Optional `serde` feature via `cfg_attr` and feature gating
- Dual-licensed `MIT OR Apache-2.0`
- Keep `CHANGELOG.md` up to date

## Code style

- `cargo just check` before committing (lint + format)
- `cargo just test-all` to run all tests (unit + doc)
- `cargo just semver-checks` to verify no accidental breaking changes
- All public items must have doc examples
- `#[must_use]` on all methods returning a value
- Blanket trait impls over `GridRead + ExactSizeGrid` are preferred for extension traits (see `GridDiff`, `GridIter`)

## Architecture

```
src/
├── lib.rs         # Crate root, feature gates, module declarations
├── core.rs        # Pos, Rect, Size type aliases + GridError + conversion fns
├── prelude.rs     # Re-exports for `use grixy::prelude::*`
├── internal.rs    # Sealed trait + internal iter enums (not public API)
├── test.rs        # NaiveGrid test fixture
├── buf.rs         # GridBuf struct + Display + Index/IndexMut + module decls
├── buf/
│   ├── impl_grid.rs    # GridBase, ExactSizeGrid, TrustedSizeGrid, unchecked trait impls
│   ├── impl_new.rs     # new(), new_filled(), from_buffer() constructors
│   ├── impl_resize.rs  # resize(), resize_filled() (alloc feature)
│   ├── impl_serde.rs   # Serialize/Deserialize (serde feature)
│   ├── impl_slice.rs   # AsRef/AsMut<[T]>
│   └── bits/           # GridBits — bit-packed boolean grids
├── ops.rs          # Module declarations for ops
├── ops/
│   ├── base.rs     # GridBase, ExactSizeGrid traits
│   ├── read.rs     # GridRead, GridIter traits (get, iter_rect, iter_rect_with_pos, etc.)
│   ├── write.rs    # GridWrite trait (set, fill_*, clear_*)
│   ├── diff.rs     # GridDiff trait (grid comparison via diff())
│   ├── draw.rs     # copy_rect() standalone function
│   ├── layout.rs   # Re-exports from ixy (RowMajor, ColumnMajor, Block, Linear, Traversal)
│   ├── cell.rs     # GridWrite for Cell/RefCell/UnsafeCell wrappers
│   ├── alloc.rs    # GridRead for Rc/Arc
│   └── unchecked/  # GridReadUnchecked, GridWriteUnchecked, TrustedSizeGrid
└── transform.rs    # GridConvertExt (map, copied, view, scale, blend, flatten)
    └── transform/  # Copied, Mapped, Viewed, Scaled, Blended wrappers
```

## Safety

- Unsafe is permitted in this crate for unchecked trait impls
- The `unsafe` traits `TrustedSizeGrid`, `GridReadUnchecked`, `GridWriteUnchecked` are the only source of unsoundness if implemented incorrectly
- Every `unsafe fn` requires a `# Safety` doc section with preconditions
- Every `unsafe {}` block requires a `// SAFETY:` justification
- Blanket impls in `unchecked/` convert unchecked traits to checked traits when `TrustedSizeGrid` is present — these are the safety boundary

## Testing

- Unit tests live in `#[cfg(test)] mod tests {}` at the bottom of each source file
- Doc tests (`/// ```rust`) are required for all public APIs
- `NaiveGrid` in `test.rs` is the reference implementation for test fixtures
- Run `cargo just test-all` to run everything
