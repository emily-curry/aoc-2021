use aoc_core::puzzle_input::PuzzleInput;
use colored::*;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::Lines;

fn main() {
    let input = PuzzleInput::new("aoc-09/input.txt");
    let depth_map: DepthMap = input.to_lines().into();
    println!("Depth Map:\n{}", depth_map);
    let low_points = depth_map.get_local_minimums();
    let risk_levels: Vec<u8> = low_points.iter().map(|x| x.2 + 1).collect();
    let risk_level_sum = risk_levels.iter().fold(0u32, |acc, val| acc + *val as u32);
    println!("Sum of risk levels at low points: {}", risk_level_sum);

    let mut basins = depth_map.get_basins();
    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    let basin_product = basins[0].len() * basins[1].len() * basins[2].len();
    println!("Product of 3 largest basins: {}", basin_product);
}

type Point = (usize, usize, u8);

type Basin = HashSet<Point>;

struct DepthMap {
    map: [[u8; 100]; 100],
}

impl DepthMap {
    pub fn new(map: [[u8; 100]; 100]) -> Self {
        DepthMap { map }
    }

    pub fn get_local_minimums(&self) -> Vec<Point> {
        let mut result = Vec::new();

        for (x, col) in self.map.iter().enumerate() {
            for (y, value) in col.iter().enumerate() {
                let point = (x, y, *value);
                if self.is_local_minimum(&point) {
                    result.push(point);
                }
            }
        }

        result
    }

    pub fn get_basins(&self) -> Vec<Basin> {
        self.get_local_minimums()
            .iter()
            .map(|low_point| self.find_basin(*low_point)) // Every basin will contain at least a low point, so we can start there and build outwards
            .collect()
    }

    fn find_basin(&self, point: Point) -> Basin {
        let mut basin = HashSet::new();
        basin.insert(point);
        for adj in self.get_adjacent_points(&point) {
            // If the adjacent point is greater than the current point, the adjacent point's value is not 9, and the basin doesn't already contain that point,
            // then add that point and all points adjacent to it that meet the above criteria (recursively) to the basin.
            if adj.2 > point.2 && adj.2 != 9 && !basin.contains(&adj) {
                basin.extend(self.find_basin(adj));
            }
        }

        basin
    }

    fn is_local_minimum(&self, point: &Point) -> bool {
        let points = self.get_adjacent_points(point);
        points.iter().all(|adj| point.2 < adj.2)
    }

    fn get_adjacent_points(&self, point: &Point) -> Vec<Point> {
        let mut result = Vec::new();
        if point.0 > 0 {
            result.push((point.0 - 1, point.1, self.map[point.0 - 1][point.1]));
        }
        if point.0 < 99 {
            result.push((point.0 + 1, point.1, self.map[point.0 + 1][point.1]));
        }
        if point.1 > 0 {
            result.push((point.0, point.1 - 1, self.map[point.0][point.1 - 1]));
        }
        if point.1 < 99 {
            result.push((point.0, point.1 + 1, self.map[point.0][point.1 + 1]));
        }
        result
    }
}

impl<'a> From<Lines<'a>> for DepthMap {
    fn from(input: Lines) -> Self {
        let mut map = [[0u8; 100]; 100];
        for (y, line) in input.enumerate() {
            for (x, value) in line.chars().enumerate() {
                let parsed: u8 = value.to_digit(10).expect("Could not parse!") as u8;
                map[x][y] = parsed;
            }
        }
        DepthMap::new(map)
    }
}

impl Display for DepthMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let basins = self.get_basins();
        for y in 0..100 {
            for x in 0..100 {
                let char = &self.map[x][y];
                let point = (x, y, *char);
                let mut char_str: ColoredString = format!("{}", char).as_str().into();
                if !basins.iter().any(|basin| basin.contains(&point)) {
                    char_str = char_str.bold().on_bright_white();
                }
                if self.is_local_minimum(&point) {
                    char_str = char_str.red();
                }
                f.write_str(format!("{}", char_str).as_str())?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}
