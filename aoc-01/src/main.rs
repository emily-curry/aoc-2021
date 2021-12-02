use aoc_core::puzzle_input::PuzzleInput;
use std::collections::VecDeque;
use std::ops::Range;
use std::slice::Iter;
use std::str::Lines;
use std::{i64, isize};

fn main() {
    let input = PuzzleInput::new("aoc-01/input.txt");
    let summer = DepthSummer::new(input.to_lines());
    println!(
        "Number of increments with window [ 1 ]: {:?}",
        summer.sum(1)
    );
    println!(
        "Number of increments with window [ 3 ]: {:?}",
        summer.sum(3)
    );
}

struct DepthSummer {
    lines: Vec<i32>,
}

impl DepthSummer {
    fn new(lines: Lines) -> Self {
        DepthSummer {
            lines: lines.map(|x| x.clone().parse().unwrap()).collect(),
        }
    }

    fn sum(&self, window_size: usize) -> i32 {
        let mut sum = 0;
        // Create a vec with enough space for the previous window and the current window.
        let mut window: VecDeque<Option<&i32>> = VecDeque::from(vec![None; window_size + 1]);
        // Iterate over every line in the input.
        for (i, val) in self.lines.iter().enumerate() {
            window.pop_front(); // Remove the first element from the window.
            window.push_back(Some(val)); // And add the current element to the window.
            if window.iter().any(|x| x.is_none()) {
                continue; // If any element in our prev/curr window vec is None, we can't sum, continue to the next line.
            }

            let mut prev_window_sum = 0;
            let mut curr_window_sum = 0;
            for (window_index, window_value) in window.iter().map(|x| x.unwrap()).enumerate() {
                if window_index != 0 {
                    curr_window_sum += window_value; // If we're within the window for the current depth, update the current sum.
                }
                if window_index != window.len() - 1 {
                    prev_window_sum += window_value; // If we're within the window for the previous depth, update the previous sum.
                }
            }
            if curr_window_sum > prev_window_sum {
                sum += 1; // If the current window has a greater depth sum than the previous depth window, increment our overall sum.
            }
        }
        sum
    }
}
