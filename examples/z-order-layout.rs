//! Implements a custom Z-order layout for a grid buffer.

use grixy::{
    ops::layout::{Layout, Linear},
    prelude::*,
};

/// A Z-order curve is a space-filling curve that maps multi-dimensional data to one dimension.
///
/// See <https://en.m.wikipedia.org/wiki/Z-order_curve>.
enum ZOrderCurve {}

const fn spread_bits(n: u32) -> u32 {
    let mut x = n;
    x = (x | (x << 8)) & 0x00ff_00ff;
    x = (x | (x << 4)) & 0x0f0f_0f0f;
    x = (x | (x << 2)) & 0x3333_3333;
    x = (x | (x << 1)) & 0x5555_5555;
    x
}

const fn unspread_bits(n: u32) -> u32 {
    let mut x = n & 0x5555_5555;
    x = (x | (x >> 1)) & 0x3333_3333;
    x = (x | (x >> 2)) & 0x0f0f_0f0f;
    x = (x | (x >> 4)) & 0x00ff_00ff;
    x = (x | (x >> 8)) & 0x0000_ffff;
    x
}

impl Layout for ZOrderCurve {
    fn iter_pos(rect: Rect) -> impl Iterator<Item = Pos> {
        RowMajor::iter_pos(rect)
    }
}

impl Linear for ZOrderCurve {
    fn to_1d(pos: Pos, _width: usize) -> usize {
        let x: u16 = pos.x.try_into().expect("Coordinates must fit in a u16");
        let y: u16 = pos.y.try_into().expect("Coordinates must fit in a u16");
        let x: u32 = x.into();
        let y: u32 = y.into();

        let x = spread_bits(x);
        let y = spread_bits(y);
        let index = x | (y << 1);
        index as usize
    }

    fn to_2d(index: usize, _width: usize) -> Pos {
        let index: u32 = index.try_into().expect("Index must fit in a u32");
        let x = unspread_bits(index);
        let y = unspread_bits(index >> 1);
        Pos::new(x as usize, y as usize)
    }

    fn slice_rect_aligned<E>(slice: &[E], size: Size, rect: Rect) -> Option<&[E]> {
        let start = Self::to_1d(rect.top_left(), size.width);
        let end = Self::to_1d(rect.bottom_right(), size.width);

        if start >= slice.len() || end > slice.len() {
            return None;
        }

        Some(&slice[start..end])
    }

    fn slice_rect_aligned_mut<E>(slice: &mut [E], size: Size, rect: Rect) -> Option<&mut [E]> {
        let start = Self::to_1d(rect.top_left(), size.width);
        let end = Self::to_1d(rect.bottom_right(), size.width);

        if start >= slice.len() || end > slice.len() {
            return None;
        }

        Some(&mut slice[start..end])
    }
}

fn main() {
    let rect = Rect::new(Pos::new(0, 0), Size::new(4, 4));
    for pos in ZOrderCurve::iter_pos(rect) {
        println!("{pos:?}");
    }

    let index = ZOrderCurve::to_1d(Pos::new(2, 3), 4);
    println!("Index of (2, 3): {index}");

    let pos = ZOrderCurve::to_2d(index, 4);
    println!("Position of {index}: {pos:?}");
}
