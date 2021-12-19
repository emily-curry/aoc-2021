mod beacon;
mod beacon_map;
mod rotation;
mod scanner;

use crate::beacon_map::BeaconMap;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::new("aoc-19/input.txt");
    let mut map = BeaconMap::from(input.to_lines());
    map.associate_all();
    let beacon_count = map.count();
    println!("Final beacon count: {}", beacon_count);
    let max_distance = map.find_max_distance();
    println!("Maximum distance between any two beacons: {}", max_distance);
}
