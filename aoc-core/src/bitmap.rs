use std::ops::{BitAnd, Not};

#[derive(Debug, Clone, Copy)]
pub struct Bitmap {
    value: u32,
    bitmap_size: usize,
}

impl Bitmap {
    pub const fn new(value: u32, bitmap_size: usize) -> Self {
        let masked_value = value & Self::calc_mask(bitmap_size as u32);
        Bitmap {
            value: masked_value,
            bitmap_size,
        }
    }

    pub fn at_pos(&self, pos: usize) -> bool {
        match (self.value >> pos) & 1 {
            1 => true,
            0 => false,
            _ => panic!("Neither true nor false!"),
        }
    }

    pub fn set_pos(&mut self, pos: usize, value: bool) {
        if pos >= self.bitmap_size {
            return;
        }
        let current = self.at_pos(pos);
        if current != value {
            self.value = self.value ^ (1 << pos);
        }
    }

    /// Returns an integer mask that can be applied to the value with BitAnd, setting all values outside the bitmap_size to 0.
    const fn calc_mask(bitmap_size: u32) -> u32 {
        match bitmap_size {
            0..=31 => 2u32.pow(bitmap_size) - 1,
            _ => 4294967295u32, // 2u32.pow(32) would overflow, everything else is invalid
        }
    }

    /// Returns an integer mask that can be applied to the value with BitAnd, setting all values outside the bitmap_size to 0.
    fn get_mask(&self) -> u32 {
        Bitmap::calc_mask(self.bitmap_size as u32)
    }
}

impl From<u8> for Bitmap {
    fn from(int: u8) -> Self {
        Bitmap::new(int.into(), 8)
    }
}

impl From<u16> for Bitmap {
    fn from(int: u16) -> Self {
        Bitmap::new(int.into(), 16)
    }
}

impl From<u32> for Bitmap {
    fn from(int: u32) -> Self {
        Bitmap::new(int.into(), 32)
    }
}

impl From<Bitmap> for u32 {
    fn from(bitmap: Bitmap) -> Self {
        bitmap.value
    }
}

impl Not for Bitmap {
    type Output = Bitmap;

    fn not(self) -> Self::Output {
        let inv = !self.value;
        Bitmap::new(inv, self.bitmap_size)
    }
}

impl BitAnd for Bitmap {
    type Output = Bitmap;

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

impl PartialEq<Self> for Bitmap {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[cfg(test)]
mod tests {
    use crate::bitmap::Bitmap;

    #[test]
    fn can_construct_without_panic() {
        let _bits: Bitmap = 1982u32.into();
        let bits2: Bitmap = 4294967295u32.into();
        assert_eq!(bits2.value, 4294967295u32);
    }

    #[test]
    fn can_access_bits() {
        let bitmap: Bitmap = 2u8.into();
        assert_eq!(bitmap.at_pos(0), false);
        assert_eq!(bitmap.at_pos(1), true);
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
            let from_bitmap = bitmap.at_pos(index);
            if from_bitmap {
                assert_eq!(*char, '1');
            } else {
                assert_eq!(*char, '0');
            }
        }
    }

    #[test]
    fn can_set_pos() {
        let mut bitmap = Bitmap::new(0, 2);
        bitmap.set_pos(0, true);
        assert_eq!(bitmap.at_pos(0), true);
        assert_eq!(bitmap.value, 1);

        bitmap.set_pos(1, true);
        assert_eq!(bitmap.at_pos(1), true);
        assert_eq!(bitmap.value, 3);

        bitmap.set_pos(0, false);
        assert_eq!(bitmap.at_pos(0), false);
        assert_eq!(bitmap.value, 2);

        let mut bitmap2 = Bitmap::new(4294967295u32, 32);
        assert_eq!(bitmap2.value, 4_294_967_295u32);
        bitmap2.set_pos(0, false);
        assert_eq!(bitmap2.value, 4_294_967_294u32);
        bitmap2.set_pos(31, false);
        assert_eq!(bitmap2.value, 2_147_483_646u32);

        // Set to same as existing value, should remain unchanged.
        bitmap.set_pos(1, true);
        assert_eq!(bitmap.at_pos(0), false);
        assert_eq!(bitmap.at_pos(1), true);
        assert_eq!(bitmap.value, 2);

        // Set to value outside size, should remain unchanged.
        assert_eq!(bitmap.at_pos(2), false);
        bitmap.set_pos(2, true);
        assert_eq!(bitmap.at_pos(0), false);
        assert_eq!(bitmap.at_pos(1), true);
        assert_eq!(bitmap.at_pos(2), false);
        assert_eq!(bitmap.value, 2);
    }

    #[test]
    fn to_u32() {
        let zero: u32 = Bitmap::new(0, 2).into();
        assert_eq!(zero, 0);

        let mut bitmap1 = Bitmap::new(0, 2);
        bitmap1.set_pos(0, true);
        let one: u32 = bitmap1.into();
        assert_eq!(one, 1);

        let mut bitmap2 = Bitmap::new(0, 2);
        bitmap2.set_pos(1, true);
        let two: u32 = bitmap2.into();
        assert_eq!(two, 2);
    }

    #[test]
    fn not_op() {
        let mut bitmap = Bitmap::new(0, 2);
        bitmap.set_pos(1, true);
        assert_eq!(bitmap.at_pos(2), false);
        let flipped = !bitmap;
        assert_eq!(flipped.at_pos(1), false);
        assert_eq!(flipped.at_pos(0), true);
        // assert ! doesn't flip bits outside size
        assert_eq!(flipped.at_pos(2), false);
    }

    #[test]
    fn bitand_op() {
        let bitmap1 = Bitmap::new(6, 4);
        let bitmap2 = Bitmap::new(10, 4);
        let bitmap3 = bitmap1 & bitmap2;
        assert_eq!(bitmap3.value, 2);
    }
}
