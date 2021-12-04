use std::fs::read_to_string;
use std::path::Path;
use std::str::Lines;

#[derive(Debug)]
pub struct PuzzleInput {
    raw: String,
}

impl PuzzleInput {
    /// Reads the file provided in `path` and returns something usable by the puzzles.
    /// This panics immediately if the file cannot be read.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let raw_result = read_to_string(path);
        PuzzleInput {
            raw: raw_result.unwrap(),
        }
    }

    pub fn as_string(&self) -> &String {
        &self.raw
    }

    pub fn to_lines(&self) -> Lines {
        self.raw.lines()
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle_input::PuzzleInput;
    use std::ops::Index;

    #[test]
    fn can_construct_without_panic() {
        let input = PuzzleInput::new("../aoc-03/input.txt");
        assert!(!input.raw.is_empty());
    }

    #[test]
    fn as_string() {
        let input = PuzzleInput::new("../aoc-03/input.txt");
        assert!(!input.as_string().is_empty());
    }

    #[test]
    fn to_lines() {
        let input = PuzzleInput::new("../aoc-03/input.txt");
        let lines = input.to_lines().collect::<Vec<&str>>();
        assert_eq!(input.to_lines().collect::<Vec<&str>>().len(), 1000);
        assert_eq!(*lines.index(3), "000101111101")
    }
}
