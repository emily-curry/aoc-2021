mod risk_map;
mod risk_map_point;

use crate::risk_map::RiskMap;
use aoc_core::intmap::IntMap;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::new("aoc-15/input.txt");
    let map = RiskMap::new(input.to_lines().into());
    let least_risky = map.get_path().expect("No value returned!");
    println!("Risk score of least risky path: {}", least_risky);

    let mut big_map_data: Vec<Vec<u8>> = Vec::new();
    for y_mod in 0..5usize {
        for y_base in 0..map.risks.get_height() {
            let mut big_map_row = Vec::new();

            for x_mod in 0..5usize {
                for x_base in 0..map.risks.get_height() {
                    let point = map.risks.get_point(x_base, y_base);
                    let mut value = point.2 as usize + y_mod + x_mod;
                    while value > 9 {
                        value -= 9;
                    }
                    big_map_row.push(value as u8);
                }
            }

            big_map_data.push(big_map_row);
        }
    }
    let big_map = RiskMap::new(IntMap::new(big_map_data));
    let big_risky = big_map.get_path().expect("No path found!");
    println!("Best path through Big Risky: {}", big_risky);
}
