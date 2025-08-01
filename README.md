# grixy

Zero-cost 2D grids for Rust.

_Powerful enough for embedded systems, convenient enough for game development._

[![Test](https://github.com/crates-lurey-io/grixy/actions/workflows/test.yml/badge.svg)](https://github.com/crates-lurey-io/grixy/actions/workflows/test.yml)
[![Docs](https://github.com/crates-lurey-io/grixy/actions/workflows/docs.yml/badge.svg)](https://github.com/crates-lurey-io/grixy/actions/workflows/docs.yml)
[![Crates.io Version](https://img.shields.io/crates/v/grixy)](https://crates.io/crates/grixy)
[![codecov](https://codecov.io/gh/crates-lurey-io/grixy/graph/badge.svg?token=Z3VUWA3WYY)](https://codecov.io/gh/crates-lurey-io/grixy)

## Example

### Drawing

Grixy can be used for more complex operations like software blending or scaling:

```sh
cargo run --example mono-font-raster
```

This example:

- loads a bitmap font, and views it as a grid of glyphs;
- creates an in-memory buffer of rgba pixels;
- draws the glyphs into the pixel buffer;
- (using the `png` crate) saves the pixel buffer as a PNG file, seen below.

![Loading and rendering an 8x8 font](examples/mono-font-raster-out.png)

## Contributing

This project uses [`just`][] to run commands the same way as the CI:

- `cargo just check` to check formatting and lints.
- `cargo just coverage` to generate and preview code coverage.
- `cargo just doc` to generate and preview docs.
- `cargo just test` to run tests.

[`just`]: https://crates.io/crates/just

For a full list of commands, see the [`Justfile`](./Justfile).
