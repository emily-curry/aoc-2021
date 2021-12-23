use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CuboidState {
    On,
    Off,
}

impl From<&str> for CuboidState {
    fn from(input: &str) -> Self {
        match input {
            "on" => CuboidState::On,
            "off" => CuboidState::Off,
            _ => panic!("Invalid input: {}", input),
        }
    }
}

impl Display for CuboidState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            CuboidState::On => "on",
            CuboidState::Off => "off",
        };
        f.write_str(result)
    }
}
