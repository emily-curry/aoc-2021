use crate::from_bitvec::FromBitvec;
use crate::BitEvaluate;
use aoc_core::bit::bit_size::BitSize;
use aoc_core::bit::bitvec::BitVec;

#[derive(Debug)]
pub struct BitPacketDataLiteral {
    value: u64,
    value_size: usize,
}

impl FromBitvec<u8> for BitPacketDataLiteral {
    fn from_bitvec(bitvec: &BitVec<u8>, start_bit: usize) -> Self {
        let mut result = 0u64;
        let mut size = 0;
        loop {
            let idx = start_bit + size;
            let continue_reading = bitvec[idx];
            let next_range = ((idx + 1)..(idx + 5)).rev();
            let next = bitvec.get_range_as_int(next_range);
            result = result << 4;
            result = result | next;
            size += 5;
            if !continue_reading {
                break;
            }
        }
        // Leaving this in because it was the source of _hours_ of grief.
        // I interpreted the instructions as "literal values are padded to the nearest hex nibble", but this is not correct.
        // Unit tests ended up saving the day, bless rust for making that so easy.
        // if size % 4 != 0 {
        //     size += 4 - (size % 4);
        // }
        BitPacketDataLiteral {
            value: result,
            value_size: size,
        }
    }
}

impl BitEvaluate for BitPacketDataLiteral {
    fn evaluate(&self) -> u64 {
        self.value
    }
}

impl BitSize for BitPacketDataLiteral {
    fn bit_size(&self) -> usize {
        self.value_size
    }
}
