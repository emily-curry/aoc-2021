#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Register {
    Value(i64),
    W,
    X,
    Y,
    Z,
}

impl From<&str> for Register {
    fn from(input: &str) -> Self {
        match input {
            "w" => Register::W,
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            _ => Register::Value(input.parse().unwrap()),
        }
    }
}
