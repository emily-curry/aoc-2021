use crate::classical::deterministic_die::DeterministicDie;
use crate::pawn::Pawn;
use std::str::Lines;

pub struct ClassicalGameBoard {
    p1: Pawn,
    p2: Pawn,
    turn: u32,
    die: DeterministicDie,
}

impl ClassicalGameBoard {
    pub fn play(&mut self) {
        loop {
            let next_roll = self.roll_thrice();
            let pawn = match self.turn % 2 {
                0 => &mut self.p1,
                1 => &mut self.p2,
                _ => panic!("Impossible!"),
            };
            self.turn += 1;
            pawn.advance(next_roll);
            if pawn.get_score() >= 1000 {
                break;
            }
        }
    }

    pub fn get_p1(&self) -> &Pawn {
        &self.p1
    }

    pub fn get_p2(&self) -> &Pawn {
        &self.p2
    }

    pub fn get_die(&self) -> &DeterministicDie {
        &self.die
    }

    fn roll_thrice(&mut self) -> u16 {
        self.die.roll() + self.die.roll() + self.die.roll()
    }
}

impl From<Lines<'_>> for ClassicalGameBoard {
    fn from(mut input: Lines<'_>) -> Self {
        let p1 = Pawn::from(input.next().unwrap());
        let p2 = Pawn::from(input.next().unwrap());
        ClassicalGameBoard {
            p1,
            p2,
            turn: 0,
            die: DeterministicDie::new(),
        }
    }
}
