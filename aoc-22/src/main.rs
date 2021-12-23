use crate::cube_reactor::CubeReactor;
use aoc_core::puzzle_input::PuzzleInput;
use std::time::SystemTime;

mod cube_reactor;
mod cube_reactor_instruction;
mod cuboid_cube;
mod cuboid_set;
mod cuboid_state;

fn main() {
    let input = PuzzleInput::new("aoc-22/input.txt");
    let mut reactor = CubeReactor::from(input.to_lines());
    let start = SystemTime::now();
    reactor.reboot_init();
    println!(
        "Reboot initialization sequence complete, enabled {} cuboids in {}s",
        reactor.count(),
        start.elapsed().unwrap().as_secs_f32()
    );

    reactor.reboot();
    println!(
        "Full reboot complete, enabled {} cuboids in {}s",
        reactor.count(),
        start.elapsed().unwrap().as_secs_f32()
    );
}
