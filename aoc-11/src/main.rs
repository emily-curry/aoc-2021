mod octomap;

use aoc_core::puzzle_input::PuzzleInput;
use octomap::OctoMap;

fn main() {
    let input = PuzzleInput::new("aoc-11/input.txt");
    let mut map = OctoMap::new(input.to_lines().into());
    let mut flashes = 0u64;
    for _ in 0..100 {
        flashes += map.step();
    }
    println!("Flashes after 100 steps: {}", flashes);
    debug_assert_eq!(flashes, 1785);
    let mut step = 100usize;
    loop {
        step += 1;
        let step_flashes = map.step();
        if step_flashes == map.get_size() as u64 {
            break;
        }
    }
    println!("Every octopi flashed on step: {}", step);
    debug_assert_eq!(step, 354);
}
