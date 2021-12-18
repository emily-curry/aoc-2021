use crate::bit_evaluate::BitEvaluate;
use crate::bit_packet::BitPacket;
use crate::bit_packet_version::BitPacketVersion;
use crate::from_bitvec::FromBitvec;
use aoc_core::bit::bit_size::BitSize;
use aoc_core::bit::bitvec::BitVec;
use std::ops::Index;

#[derive(Debug)]
pub struct BitTrans {
    bits: BitVec<u8>,
    packet: BitPacket,
}

impl BitTrans {
    pub fn new(bits: BitVec<u8>) -> Self {
        let packet = BitPacket::from_bitvec(&bits, 0);
        BitTrans { bits, packet }
    }

    fn chars_to_bitvec(chars: Vec<char>) -> BitVec<u8> {
        if chars.len() % 2 != 0 {
            panic!("Must be a multiple of 2!");
        }
        let pairs = chars
            .chunks(2)
            .map(|win| String::from_iter(win.iter()))
            .map(|st| u8::from_str_radix(st.as_str(), 16))
            .map(|res| res.expect("Could not parse!"))
            .map(|val| val.reverse_bits()) // Reverse the order of the bits, we want to read/index left-to-right (index 0 is the *most* significant bit of the first block)
            .collect::<Vec<_>>();
        let bits = BitVec::from_int(pairs);
        bits
    }
}

impl From<Vec<char>> for BitTrans {
    fn from(chars: Vec<char>) -> Self {
        BitTrans::new(BitTrans::chars_to_bitvec(chars))
    }
}

impl Index<usize> for BitTrans {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bits[index]
    }
}

impl BitPacketVersion for BitTrans {
    fn get_version(&self) -> Option<u8> {
        None
    }

    fn sum_version(&self) -> u64 {
        self.packet.sum_version()
    }
}

impl BitEvaluate for BitTrans {
    fn evaluate(&self) -> u64 {
        self.packet.evaluate()
    }
}

impl BitSize for BitTrans {
    fn bit_size(&self) -> usize {
        self.bits.bit_size()
    }
}

#[cfg(test)]
mod tests {
    use crate::bit_trans::BitTrans;
    use crate::BitPacketVersion;
    use aoc_core::bit::bit_size::BitSize;
    use aoc_core::puzzle_input::PuzzleInput;

    #[test]
    fn test_known_input() {
        let input = PuzzleInput::new("../aoc-16/input.txt");
        let bits = BitTrans::from(input.as_string().chars().collect::<Vec<_>>());
        // 02 should be (left-to-right is 0-to-7) 00000010
        for i in 0..8usize {
            match i {
                6 => assert_eq!(bits[i], true),
                _ => assert_eq!(bits[i], false),
            }
        }
        // 8c should be 10001100
        for i in 48..56usize {
            match i {
                48 | 52 | 53 => assert_eq!(bits[i], true),
                _ => assert_eq!(bits[i], false),
            }
        }
        // 70 should be 01110000
        for i in 16..24usize {
            match i {
                17 | 18 | 19 => assert_eq!(bits[i], true),
                _ => assert_eq!(bits[i], false),
            }
        }
    }

    #[test]
    fn bitvec_input() {
        let input = PuzzleInput::new("../aoc-16/input.txt");
        let bitvec = BitTrans::chars_to_bitvec(input.as_string().chars().collect());
        let first_size = bitvec.get_range_as_int((7..18).rev());
        assert_eq!(first_size, 53);
    }

    #[test]
    fn display_bitvec() {
        let input = PuzzleInput::new("../aoc-16/input.txt");
        let bitvec = BitTrans::chars_to_bitvec(input.as_string().chars().collect());
        println!("{}", bitvec);
    }

    #[test]
    fn example_input() {
        let result = bools_from_binary_string("11010010 11111110 00101000");
        let bitvec = BitTrans::chars_to_bitvec("D2FE28".chars().collect());
        assert_eq!(result.len(), bitvec.bit_size());
        for (idx, val) in result.iter().enumerate() {
            assert_eq!(&bitvec[idx], val);
        }
        assert_eq!(bitvec.get_range_as_int((0..3).rev()), 6);
        assert_eq!(bitvec.get_range_as_int((3..6).rev()), 4);
        assert_eq!(bitvec[6], true);
        assert_eq!(bitvec.get_range_as_int((7..11).rev()), 7);
        assert_eq!(bitvec[11], true);
        assert_eq!(bitvec.get_range_as_int((12..16).rev()), 14);
        assert_eq!(bitvec[16], false);
        assert_eq!(bitvec.get_range_as_int((17..21).rev()), 5);
        assert_eq!(bitvec[21], false);
        assert_eq!(bitvec[22], false);
        assert_eq!(bitvec[23], false);

        let bittrans = BitTrans::new(bitvec);
        assert_eq!(bittrans.sum_version(), 6);
        assert_eq!(bittrans.bit_size(), 24);
    }

    #[test]
    fn example_input_2() {
        let result = bools_from_binary_string(
            "0011 1000 0000 0000 0110 1111 0100 0101 0010 1001 0001 0010 0000 0000",
        );
        let bitvec = BitTrans::chars_to_bitvec("38006F45291200".chars().collect());
        assert_eq!(result.len(), bitvec.bit_size());
        for (idx, val) in result.iter().enumerate() {
            assert_eq!(&bitvec[idx], val);
        }
        let bittrans = BitTrans::new(bitvec);
        assert_eq!(bittrans.packet.get_version().unwrap(), 1);
        assert_eq!(bittrans.sum_version(), 9);
    }

    fn bools_from_binary_string(bin: &str) -> Vec<bool> {
        bin.chars()
            .filter(|c| c == &'1' || c == &'0')
            .map(|c| match c {
                '1' => true,
                '0' => false,
                _ => panic!("Invalid char!"),
            })
            .collect::<Vec<_>>()
    }
}
