use std::str::Lines;

use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::new("aoc-02/input.txt");
    let nav = SubNavigation::new(input.to_lines());
    let position_basic = nav.navigate_basic();
    println!(
        "Computed initial distance: {:?}",
        position_basic.horizontal * position_basic.depth
    );
    let position_adv = nav.navigate_advanced();
    println!(
        "Computed distance with aim: {:?}",
        position_adv.horizontal * position_adv.depth
    )
}

struct SubPosition {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl SubPosition {
    fn new() -> Self {
        SubPosition {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

struct SubNavigation {
    movement: Vec<SubMovement>,
}

impl SubNavigation {
    fn new(lines: Lines) -> Self {
        // From<&str> is impl for SubMovement, and since movement is returned as part of SubNavigation, its type can be inferred, so our map lambda can just call into() to automatically convert each &str element to a SubMovement. Compilers!
        let movement = lines.map(|x| x.into()).collect();

        SubNavigation { movement }
    }

    fn navigate_basic(&self) -> SubPosition {
        let mut pos = SubPosition::new();
        for step in &self.movement {
            match step {
                SubMovement::Forward(dist) => {
                    pos.horizontal += dist;
                }
                SubMovement::Down(dist) => {
                    pos.depth += dist;
                }
                SubMovement::Up(dist) => {
                    pos.depth -= dist;
                }
            }
        }
        pos
    }

    fn navigate_advanced(&self) -> SubPosition {
        let mut pos = SubPosition::new();
        for step in &self.movement {
            match step {
                SubMovement::Forward(dist) => {
                    pos.horizontal += dist;
                    pos.depth += pos.aim * dist;
                }
                SubMovement::Down(dist) => {
                    pos.aim += dist;
                }
                SubMovement::Up(dist) => {
                    pos.aim -= dist;
                }
            }
        }
        pos
    }
}

#[derive(Debug)]
enum SubMovement {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl From<&str> for SubMovement {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split(" ").collect();
        let dir = *split.get(0).unwrap();
        let dist: i32 = split.get(1).unwrap().clone().parse().unwrap();
        match dir {
            "down" => SubMovement::Down(dist),
            "up" => SubMovement::Up(dist),
            "forward" => SubMovement::Forward(dist),
            _ => panic!(),
        }
    }
}
