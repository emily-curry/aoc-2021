use std::ops::Not;

#[derive(Debug, Clone, Copy)]
pub struct Bitmap {
    value: u32,
    bitmap_size: usize,
}

impl Bitmap {
    pub fn new(value: u32, bitmap_size: usize) -> Self {
        Bitmap { value, bitmap_size }
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
        let mask = 2u32.pow(self.bitmap_size as u32) - 1;
        let val = inv & mask;
        Bitmap::new(val, self.bitmap_size)
    }
}

#[cfg(test)]
mod tests {
    use crate::bitmap::Bitmap;

    #[test]
    fn can_construct_without_panic() {
        let _bits: Bitmap = 1982u32.into();
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
}
