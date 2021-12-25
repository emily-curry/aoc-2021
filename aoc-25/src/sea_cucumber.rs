use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SeaCucumber {
    Right,
    Down,
    None,
}

impl From<char> for SeaCucumber {
    fn from(input: char) -> Self {
        match input {
            '>' => SeaCucumber::Right,
            'v' => SeaCucumber::Down,
            '.' => SeaCucumber::None,
            _ => panic!("Invalid input: {}", input),
        }
    }
}

impl Default for SeaCucumber {
    fn default() -> Self {
        SeaCucumber::None
    }
}

impl Display for SeaCucumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SeaCucumber::Right => ">",
            SeaCucumber::Down => "v",
            SeaCucumber::None => ".",
        };
        f.write_str(s)
    }
}
