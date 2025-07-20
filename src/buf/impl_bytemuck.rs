use ixy::index::Layout;

use crate::buf::GridBuf;

impl<T, B, L> GridBuf<T, B, L>
where
    T: bytemuck::Pod,
    B: AsRef<[T]>,
    L: Layout,
{
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(self.buffer.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use crate::{buf::VecGrid, core::Pos, grid::GridWrite};
    use bytemuck::{Pod, Zeroable};

    #[derive(Clone, Copy, Default)]
    #[repr(C)]
    struct Rgba {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }

    unsafe impl Pod for Rgba {}
    unsafe impl Zeroable for Rgba {}

    #[test]
    fn rgba_as_bytes() {
        let mut grid = VecGrid::<Rgba, _>::new_row_major(2, 2);
        let _ = grid.set(
            Pos::new(0, 0),
            Rgba {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
        );
        let _ = grid.set(
            Pos::new(1, 0),
            Rgba {
                r: 0,
                g: 255,
                b: 0,
                a: 255,
            },
        );
        let _ = grid.set(
            Pos::new(0, 1),
            Rgba {
                r: 0,
                g: 0,
                b: 255,
                a: 255,
            },
        );
        let _ = grid.set(
            Pos::new(1, 1),
            Rgba {
                r: 255,
                g: 255,
                b: 0,
                a: 255,
            },
        );

        let bytes = grid.as_bytes();

        #[rustfmt::skip]
        assert_eq!(
            bytes,
            &[
                255, 0,   0,   255, // Red
                0,   255, 0,   255, // Green
                0,   0,   255, 255, // Blue
                255, 255, 0,   255  // Yellow
            ]
        );
    }
}
