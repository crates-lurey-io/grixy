//! Loads a monospace font and renders an RGBA PNG based on every glyph in the font.

use grixy::{
    buf::bits::GridBits,
    core::{Pos, Rect},
    ops::{copy_rect, unchecked::TrustedSizeGrid as _},
    transform::GridConvertExt as _,
};
use temp_dir::TempDir;

fn main() {
    // Loads a bitmapped font from a binary file.
    const IBM_VGA_8X8: &[u8] = include_bytes!("./IBM_VGA_8x8.bin");

    // Read how much to scale the font by from user input (stdin).
    println!("Enter the scale factor (e.g., 2 for 2x scaling):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let scale: usize = input.trim().parse().unwrap_or(1);

    // Creates a 2-dimensional view over the bits of the font.
    let font = GridBits::<_, _>::from_buffer(IBM_VGA_8X8, 8)
        .map(|bit| if bit { 0xFFFF_FFFFu32 } else { 0xFF00_0000u32 })
        .scale(scale);

    let mut canvas = grixy::buf::GridBuf::<u32, _>::new(8 * 16 * scale, 8 * 16 * scale);

    // Draws each glyph from the font onto the canvas, with 32 (different) glyphs per row.
    for i in 0..256 {
        let x = (i % 16) * 8 * scale;
        let y = (i / 16) * 8 * scale;

        // Draws the glyph onto the canvas at the specified position.
        copy_rect(
            &font,
            &mut canvas,
            Rect::from_ltwh(0, i * 8 * scale, 8 * scale, 8 * scale),
            Pos::new(x, y),
        );
    }

    // Prepare to save the RGBA font as a PNG file.
    let tmp_dir = TempDir::new().unwrap().dont_delete_on_drop();
    let png_path = tmp_dir.path().join("font.png");

    // Saves the RGBA font as a PNG file.
    let mut encoder = png::Encoder::new(
        std::fs::File::create(&png_path).unwrap(),
        u32::try_from(canvas.width()).unwrap(),
        u32::try_from(canvas.height()).unwrap(),
    );
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer
        .write_image_data(bytemuck::cast_slice(canvas.as_ref()))
        .unwrap();

    open::that(png_path).unwrap();
}
