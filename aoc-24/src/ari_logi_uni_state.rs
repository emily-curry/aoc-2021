use crate::register::Register;
use std::fmt::{Display, Formatter};

/// TODO: Break State out into separate cached "chunks".
/// Basically, we can calculate the output state for the `inp` up until the next `inp`, and keep that computation around.
/// We can probably impl Ord for this struct (using only input) and then use a BinaryHeap to prio-queue the best branches
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct AriLogiUniState {
    pub w: i64,
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub input: i64,
}

impl AriLogiUniState {
    pub fn read(&self, reg: &Register) -> i64 {
        match reg {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
            Register::Value(v) => *v,
        }
    }

    pub fn write(&mut self, reg: &Register, value: i64) {
        match reg {
            Register::W => self.w = value,
            Register::X => self.x = value,
            Register::Y => self.y = value,
            Register::Z => self.z = value,
            Register::Value(_) => panic!("Can't do that!"),
        };
    }

    pub fn record_input(&mut self, value: i64) {
        self.input *= 10;
        self.input += value;
    }
}

impl Default for AriLogiUniState {
    fn default() -> Self {
        AriLogiUniState {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
            input: 0,
        }
    }
}

impl Display for AriLogiUniState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "w: {:>5} | x: {:>5} | y: {:>5} | z: {:>5} | ",
            format!("{:+}", self.w),
            format!("{:+}", self.x),
            format!("{:+}", self.y),
            format!("{:+}", self.z)
        ))?;
        f.write_fmt(format_args!("{}", self.input))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ari_logi_uni_state::AriLogiUniState;

    #[test]
    fn display() {
        let test = AriLogiUniState {
            x: 100,
            y: -32,
            z: 854,
            w: -832,
            input: 12345678912345,
        };
        println!("{}", test);
    }
}
