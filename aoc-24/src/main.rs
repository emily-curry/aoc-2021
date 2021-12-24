use crate::ari_logi_uni::AriLogiUni;
use aoc_core::puzzle_input::PuzzleInput;

mod ari_logi_checkpoint;
mod ari_logi_uni;
mod ari_logi_uni_state;
mod instruction;
mod instruction_set;
mod instruction_state;
mod register;

fn main() {
    let input = PuzzleInput::new("aoc-24/input.txt");
    let alu = AriLogiUni::from(input.to_lines());
    let result = alu.brute_force();
    println!("Found a model number!\n{}", result.unwrap());
}
