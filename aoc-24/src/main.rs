use crate::ari_logi_uni::AriLogiUni;
use aoc_core::puzzle_input::PuzzleInput;

mod ari_logi_uni;
mod instruction;
mod instruction_set;
mod instruction_set_pair;
mod register;

/// The explanation for this insanity is in instruction_set.rs, where I originally tried to parse and execute all the instructions,
/// and then realized there were parts that repeated that I could maybe optimize,
/// and then by reducing those realized that testing inputs was totally 100% the wrong approach.
///
/// There's a lot of dead code in here because of that (RIP Instruction/Register, I enjoyed implementing you 💔).
fn main() {
    let input = PuzzleInput::new("aoc-24/input.txt");
    let alu = AriLogiUni::from(input.to_lines());
    let (max, min) = alu.solve();
    print!("Highest possible model number: ");
    for v in max {
        print!("{}", v);
    }
    println!(" ");
    print!("Lowest possible model number: ");
    for v in min {
        print!("{}", v);
    }
}
