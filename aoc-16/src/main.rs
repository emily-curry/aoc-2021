use aoc_core::bit::bitmap::Bitmap;
use aoc_core::puzzle_input::PuzzleInput;
use std::collections::HashMap;

fn main() {
    let input = PuzzleInput::new("aoc-16/input.txt");
}

#[derive(Debug)]
struct BITSTransmission {
    maps: Vec<Bitmap>,
}

// impl From<&str> for BITSTransmission {
//     fn from(input: &str) -> Self {
//         let chunks = input.chars().collect::<Vec<_>>().chunks(4);
//
//     }
// }
