use crate::bit::bit_size::BitSize;
use crate::bit::bitmap::Bitmap;
use crate::bit::bitty::Bitty;
use std::fmt::{Display, Formatter};
use std::ops::Index;

#[derive(Debug)]
pub struct BitVec<B = u32> {
    maps: Vec<Bitmap<B>>,
}

impl<B: Bitty> BitVec<B> {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut maps_len = capacity / B::max_bits();
        if capacity % B::max_bits() != 0 {
            maps_len += 1;
        }
        let maps = (0..maps_len)
            .map(|_| Bitmap::new(B::zero(), B::max_bits()))
            .collect();
        BitVec { maps }
    }

    pub fn from_int(maps: Vec<B>) -> Self {
        BitVec {
            maps: maps.iter().map(B::clone).map(Bitmap::from).collect(),
        }
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        if self.get_bitmap_index(index) > self.maps.len() - 1 {
            None
        } else {
            Some(self[index])
        }
    }

    pub fn get_range<Rng>(
        &self,
        range: Rng,
    ) -> impl Iterator<Item = bool> + DoubleEndedIterator + '_
    where
        Rng: Iterator<Item = usize> + DoubleEndedIterator + 'static,
    {
        range.map(|i| self[i])
    }

    /// For the given range, returns an int represented by the bits range.
    /// The lower bound of the range represents the *least* significant bit.
    /// Therefore, for any internal bitmap `U`, `get_range_as_int(U.first..U.last) == U.value`.
    pub fn get_range_as_int<Rng>(&self, range: Rng) -> u64
    where
        Rng: Iterator<Item = usize> + DoubleEndedIterator + 'static,
    {
        let mut result = 0;
        for val in self.get_range(range).rev() {
            result = result << 1;
            if val {
                result = result | 1;
            }
        }
        result
    }

    pub fn set(&mut self, index: usize, value: bool) {
        let map_index = self.get_bitmap_index(index);
        let inner_index = self.get_bit_index(index);
        self.maps[map_index].set(inner_index, value);
    }

    fn get_bitmap_index(&self, index: usize) -> usize {
        index / B::max_bits()
    }

    fn get_bit_index(&self, index: usize) -> usize {
        index % B::max_bits()
    }
}

impl<B: Bitty> Index<usize> for BitVec<B> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        let map_index = self.get_bitmap_index(index);
        let inner_index = self.get_bit_index(index);
        &self.maps[map_index][inner_index]
    }
}

impl<B: Bitty, I: Iterator<Item = B>> From<I> for BitVec<B> {
    fn from(input: I) -> Self {
        BitVec {
            maps: input.map(Bitmap::from).collect(),
        }
    }
}

impl<B: Bitty> BitSize for BitVec<B> {
    fn bit_size(&self) -> usize {
        self.maps.len() * B::max_bits()
    }
}

impl<B: Bitty> Display for BitVec<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let last_index = self.maps.len() - 1;
        let padding = last_index.to_string().chars().count();
        for (i, map) in self.maps.iter().enumerate() {
            f.write_str(format!("{:0width$} -> ", i, width = padding).as_str())?;
            map.fmt(f)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::bit::bitvec::BitVec;

    #[test]
    fn int_division_works_like_i_expect() {
        assert_eq!(1 / 32, 0);
        assert_eq!(31 / 32, 0);
        assert_eq!(32 / 32, 1);
        assert_eq!(33 / 32, 1);
        assert_eq!(63 / 32, 1);
    }

    #[test]
    fn display() {
        let bitvec = BitVec::from_int(vec![170u8, u8::MAX, u8::MIN, 211u8]);
        println!("{}", bitvec);
    }

    #[test]
    fn can_construct() {
        let bitvec = BitVec::from_int(vec![u32::MAX, 1u32]);
        assert_eq!(bitvec.maps.len(), 2);
    }

    #[test]
    fn can_get() {
        let bitvec = BitVec::from_int(vec![u32::MAX, 1u32]);
        for i in 0..32usize {
            assert_eq!(bitvec.get(i), Some(true));
        }
        assert_eq!(bitvec.get(32), Some(true));
        assert_eq!(bitvec.get(33), Some(false));
    }

    #[test]
    fn can_set() {
        let mut bitvec = BitVec::from_int(vec![u32::MIN, u32::MIN]);
        assert_eq!(bitvec.get(32).unwrap(), false);
        bitvec.set(32, true);
        assert_eq!(bitvec.get(32).unwrap(), true);
    }

    #[test]
    fn can_index() {
        let bitvec = BitVec::from_int(vec![u32::MIN, u32::MAX]);
        assert_eq!(bitvec[31], false);
        assert_eq!(bitvec[32], true);
    }

    #[test]
    fn get_range_as_int() {
        let bitvec = BitVec::from_int(vec![170u8, 211u8]);
        let int_1 = bitvec.get_range_as_int(1..=3);
        assert_eq!(int_1, 5); // 101 == 5
        let int_2 = bitvec.get_range_as_int(5..10);
        assert_eq!(int_2, 29); // 11101 == 29
        let int_3 = bitvec.get_range_as_int(0..8);
        assert_eq!(int_3, 170u64)
    }
}
