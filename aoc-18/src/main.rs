use crate::snail_pair::SnailPair;
use aoc_core::puzzle_input::PuzzleInput;
use std::collections::{HashMap, HashSet};

mod snail_pair;

fn main() {
    let input = PuzzleInput::new("aoc-18/input.txt");
    let mut snail_pairs = input.to_lines().map(SnailPair::from);
    let first = snail_pairs.next().unwrap();
    let snail_sum = snail_pairs.fold(first, |acc, val| acc + val);
    println!("Final sum: \n{:?}", snail_sum);
    let magnitude = snail_sum.get_magnitude();
    println!("Magnitude of final sum: {}", magnitude);

    let snail_pairs = input
        .to_lines()
        .map(SnailPair::from)
        .collect::<HashSet<_>>();
    let mut magnitudes = HashMap::<(SnailPair, SnailPair), u64>::new();
    for left in &snail_pairs {
        for right in &snail_pairs {
            if left == right {
                continue;
            }
            let mag = (left.clone() + right.clone()).get_magnitude();
            magnitudes.insert((left.clone(), right.clone()), mag);
        }
    }
    let greatest_magnitude = magnitudes
        .values()
        .fold(0u64, |acc, val| if &acc > val { acc } else { *val });
    println!(
        "Greatest magnitude of single addition: {}",
        greatest_magnitude
    );
}
