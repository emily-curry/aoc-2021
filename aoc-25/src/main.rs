use crate::ocean_floor::OceanFloor;
use aoc_core::puzzle_input::PuzzleInput;

mod ocean_floor;
mod sea_cucumber;

fn main() {
    let input = PuzzleInput::new("aoc-25/input.txt");
    let mut floor = OceanFloor::from(input.to_lines());
    let step_settled = floor.step_until_settled();
    println!("First step where no sea cucumbers moved: {}", step_settled);
}
