use aoc_core::bit::bitvec::BitVec;

pub struct ImageAlgorithm {
    bits: BitVec<u32>,
}

impl ImageAlgorithm {
    pub fn get(&self, position: usize) -> bool {
        self.bits[position]
    }
}

impl From<&str> for ImageAlgorithm {
    fn from(input: &str) -> Self {
        let mut bits = BitVec::with_capacity(512);
        for (index, char) in input.chars().enumerate() {
            let bit = match char {
                '#' => true,
                '.' => false,
                _ => panic!("Invalid value: {}", char),
            };
            bits.set(index, bit);
        }
        ImageAlgorithm { bits }
    }
}
