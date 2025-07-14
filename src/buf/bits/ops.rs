use core::{
    fmt::Debug,
    mem,
    ops::{BitAnd, BitAndAssign, BitOrAssign, Not},
};

use crate::internal::Sealed;

/// An unsigned integer type that can be used as a bit buffer.
pub trait BitOps:
    Sized
    + TryFrom<usize, Error: Debug>
    + Copy
    + Sealed
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOrAssign
    + Not<Output = Self>
{
    /// The maximum width that can be represented by this type.
    const MAX_WIDTH: usize = mem::size_of::<Self>() * 8;

    /// Converts a `usize` into this type.
    ///
    /// ## Panics
    ///
    /// Panics if the value is larger than `Self::MAX_WIDTH`.
    #[must_use]
    fn from_usize(value: usize) -> Self {
        value.try_into().expect("Value exceeds maximum width")
    }

    /// Converts this type into a `usize`.
    fn to_usize(self) -> usize;
}

impl Sealed for u8 {}
impl BitOps for u8 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Sealed for u16 {}
impl BitOps for u16 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Sealed for u32 {}
impl BitOps for u32 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Sealed for usize {}
impl BitOps for usize {
    fn to_usize(self) -> usize {
        self
    }
}

#[cfg(target_pointer_width = "64")]
impl Sealed for u64 {}

#[cfg(target_pointer_width = "64")]
impl BitOps for u64 {
    #[allow(clippy::cast_possible_truncation)]
    fn to_usize(self) -> usize {
        self as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_to_usize() {
        let value: u8 = 42;
        assert_eq!(value.to_usize(), 42);
    }

    #[test]
    fn u16_to_usize() {
        let value: u16 = 42;
        assert_eq!(value.to_usize(), 42);
    }

    #[test]
    fn u32_to_usize() {
        let value: u32 = 42;
        assert_eq!(value.to_usize(), 42);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn u64_to_usize() {
        let value: u64 = 42;
        assert_eq!(value.to_usize(), 42);
    }

    #[test]
    fn usize_to_usize() {
        let value: usize = 42;
        assert_eq!(value.to_usize(), 42);
    }
}
