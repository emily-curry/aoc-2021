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
