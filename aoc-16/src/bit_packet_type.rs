use crate::bit_evaluate::BitEvaluate;
use crate::bit_packet_data_literal::BitPacketDataLiteral;
use crate::bit_packet_data_operator::BitPacketDataOperator;
use crate::bit_packet_version::BitPacketVersion;
use crate::from_bitvec::FromBitvec;
use aoc_core::bit::bit_size::BitSize;
use aoc_core::bit::bitvec::BitVec;

#[derive(Debug)]
pub enum BitPacketType {
    Literal(BitPacketDataLiteral),
    Operator(BitPacketDataOperator),
}

impl FromBitvec<u8> for BitPacketType {
    fn from_bitvec(bitvec: &BitVec<u8>, start_bit: usize) -> Self {
        let type_int = bitvec.get_range_as_int((start_bit..start_bit + 3).rev());
        match type_int {
            4 => BitPacketType::Literal(BitPacketDataLiteral::from_bitvec(bitvec, start_bit + 3)),
            _ => BitPacketType::Operator(BitPacketDataOperator::from_bitvec(bitvec, start_bit)),
        }
    }
}

impl BitEvaluate for BitPacketType {
    fn evaluate(&self) -> u64 {
        match self {
            BitPacketType::Literal(lit) => lit.evaluate(),
            BitPacketType::Operator(op) => op.evaluate(),
        }
    }
}

impl BitSize for BitPacketType {
    fn bit_size(&self) -> usize {
        match self {
            BitPacketType::Literal(lit) => lit.bit_size() + 3, // +3 to account for the opcode size
            BitPacketType::Operator(op) => op.bit_size(),      // No +3, internal opcode adds it.
        }
    }
}

impl BitPacketVersion for BitPacketType {
    fn get_version(&self) -> Option<u8> {
        None
    }

    fn sum_version(&self) -> u64 {
        match self {
            BitPacketType::Literal(_) => 0,
            BitPacketType::Operator(op) => op.sum_version(),
        }
    }
}
