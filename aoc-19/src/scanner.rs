use crate::beacon::Beacon;
use crate::rotation::{Rotation, ROTATIONS};
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::str::Lines;

#[derive(Clone)]
pub struct Scanner {
    beacons: HashSet<Beacon>,
}

impl Scanner {
    pub fn new(beacons: HashSet<Beacon>) -> Self {
        Scanner { beacons }
    }

    pub fn extend(&mut self, other: Scanner) {
        for beacon in other.beacons {
            self.beacons.insert(beacon);
        }
    }

    pub fn count(&self) -> usize {
        self.beacons.len()
    }

    /// Detects if any rotation + translation of the other scanner can produce at least 12 matching beacon locations.
    /// If true, returns the scanner with the rotation + translation applied. Otherwise returns none.
    pub fn is_match(&self, other: &Scanner) -> Option<(Scanner, (i32, i32, i32))> {
        for rotation in &ROTATIONS {
            let other_rotated = other.rotate(rotation);
            let possible_translations = self.find_translation_vectors(&other_rotated);
            for translation in possible_translations {
                let other_translated = other_rotated.translate(&translation);
                let mut match_count = 0u8;
                for other_transformed_beacon in &other_translated.beacons {
                    if self.beacons.contains(other_transformed_beacon) {
                        match_count += 1;
                    }
                }
                if match_count >= 12 {
                    let scanner_location =
                        (translation.0 * -1, translation.1 * -1, translation.2 * -1);
                    return Some((other_translated, scanner_location));
                }
            }
        }
        None
    }

    fn rotate(&self, rotation: &Rotation) -> Self {
        let mut next: HashSet<Beacon> = HashSet::new();
        for beacon in &self.beacons {
            next.insert(beacon.rotate(rotation));
        }
        Scanner::new(next)
    }

    fn translate(&self, translation: &(i32, i32, i32)) -> Self {
        let mut next: HashSet<Beacon> = HashSet::new();
        for beacon in &self.beacons {
            next.insert(beacon.translate(translation));
        }
        Scanner::new(next)
    }

    // Finds the set of translation vectors that can be applied to other to make at least 1 point in other match that of self.
    fn find_translation_vectors(&self, other: &Scanner) -> HashSet<(i32, i32, i32)> {
        let mut vectors = HashSet::new();
        for dest in &self.beacons {
            for origin in &other.beacons {
                let vector = (
                    dest.a() - origin.a(),
                    dest.b() - origin.b(),
                    dest.c() - origin.c(),
                );
                vectors.insert(vector);
            }
        }

        vectors
    }
}

impl From<&mut Lines<'_>> for Scanner {
    fn from(input: &mut Lines<'_>) -> Self {
        let mut beacons: HashSet<Beacon> = HashSet::new();
        while let Some(line) = input.next() {
            if line == "" {
                break;
            }
            let beacon = Beacon::from(line);
            beacons.insert(beacon);
        }
        Scanner::new(beacons)
    }
}

impl Debug for Scanner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for beacon in &self.beacons {
            f.write_fmt(format_args!("{:?}\n", beacon))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;
    use aoc_core::puzzle_input::PuzzleInput;

    // I wrote this simple test based on the example given before even creating the ROTATIONS array or having the idea for the set of translation vectors.
    // This helped me a ton, to be able to look at the current output of the scanners at each intermediate step and ask "okay, what now?".
    #[test]
    fn is_match() {
        let input = PuzzleInput::new("../aoc-19/example.txt");
        let mut lines = input.to_lines();
        lines.next();
        let authority = Scanner::from(&mut lines);
        lines.next();
        let first = Scanner::from(&mut lines);
        assert_eq!(authority.count(), 25);
        assert_eq!(first.count(), 25);
        let first_transformed = authority.is_match(&first).unwrap();
        let matching_points = authority.beacons.iter().fold(0, |acc, val| {
            if first_transformed.0.beacons.contains(val) {
                acc + 1
            } else {
                acc
            }
        });
        assert!(matching_points >= 12);
    }
}
