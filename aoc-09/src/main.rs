use aoc_core::intmap::{IntMap, IntMapPoint};
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
    debug_assert_eq!(risk_level_sum, 480);

    let mut basins = depth_map.get_basins();
    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    let basin_product = basins[0].len() * basins[1].len() * basins[2].len();
    println!("Product of 3 largest basins: {}", basin_product);
    debug_assert_eq!(basin_product, 1045660);
}

type Basin = HashSet<IntMapPoint>;

struct DepthMap {
    inner: IntMap,
}

impl DepthMap {
    pub fn new(map: IntMap) -> Self {
        DepthMap { inner: map }
    }

    pub fn get_local_minimums(&self) -> Vec<IntMapPoint> {
        let mut result = Vec::new();

        for point in self.inner.iter_points() {
            if self.is_local_minimum(&point) {
                result.push(point);
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

    fn find_basin(&self, point: IntMapPoint) -> Basin {
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

    fn is_local_minimum(&self, point: &IntMapPoint) -> bool {
        let points = self.get_adjacent_points(point);
        points.iter().all(|adj| point.2 < adj.2)
    }

    fn get_adjacent_points(&self, point: &IntMapPoint) -> Vec<IntMapPoint> {
        let mut result = Vec::new();
        if point.0 > 0 {
            result.push(self.inner.get_point(point.0 - 1, point.1));
        }
        if point.0 < 99 {
            result.push(self.inner.get_point(point.0 + 1, point.1));
        }
        if point.1 > 0 {
            result.push(self.inner.get_point(point.0, point.1 - 1));
        }
        if point.1 < 99 {
            result.push(self.inner.get_point(point.0, point.1 + 1));
        }
        result
    }
}

impl<'a> From<Lines<'a>> for DepthMap {
    fn from(input: Lines) -> Self {
        let intmap = IntMap::from(input);
        DepthMap::new(intmap)
    }
}

impl Display for DepthMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let basins = self.get_basins();
        let mut y = 0usize;
        for point in self.inner.iter_points() {
            if point.1 != y {
                f.write_str("\n")?;
                y = point.1;
            }
            let mut char_str: ColoredString = format!("{}", point.2).as_str().into();
            if !basins.iter().any(|basin| basin.contains(&point)) {
                char_str = char_str.bold().on_bright_white();
            }
            if self.is_local_minimum(&point) {
                char_str = char_str.red();
            }
            f.write_str(format!("{}", char_str).as_str())?;
        }
        Ok(())
    }
}
