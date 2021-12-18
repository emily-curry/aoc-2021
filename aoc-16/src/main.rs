mod bit_evaluate;
mod bit_operator_size_type;
mod bit_operator_type;
mod bit_packet;
mod bit_packet_data_literal;
mod bit_packet_data_operator;
mod bit_packet_type;
mod bit_packet_version;
mod bit_trans;
mod from_bitvec;

use crate::bit_evaluate::BitEvaluate;
use crate::bit_packet_version::BitPacketVersion;
use crate::bit_trans::BitTrans;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::new("aoc-16/input.txt");
    let bits = BitTrans::from(input.as_string().chars().collect::<Vec<_>>());
    println!("Sum of all packet versions: {}", bits.sum_version());
    let result = bits.evaluate();
    println!("Result of evaluation: {}", result);
}
