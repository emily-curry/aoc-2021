use crate::sea_cucumber::SeaCucumber;
use std::fmt::{Display, Formatter};
use std::str::Lines;

const Y_MAX: usize = 137;
const X_MAX: usize = 139;

type OceanFloorMap = [[SeaCucumber; X_MAX]; Y_MAX];

pub struct OceanFloor {
    map: OceanFloorMap,
    step: u32,
}

impl OceanFloor {
    pub fn step_until_settled(&mut self) -> u32 {
        loop {
            let moved = self.step();
            if moved == 0 {
                break;
            }
        }
        self.step
    }

    fn step(&mut self) -> usize {
        let mut moves: usize = 0;
        for (rx, ry) in self.get_moves(SeaCucumber::Right) {
            self.move_point(rx, ry);
            moves += 1;
        }
        for (dx, dy) in self.get_moves(SeaCucumber::Down) {
            self.move_point(dx, dy);
            moves += 1;
        }
        self.step += 1;
        moves
    }

    fn get_moves(&self, t: SeaCucumber) -> Vec<(usize, usize)> {
        let delta_x = if t == SeaCucumber::Right { 1 } else { 0usize };
        let delta_y = if t == SeaCucumber::Down { 1 } else { 0usize };
        self.map
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .filter(|(_, c)| *c == t)
                    .map(move |(x, _)| (x, y))
            })
            .filter(move |(x, y)| *self.get_point(x + delta_x, y + delta_y) == SeaCucumber::None)
            .collect()
    }

    fn get_coords(&self, x: usize, y: usize) -> (usize, usize) {
        let eff_x = if x >= X_MAX { x - X_MAX } else { x };
        let eff_y = if y >= Y_MAX { y - Y_MAX } else { y };
        (eff_x, eff_y)
    }

    fn get_point(&self, x: usize, y: usize) -> &SeaCucumber {
        let (eff_x, eff_y) = self.get_coords(x, y);
        &self.map[eff_y][eff_x]
    }

    fn move_point(&mut self, x: usize, y: usize) {
        let next_type = self.map[y][x]; // Direct-access is fine since move-from points should always be in-bounds.
        let (nx, ny) = match &next_type {
            SeaCucumber::Right => self.get_coords(x + 1, y),
            SeaCucumber::Down => self.get_coords(x, y + 1),
            SeaCucumber::None => panic!("Tried to move nothing!"),
        };
        debug_assert_eq!(self.map[ny][nx], SeaCucumber::None);
        self.map[y][x] = SeaCucumber::None;
        self.map[ny][nx] = next_type;
    }
}

impl From<Lines<'_>> for OceanFloor {
    fn from(input: Lines<'_>) -> Self {
        let mut map = [[SeaCucumber::default(); X_MAX]; Y_MAX];
        for (y, line) in input.enumerate() {
            for (x, char) in line.chars().enumerate() {
                map[y][x] = SeaCucumber::from(char);
            }
        }
        OceanFloor { map, step: 0 }
    }
}

impl Display for OceanFloor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in &self.map {
            for x in y {
                f.write_fmt(format_args!("{}", x))?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ocean_floor::OceanFloor;
    use aoc_core::puzzle_input::PuzzleInput;

    #[test]
    fn display() {
        let input = PuzzleInput::new("../aoc-25/input.txt");
        let floor = OceanFloor::from(input.to_lines());
        println!("{}", floor);
    }
}
