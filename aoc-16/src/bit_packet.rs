use crate::bit_evaluate::BitEvaluate;
use crate::bit_packet_type::BitPacketType;
use crate::bit_packet_version::BitPacketVersion;
use crate::from_bitvec::FromBitvec;
use aoc_core::bit::bit_size::BitSize;
use aoc_core::bit::bitvec::BitVec;

#[derive(Debug)]
pub struct BitPacket {
    version: u8,
    data: BitPacketType,
}

impl FromBitvec<u8> for BitPacket {
    fn from_bitvec(bitvec: &BitVec<u8>, start_bit: usize) -> Self {
        let version = bitvec.get_range_as_int((start_bit..start_bit + 3).rev()) as u8;
        let data = BitPacketType::from_bitvec(bitvec, start_bit + 3);
        BitPacket { version, data }
    }
}

impl BitEvaluate for BitPacket {
    fn evaluate(&self) -> u64 {
        self.data.evaluate()
    }
}

impl BitSize for BitPacket {
    fn bit_size(&self) -> usize {
        3 + self.data.bit_size()
    }
}

impl BitPacketVersion for BitPacket {
    fn get_version(&self) -> Option<u8> {
        Some(self.version)
    }

    fn sum_version(&self) -> u64 {
        self.data.sum_version() + self.version as u64
    }
}
