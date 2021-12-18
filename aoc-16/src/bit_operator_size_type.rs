use crate::from_bitvec::FromBitvec;
use aoc_core::bit::bit_size::BitSize;
use aoc_core::bit::bitvec::BitVec;

#[derive(Debug)]
pub enum BitOperatorSizeType {
    Bits(u16),
    Packets(u16),
}

impl FromBitvec<u8> for BitOperatorSizeType {
    fn from_bitvec(bitvec: &BitVec<u8>, start_bit: usize) -> Self {
        let type_id = bitvec[start_bit];
        let size_start = start_bit + 1;
        match type_id {
            false => {
                let data_range = (size_start..(size_start + 15)).rev();
                let bit_count = bitvec.get_range_as_int(data_range);
                BitOperatorSizeType::Bits(bit_count as u16)
            }
            true => {
                let data_range = (size_start..(size_start + 11)).rev();
                let packet_count = bitvec.get_range_as_int(data_range);
                BitOperatorSizeType::Packets(packet_count as u16)
            }
        }
    }
}

impl BitSize for BitOperatorSizeType {
    fn bit_size(&self) -> usize {
        match self {
            BitOperatorSizeType::Bits(_) => 16,
            BitOperatorSizeType::Packets(_) => 12,
        }
    }
}
