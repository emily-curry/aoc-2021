use crate::bit_operator_size_type::BitOperatorSizeType;
use crate::bit_operator_type::BitOperatorType;
use crate::bit_packet::BitPacket;
use crate::from_bitvec::FromBitvec;
use crate::{BitEvaluate, BitPacketVersion};
use aoc_core::bit::bit_size::BitSize;
use aoc_core::bit::bitvec::BitVec;

#[derive(Debug)]
pub struct BitPacketDataOperator {
    op: BitOperatorType,
    sub: Vec<BitPacket>,
    sub_size: BitOperatorSizeType,
}

impl FromBitvec<u8> for BitPacketDataOperator {
    fn from_bitvec(bitvec: &BitVec<u8>, start_bit: usize) -> Self {
        let type_int = bitvec.get_range_as_int((start_bit..start_bit + 3).rev());
        let op = BitOperatorType::from(type_int as u8);
        let sub_size = BitOperatorSizeType::from_bitvec(bitvec, start_bit + 3);

        let sub = match sub_size {
            BitOperatorSizeType::Bits(bit_count) => {
                let mut sub = Vec::new();
                let initial_offset = start_bit + op.bit_size() + sub_size.bit_size();
                let mut sub_offset = initial_offset;
                while sub_offset - initial_offset < (bit_count as usize) {
                    let sub_packet = BitPacket::from_bitvec(bitvec, sub_offset);
                    sub_offset += sub_packet.bit_size();
                    sub.push(sub_packet);
                }

                sub
            }
            BitOperatorSizeType::Packets(packet_count) => {
                let mut sub = Vec::new();
                let mut sub_offset = start_bit + op.bit_size() + sub_size.bit_size();
                for _ in 0..packet_count {
                    let sub_packet = BitPacket::from_bitvec(bitvec, sub_offset);
                    sub_offset += sub_packet.bit_size();
                    sub.push(sub_packet);
                }
                sub
            }
        };

        BitPacketDataOperator { op, sub, sub_size }
    }
}

impl BitEvaluate for BitPacketDataOperator {
    fn evaluate(&self) -> u64 {
        match self.op {
            BitOperatorType::Sum => self.sub.iter().fold(0, |acc, val| acc + val.evaluate()),
            BitOperatorType::Product => self.sub.iter().fold(1, |acc, val| acc * val.evaluate()),
            BitOperatorType::Minimum => self
                .sub
                .iter()
                .fold(u64::MAX, |acc, val| acc.min(val.evaluate())),
            BitOperatorType::Maximum => self
                .sub
                .iter()
                .fold(u64::MIN, |acc, val| acc.max(val.evaluate())),
            BitOperatorType::GreaterThan => {
                if self.sub.len() != 2 {
                    panic!("Illegal state, expected exactly 2 sub-packets, got: {}", 2);
                }
                if self.sub[0].evaluate() > self.sub[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            BitOperatorType::LessThan => {
                if self.sub.len() != 2 {
                    panic!("Illegal state, expected exactly 2 sub-packets, got: {}", 2);
                }
                if self.sub[0].evaluate() < self.sub[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            BitOperatorType::Equal => {
                if self.sub.len() != 2 {
                    panic!("Illegal state, expected exactly 2 sub-packets, got: {}", 2);
                }
                if self.sub[0].evaluate() == self.sub[1].evaluate() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl BitSize for BitPacketDataOperator {
    fn bit_size(&self) -> usize {
        self.sub_size.bit_size()
            + self.op.bit_size()
            + self.sub.iter().fold(0, |acc, val| acc + val.bit_size())
    }
}

impl BitPacketVersion for BitPacketDataOperator {
    fn get_version(&self) -> Option<u8> {
        None
    }

    fn sum_version(&self) -> u64 {
        self.sub
            .iter()
            .map(BitPacket::sum_version)
            .fold(0, |acc, val| acc + val)
    }
}
