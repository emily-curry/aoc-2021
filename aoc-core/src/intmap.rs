use std::fmt::{Display, Formatter};
use std::str::Lines;

pub type IntMapPoint = (usize, usize, u8);

/// Represents a grid of single-digit integers.
pub struct IntMap {
    /// The grid of integers, where the index of the vector represents the y position,
    /// and the value is a vector representing a row of integers where the index is the x position.
    map: Vec<Vec<u8>>,
}

impl IntMap {
    pub fn new(map: Vec<Vec<u8>>) -> Self {
        IntMap { map }
    }

    /// Returns the horizontal size of the map (not the last index).
    pub fn get_width(&self) -> usize {
        self.map[0].len()
    }

    /// Returns the vertical size of the map (not the last index).
    pub fn get_height(&self) -> usize {
        self.map.len()
    }

    /// Returns a specific point in the map. Points are 0-indexed.
    ///
    /// Panics if a provided x or y is outside the range of the map.
    pub fn get_point(&self, x: usize, y: usize) -> IntMapPoint {
        (x, y, self.map[y][x])
    }

    /// Sets a specific point in the map. Points are 0-indexed.
    ///
    /// Panics if a provided x or y is outside the range of the map.
    pub fn set_point(&mut self, point: &IntMapPoint) {
        self.map[point.1][point.0] = point.2;
    }

    /// Returns an iterator over all points in the map, from left to right, top to bottom.
    pub fn iter_points(&self) -> impl Iterator<Item = IntMapPoint> + '_ {
        self.map
            .iter()
            .enumerate()
            .flat_map(|c| c.1.iter().enumerate().map(move |r| (r.0, c.0, *r.1)))
    }
}

impl<'a> From<Lines<'a>> for IntMap {
    fn from(input: Lines) -> Self {
        let lines: Vec<&str> = input.collect();
        let height = lines.len();
        let width: usize = lines.first().expect("No first element in lines!").len();
        let mut map = vec![vec![0u8; width]; height];
        for (y, line) in lines.iter().enumerate() {
            for (x, value) in line.chars().enumerate() {
                let parsed: u8 = value.to_digit(10).expect("Could not parse!") as u8;
                map[y][x] = parsed;
            }
        }
        IntMap::new(map)
    }
}

impl Display for IntMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut y = 0usize;
        for point in self.iter_points() {
            if point.1 != y {
                f.write_str("\n")?;
                y = point.1;
            }
            let char_str = format!("{}", point.2);
            f.write_str(format!("{}", char_str).as_str())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::intmap::{IntMap, IntMapPoint};
    use crate::puzzle_input::PuzzleInput;

    fn construct(initial_value: u8) -> IntMap {
        let inner = vec![vec![initial_value; 100]; 100];
        IntMap::new(inner)
    }

    #[test]
    fn can_construct() {
        let map = construct(0u8);
        assert!(true, "Didn't panic!")
    }

    #[test]
    fn can_get_point() {
        let map = construct(9u8);
        let point = map.get_point(1, 2);
        assert_eq!(point.0, 1);
        assert_eq!(point.1, 2);
        assert_eq!(point.2, 9);
    }

    #[test]
    fn can_set_point() {
        let mut map = construct(0u8);
        let point: IntMapPoint = (77, 88, 4);
        let map_point = map.get_point(point.0, point.1);
        assert_eq!(map_point.2, 0);

        map.set_point(&point);
        let map_point_2 = map.get_point(point.0, point.1);
        assert_eq!(map_point_2.0, point.0);
        assert_eq!(map_point_2.1, point.1);
        assert_eq!(map_point_2.2, point.2);
    }

    #[test]
    fn can_iter_points() {
        let map = construct(0u8);
        let v: Vec<IntMapPoint> = map.iter_points().collect();
        // Assert iter size is map's w*h.
        assert_eq!(v.len(), map.get_width() * map.get_height());
        // Assert first item is the top-left corner.
        assert_eq!(v[0].0, 0);
        assert_eq!(v[0].1, 0);
        // Assert iter starts with x-axis first.
        for i in 1..100usize {
            assert_eq!(v[i].0, i);
            assert_eq!(v[i].1, 0);
        }
        // Assert iter continues with next row.
        assert_eq!(v[100].0, 0);
        assert_eq!(v[100].1, 1);
        // General-purpose assert all.
        for (i, point) in map.iter_points().enumerate() {
            let x = i % 100;
            let y = (i - x) / 100;
            assert_eq!(point.0, x);
            assert_eq!(point.1, y);
        }
    }

    #[test]
    fn from_lines() {
        let input = PuzzleInput::new("../aoc-09/input.txt");
        let lines = input.to_lines();
        let map = IntMap::from(lines);
        let point_1 = map.get_point(3, 2);
        assert_eq!(point_1.0, 3);
        assert_eq!(point_1.1, 2);
        assert_eq!(point_1.2, 6);
        let point_2 = map.get_point(70, 50);
        assert_eq!(point_2.0, 70);
        assert_eq!(point_2.1, 50);
        assert_eq!(point_2.2, 5);
    }
}
