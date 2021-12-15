mod element;
mod element_pair;
mod polymer_template;

use crate::polymer_template::PolymerTemplate;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::new("aoc-14/input.txt");
    let mut polymer = PolymerTemplate::from(input.to_lines());
    let mut step = 0u8;

    while step < 10 {
        polymer.step();
        step += 1;
    }

    let least_common = polymer.get_least_common().expect("No elements at all!");
    println!(
        "Least common element after {} steps: {}, {}x",
        step, least_common.0, least_common.1
    );
    let most_common = polymer.get_most_common().expect("No elements at all!");
    println!(
        "Most common element after {} steps: {}, {}x",
        step, most_common.0, most_common.1
    );
    println!(
        "Difference between most common and least common: {}",
        most_common.1 - least_common.1
    );

    while step < 40 {
        println!("Step {}", step + 1);
        polymer.step();
        step += 1;
    }

    let least_common = polymer.get_least_common().expect("No elements at all!");
    println!(
        "Least common element after {} steps: {}, {}x",
        step, least_common.0, least_common.1
    );
    let most_common = polymer.get_most_common().expect("No elements at all!");
    println!(
        "Most common element after {} steps: {}, {}x",
        step, most_common.0, most_common.1
    );
    println!(
        "Difference between most common and least common: {}",
        most_common.1 - least_common.1
    );
}
