use std::hash::Hash;
use std::ops::{Add, BitAnd, BitOr, BitXor, Not, Rem, Shl, Shr, Sub};

pub trait Bitmappable:
    Copy
    + Clone
    + Eq
    + Ord
    + Hash
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + Not<Output = Self>
    + BitAnd<Self, Output = Self>
    + BitOr<Self, Output = Self>
    + BitXor<Self, Output = Self>
    + Rem<Self, Output = Self>
{
    fn zero() -> Self;

    fn one() -> Self;

    fn max_value() -> Self;

    fn max_bits() -> usize;

    fn pow(self, rhs: u32) -> Self;
}

impl Bitmappable for u32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn max_value() -> Self {
        u32::MAX
    }

    fn max_bits() -> usize {
        u32::BITS as usize
    }

    fn pow(self, rhs: u32) -> Self {
        self.pow(rhs)
    }
}

impl Bitmappable for u16 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn max_value() -> Self {
        u16::MAX
    }

    fn max_bits() -> usize {
        u16::BITS as usize
    }

    fn pow(self, rhs: u32) -> Self {
        self.pow(rhs)
    }
}

impl Bitmappable for u8 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn max_value() -> Self {
        u8::MAX
    }

    fn max_bits() -> usize {
        u8::BITS as usize
    }

    fn pow(self, rhs: u32) -> Self {
        self.pow(rhs)
    }
}
