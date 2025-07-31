# Todo for 0.5.0 (Stable)

- [ ] Add `fn size_hint(&self) -> Option<Size>` to `GridRead` and `GridWrite`
- [x] Rename `BoundedGrid` to `TrustedSizeGrid`
- [x] Add `GridReadMapped` and `GridWriteMapped` traits for mapped buffers
- [x] As part of above, figure out GATs versus ...
- [x] Add `GridReadExt` and `GridWriteExt` traits for additional methods
- [ ] Move `ops::unchecked` back into `ops` module
