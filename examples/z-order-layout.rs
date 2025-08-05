//! Implements a custom Z-order layout for a grid buffer.

use grixy::{
    ops::layout::{Linear, Traversal},
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

impl Traversal for ZOrderCurve {
    fn iter_pos<T: ixy::int::Int>(rect: ixy::Rect<T>) -> impl Iterator<Item = ixy::Pos<T>> {
        RowMajor::iter_pos(rect)
    }

    fn iter_rect<T: ixy::int::Int>(
        rect: ixy::Rect<T>,
        size: ixy::Size,
    ) -> impl Iterator<Item = ixy::Rect<T>> {
        RowMajor::iter_rect(rect, size)
    }
}

impl Linear for ZOrderCurve {
    fn pos_to_index(pos: Pos, _width: usize) -> usize {
        let x: u16 = pos.x.try_into().expect("Coordinates must fit in a u16");
        let y: u16 = pos.y.try_into().expect("Coordinates must fit in a u16");
        let x: u32 = x.into();
        let y: u32 = y.into();

        let x = spread_bits(x);
        let y = spread_bits(y);
        let index = x | (y << 1);
        index as usize
    }

    fn index_to_pos(index: usize, _width: usize) -> Pos {
        let index: u32 = index.try_into().expect("Index must fit in a u32");
        let x = unspread_bits(index);
        let y = unspread_bits(index >> 1);
        Pos::new(x as usize, y as usize)
    }

    fn slice_rect_aligned<E>(slice: &[E], size: Size, rect: Rect) -> Option<&[E]> {
        let start = Self::pos_to_index(rect.top_left(), size.width);
        let end = Self::pos_to_index(rect.bottom_right(), size.width);

        if start >= slice.len() || end > slice.len() {
            return None;
        }

        Some(&slice[start..end])
    }

    fn slice_rect_aligned_mut<E>(slice: &mut [E], size: Size, rect: Rect) -> Option<&mut [E]> {
        let start = Self::pos_to_index(rect.top_left(), size.width);
        let end = Self::pos_to_index(rect.bottom_right(), size.width);

        if start >= slice.len() || end > slice.len() {
            return None;
        }

        Some(&mut slice[start..end])
    }

    fn len_aligned(_size: ixy::Size) -> usize {
        unimplemented!()
    }

    fn rect_to_range(_size: ixy::Size, _rect: ixy::Rect<usize>) -> Option<std::ops::Range<usize>> {
        unimplemented!()
    }

    fn slice_aligned<E>(_slice: &[E], _size: ixy::Size, _axis: usize) -> &[E] {
        unimplemented!()
    }

    fn slice_aligned_mut<E>(_slice: &mut [E], _size: Size, _axis: usize) -> &mut [E] {
        unimplemented!()
    }
}

fn main() {
    let rect = Rect::new(Pos::new(0, 0), Size::new(4, 4));
    for pos in ZOrderCurve::iter_pos(rect) {
        println!("{pos:?}");
    }

    let index = ZOrderCurve::pos_to_index(Pos::new(2, 3), 4);
    println!("Index of (2, 3): {index}");

    let pos = ZOrderCurve::index_to_pos(index, 4);
    println!("Position of {index}: {pos:?}");
}
