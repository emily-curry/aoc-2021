use aoc_core::puzzle_input::PuzzleInput;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::Lines;

fn main() {
    let input = PuzzleInput::new("aoc-13/input.txt");
    let mut sheet = DotSheet::from(input.to_lines());
    sheet.fold_next();
    println!("Dots after first fold: {}", sheet.get_dots());
    while sheet.get_next_fold().is_some() {
        sheet.fold_next();
    }
    println!("------- Dot Sheet -------\n{}", sheet);
}

type Dot = (u32, u32);

#[derive(Debug)]
struct DotSheet {
    dots: HashSet<Dot>,
    folds: Vec<DotSheetFold>,
    folds_completed: usize,
}

impl DotSheet {
    pub fn new(dots: HashSet<Dot>, folds: Vec<DotSheetFold>) -> Self {
        DotSheet {
            dots,
            folds,
            folds_completed: 0,
        }
    }

    pub fn fold_next(&mut self) -> Option<DotSheetFold> {
        let next_fold = *self.get_next_fold()?;
        match &next_fold {
            DotSheetFold::Up(line) => self.fold_up(*line),
            DotSheetFold::Left(line) => self.fold_left(*line),
        };

        self.folds_completed += 1;
        Some(next_fold)
    }

    pub fn get_next_fold(&self) -> Option<&DotSheetFold> {
        self.folds.get(self.folds_completed)
    }

    pub fn get_dots(&self) -> usize {
        self.dots.len()
    }

    fn fold_up(&mut self, line: u32) {
        let points_past: Vec<Dot> = self
            .dots
            .iter()
            .filter(|dot| dot.1 > line)
            .map(Dot::clone)
            .collect();
        for dot in points_past {
            self.dots.remove(&dot);
            self.dots.insert((dot.0, line - (dot.1 - line)));
        }
    }

    fn fold_left(&mut self, line: u32) {
        let points_past: Vec<Dot> = self
            .dots
            .iter()
            .filter(|dot| dot.0 > line)
            .map(Dot::clone)
            .collect();
        for dot in points_past {
            self.dots.remove(&dot);
            self.dots.insert((line - (dot.0 - line), dot.1));
        }
    }
}

impl Display for DotSheet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x_bound = self
            .dots
            .iter()
            .fold(0u32, |acc, val| if val.0 > acc { val.0 } else { acc })
            + 1;
        let y_bound = self
            .dots
            .iter()
            .fold(0u32, |acc, val| if val.1 > acc { val.1 } else { acc })
            + 1;

        for y in 0..=y_bound {
            for x in 0..=x_bound {
                let char = if self.dots.contains(&(x, y)) {
                    '█'
                } else {
                    ' '
                };
                f.write_str(char.to_string().as_str())?;
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl<'a> From<Lines<'a>> for DotSheet {
    fn from(input: Lines<'a>) -> Self {
        let mut dots = HashSet::new();
        let mut folds = Vec::new();
        let mut is_dots = true;
        for line in input {
            if line.is_empty() {
                is_dots = false;
                continue;
            }
            if is_dots {
                let mut pair = line.split(',');
                let dot: (u32, u32) = (
                    pair.next()
                        .expect("No first element!")
                        .parse()
                        .expect("Could not parse!"),
                    pair.next()
                        .expect("No second element!")
                        .parse()
                        .expect("Could not parse!"),
                );
                dots.insert(dot);
            } else {
                folds.push(DotSheetFold::from(line));
            }
        }

        DotSheet::new(dots, folds)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum DotSheetFold {
    Left(u32),
    Up(u32),
}

impl From<&str> for DotSheetFold {
    fn from(input: &str) -> Self {
        let mut split = input.split('=');
        let dir_str = split.next().expect("No first element!");
        let line = split
            .next()
            .expect("No second element!")
            .parse()
            .expect("Could not parse!");
        match dir_str {
            "fold along x" => DotSheetFold::Left(line),
            "fold along y" => DotSheetFold::Up(line),
            _ => panic!("Invalid value! Was: {:?}", dir_str),
        }
    }
}
