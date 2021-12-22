#[derive(Copy, Clone, Debug)]
pub struct Pawn {
    position: u8,
    score: u16,
}

impl Pawn {
    pub fn advance(&mut self, steps: u16) {
        let mut next_position = (self.position as u16) + steps;
        while next_position > 10 {
            next_position -= 10;
        }
        self.score += next_position;
        self.position = next_position as u8;
    }

    pub fn get_score(&self) -> u16 {
        self.score
    }
}

impl From<&str> for Pawn {
    fn from(input: &str) -> Self {
        Pawn {
            position: input.parse().unwrap(),
            score: 0,
        }
    }
}
