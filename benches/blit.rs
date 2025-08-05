use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use grixy::prelude::*;

// Loads a bitmapped font from a binary file.
//
// The font looks like this:
//
// # Glyph 0x00
// • • • • • • • •
// • • • • • • • •
// • • • • • • • •
// • • • • • • • •
// • • • • • • • •
// • • • • • • • •
// • • • • • • • •
// • • • • • • • •
//
// # Glyph 0x01
// • • • • • • • •
// • • • • • • • •
// • • • • • • • •
// • • • • • • • •
// • • • • • • • •
// • • • • • • • •
// • • • • • • • •
// • • • • • • • •
//
// ... up to 0xFF.
const IBM_VGA_8X8: &[u8] = include_bytes!("./IBM_VGA_8x8.bin");

#[inline]
fn expand(bits: &[u8]) -> Vec<u32> {
    // Convert each bit to a u32, where 1 becomes 0xFFFF_FFFF and 0 becomes 0xFF00_0000.
    bits.iter()
        .flat_map(|&byte| {
            (0..8).map(move |i| {
                if (byte >> (7 - i)) & 1 == 1 {
                    0xFFFF_FFFF
                } else {
                    0xFF00_0000
                }
            })
        })
        .collect::<Vec<_>>()
}

#[inline]
#[allow(clippy::needless_pass_by_value)]
fn blit_vec(pixels: Vec<u32>) -> Vec<u32> {
    // Create a Vec-based output buffer.
    let mut canvas = Vec::<u32>::with_capacity(8 * 16 * 8 * 16);

    // Read each glyph from the font and copy it to the canvas in reverse order.
    for i in (0..256).rev() {
        let offset_start = i * 8 * 8;
        let offset_end = offset_start + 8 * 8;
        let glyph = &pixels[offset_start..offset_end];

        // Copy the glyph to the canvas.
        canvas.extend_from_slice(glyph);
    }

    canvas
}

#[inline]
fn blit_grid(pixels: Vec<u32>) -> Vec<u32> {
    // Create a Grid-based output buffer.
    let mut dst = GridBuf::<u32, _, RowMajor>::new(8 * 16, 8 * 16);

    // Create a Grid-based view over the font data.
    let src = GridBuf::<u32, _, RowMajor>::from_buffer(pixels, 8).copied();

    // Read each glyph from the font and copy it to the canvas in reverse order.
    for i in (0..256).rev() {
        copy_rect(
            &src,
            &mut dst,
            Rect::from_ltwh(0, i * 8, 8, 8),
            Pos::new((i % 16) * 8, (i / 16) * 8),
        );
    }

    dst.into_inner().0
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Blit");
    let pixels = expand(IBM_VGA_8X8);

    group.bench_function("blit_vec IBM_VGA_8X8", |b| {
        b.iter_batched(
            || pixels.clone(),
            |pixels| black_box(blit_vec(pixels)),
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("blit_grid IBM_VGA_8X8", |b| {
        b.iter_batched(
            || pixels.clone(),
            |pixels| black_box(blit_grid(pixels)),
            criterion::BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
