use std::fmt::{Display, Formatter};
use std::ops::{BitAnd, Index, Not};

use crate::bit::bitty::Bitty;

#[derive(Debug, Clone, Copy)]
pub struct Bitmap<B = u32> {
    value: B,
    bitmap_size: usize,
}

// Kept around because a puzzle needs the const impl, but we can't const impl trait functions
impl Bitmap<u32> {
    pub const fn new_const(value: u32, bitmap_size: usize) -> Self {
        let masked_value = value & Self::calc_mask_const(bitmap_size);
        Bitmap {
            value: masked_value,
            bitmap_size,
        }
    }

    /// Returns an integer mask that can be applied to the value with BitAnd, setting all values outside the bitmap_size to 0.
    const fn calc_mask_const(bitmap_size: usize) -> u32 {
        match bitmap_size {
            0..=31 => 2u32.pow(bitmap_size as u32) - 1,
            _ => u32::MAX, // 2u32.pow(32) would overflow, everything else is invalid
        }
    }
}

impl<B: Bitty> Bitmap<B> {
    pub fn new(value: B, bitmap_size: usize) -> Self {
        if bitmap_size > B::max_bits() {
            panic!("Out of range bitmap_size!")
        }
        let masked_value = value & Self::calc_mask(bitmap_size);
        Bitmap {
            value: masked_value,
            bitmap_size,
        }
    }

    /// Returns an integer mask that can be applied to the value with BitAnd, setting all values outside the bitmap_size to 0.
    pub fn calc_mask(bitmap_size: usize) -> B {
        if bitmap_size == B::max_bits() {
            B::max_value()
        } else {
            let two = B::one() + B::one();
            two.pow(bitmap_size as u32) - B::one()
        }
    }

    pub fn get(&self, pos: usize) -> bool {
        self[pos]
    }

    pub fn set(&mut self, pos: usize, value: bool) {
        if pos >= self.bitmap_size {
            return;
        }
        let current = self.get(pos);
        if current != value {
            self.value = self.value ^ (B::one() << pos);
        }
    }
}

impl<B: Bitty> Index<usize> for Bitmap<B> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        let calc = (self.value >> index) & B::one();
        if calc == B::one() {
            &true
        } else if calc == B::zero() {
            &false
        } else {
            panic!("Neither true nor false!")
        }
    }
}

impl<B: Bitty> From<B> for Bitmap<B> {
    fn from(int: B) -> Self {
        Bitmap::new(int, B::max_bits())
    }
}

impl From<Bitmap<u32>> for u32 {
    fn from(bitmap: Bitmap<u32>) -> Self {
        bitmap.value
    }
}

impl From<Bitmap<u16>> for u16 {
    fn from(bitmap: Bitmap<u16>) -> Self {
        bitmap.value
    }
}

impl<B: Bitty> Not for Bitmap<B> {
    type Output = Bitmap<B>;

    fn not(self) -> Self::Output {
        let inv = !self.value;
        Bitmap::new(inv, self.bitmap_size)
    }
}

impl<B: Bitty> BitAnd for Bitmap<B> {
    type Output = Bitmap<B>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let size = if self.bitmap_size >= rhs.bitmap_size {
            self.bitmap_size
        } else {
            rhs.bitmap_size
        };
        let val = self.value & rhs.value;
        Bitmap::new(val, size)
    }
}

impl<B: Bitty> PartialEq<Self> for Bitmap<B> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<B: Bitty> Display for Bitmap<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..B::max_bits() {
            if i < self.bitmap_size {
                let value = if self[i] { "✅️" } else { "⬛️" };
                f.write_str(value)?;
            } else {
                f.write_str("➖️")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::bit::bitmap::Bitmap;

    #[test]
    fn can_construct_without_panic() {
        let _bits: Bitmap = 1982u32.into();
        let bits2: Bitmap = 4294967295u32.into();
        assert_eq!(bits2.value, 4294967295u32);
    }

    #[test]
    fn can_access_bits() {
        let bitmap: Bitmap<_> = 2u8.into();
        assert_eq!(bitmap.get(0), false);
        assert_eq!(bitmap.get(1), true);
    }

    #[test]
    fn can_access_bits_parsed() {
        let bit_str = "011110111110";
        let int = u32::from_str_radix(bit_str, 2).unwrap();
        assert_eq!(int, 1982);
        let bit_list: Vec<char> = bit_str.chars().collect();
        let bitmap: Bitmap = int.into();
        // reverse the bit list, read right-to-left
        for (index, char) in bit_list.iter().rev().enumerate() {
            let from_bitmap = bitmap.get(index);
            if from_bitmap {
                assert_eq!(*char, '1');
            } else {
                assert_eq!(*char, '0');
            }
        }
    }

    #[test]
    fn can_set_pos() {
        let mut bitmap = Bitmap::new(0u32, 2);
        bitmap.set(0, true);
        assert_eq!(bitmap.get(0), true);
        assert_eq!(bitmap.value, 1);

        bitmap.set(1, true);
        assert_eq!(bitmap.get(1), true);
        assert_eq!(bitmap.value, 3);

        bitmap.set(0, false);
        assert_eq!(bitmap.get(0), false);
        assert_eq!(bitmap.value, 2);

        let mut bitmap2 = Bitmap::new(4294967295u32, 32);
        assert_eq!(bitmap2.value, 4_294_967_295u32);
        bitmap2.set(0, false);
        assert_eq!(bitmap2.value, 4_294_967_294u32);
        bitmap2.set(31, false);
        assert_eq!(bitmap2.value, 2_147_483_646u32);

        // Set to same as existing value, should remain unchanged.
        bitmap.set(1, true);
        assert_eq!(bitmap.get(0), false);
        assert_eq!(bitmap.get(1), true);
        assert_eq!(bitmap.value, 2);

        // Set to value outside size, should remain unchanged.
        assert_eq!(bitmap.get(2), false);
        bitmap.set(2, true);
        assert_eq!(bitmap.get(0), false);
        assert_eq!(bitmap.get(1), true);
        assert_eq!(bitmap.get(2), false);
        assert_eq!(bitmap.value, 2);
    }

    #[test]
    fn to_u32() {
        let zero: u32 = Bitmap::new(0, 2).into();
        assert_eq!(zero, 0);

        let mut bitmap1 = Bitmap::new(0, 2);
        bitmap1.set(0, true);
        let one: u32 = bitmap1.into();
        assert_eq!(one, 1);

        let mut bitmap2 = Bitmap::new(0, 2);
        bitmap2.set(1, true);
        let two: u32 = bitmap2.into();
        assert_eq!(two, 2);
    }

    #[test]
    fn not_op() {
        let mut bitmap = Bitmap::new(0u32, 2);
        bitmap.set(1, true);
        assert_eq!(bitmap.get(2), false);
        let flipped = !bitmap;
        assert_eq!(flipped.get(1), false);
        assert_eq!(flipped.get(0), true);
        // assert ! doesn't flip bits outside size
        assert_eq!(flipped.get(2), false);
    }

    #[test]
    fn bitand_op() {
        let bitmap1 = Bitmap::new(6u32, 4);
        let bitmap2 = Bitmap::new(10u32, 4);
        let bitmap3 = bitmap1 & bitmap2;
        assert_eq!(bitmap3.value, 2);
    }

    #[test]
    fn display() {
        let bitmap1 = Bitmap::new(6u32, 4);
        println!("{}", bitmap1);
        let bitmap2 = Bitmap::new(2893u16, 14);
        println!("{}", bitmap2);
        let bitmap3 = Bitmap::new(218u8, 8);
        println!("{}", bitmap3);
    }
}
