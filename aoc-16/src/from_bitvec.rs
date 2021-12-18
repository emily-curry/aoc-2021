use aoc_core::bit::bit_size::BitSize;
use aoc_core::bit::bitty::Bitty;
use aoc_core::bit::bitvec::BitVec;

pub trait FromBitvec<B>
where
    B: Bitty,
{
    fn from_bitvec(bitvec: &BitVec<B>, start_bit: usize) -> Self
    where
        Self: BitSize;
}
