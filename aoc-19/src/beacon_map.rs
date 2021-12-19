use crate::scanner::Scanner;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::str::Lines;

pub struct BeaconMap {
    /// Our source of truth scanner. We will attempt to transform each unassociated scanner
    /// to match the points in this scanner. If a match is found, we'll add the transformed points to the authority.
    authority: Scanner,
    /// A list of scanners that have not been associated with the authority scanner. After the program completes, this should be empty.
    unassociated: VecDeque<Scanner>,
    /// A set of all known scanner positions.
    scanner_positions: HashSet<(i32, i32, i32)>,
}

impl BeaconMap {
    pub fn new(mut scanners: VecDeque<Scanner>) -> Self {
        let authority = scanners.pop_front().expect("Length 0 vec provided!");
        let scanner_positions = HashSet::from([(0, 0, 0)]);
        BeaconMap {
            authority,
            unassociated: scanners,
            scanner_positions,
        }
    }

    pub fn associate_all(&mut self) {
        let mut retry = self.unassociated.len();
        while self.unassociated.len() > 0 {
            if retry == 0 {
                // I added this because I thought this may loop forever. Just turned out that the program wasn't very efficient.
                panic!("Went through the whole queue and never found a match!");
            }
            let next = self.unassociated.pop_front().unwrap();
            if let Some(transformed) = self.authority.is_match(&next) {
                // If a match, add all points in the transformed scanner to the authority.
                // This will allow us to match more unassociated scanners.
                self.authority.extend(transformed.0);
                self.scanner_positions.insert(transformed.1);
                // Reset our retry count.
                retry = self.unassociated.len();
            } else {
                // If no match, push it back into the queue to try later.
                self.unassociated.push_back(next);
                // Decrement our retry count.
                retry -= 1;
            }
        }
    }

    /// Returns the number of beacons in the authority scanner.
    pub fn count(&self) -> usize {
        self.authority.count()
    }

    pub fn find_max_distance(&self) -> i32 {
        let mut distances: HashSet<i32> = HashSet::new();
        for (l_a, l_b, l_c) in self.scanner_positions.iter() {
            for (r_a, r_b, r_c) in self.scanner_positions.iter() {
                let dist = (l_a - r_a).abs() + (l_b - r_b).abs() + (l_c - r_c).abs();
                distances.insert(dist);
            }
        }
        *distances.iter().max().unwrap()
    }
}

impl From<Lines<'_>> for BeaconMap {
    fn from(mut input: Lines) -> Self {
        let mut scanners: VecDeque<Scanner> = VecDeque::new();
        loop {
            if input.next().is_none() {
                break;
            }
            // Pass Scanner::from a mutable reference, so it can consumer the iterator until it hits a terminator.
            // This loop will pick up where it leaves off.
            let scanner = Scanner::from(&mut input);
            scanners.push_back(scanner);
        }

        BeaconMap::new(scanners)
    }
}

impl Debug for BeaconMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "----- Authority -----\n{:?}\n",
            self.authority
        ))?;
        for (index, rest) in self.unassociated.iter().enumerate() {
            f.write_fmt(format_args!(
                "-----Scanner {:0>3}-----\n{:?}\n",
                index, rest
            ))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::beacon_map::BeaconMap;
    use aoc_core::puzzle_input::PuzzleInput;

    #[test]
    fn parse_example() {
        let input = PuzzleInput::new("../aoc-19/example.txt");
        let map = BeaconMap::from(input.to_lines());
        assert_eq!(map.unassociated.len(), 4);
        assert_eq!(map.authority.count(), 25);
        assert_eq!(map.unassociated[3].count(), 26);
    }

    #[test]
    fn parse_input() {
        let input = PuzzleInput::new("../aoc-19/input.txt");
        let map = BeaconMap::from(input.to_lines());
        assert_eq!(map.unassociated.len(), 37);
    }

    #[test]
    fn debug() {
        let input = PuzzleInput::new("../aoc-19/example.txt");
        let map = BeaconMap::from(input.to_lines());
        print!("{:?}", map);
    }
}
