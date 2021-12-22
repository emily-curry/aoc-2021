use crate::pawn::Pawn;

#[derive(Copy, Clone, Debug)]
pub struct QuantumGameState {
    p1: Pawn,
    p2: Pawn,
    turn: bool,
    /// The number of times this state occurred
    weight: u64,
}

impl QuantumGameState {
    pub fn new(p1: Pawn, p2: Pawn) -> Self {
        QuantumGameState {
            p1,
            p2,
            turn: true,
            weight: 1,
        }
    }

    pub fn step(mut self) -> impl Iterator<Item = QuantumGameState> {
        self.turn = !self.turn; // Doing this here instead of 7 times in clone_advance saves 0.1 seconds, neato
        QuantumGameStateIter::new(self)
    }

    fn clone_advance(&self, count: u16, weight: u64) -> Self {
        let mut next = self.clone();
        next.weight *= weight;
        match next.turn {
            false => next.p1.advance(count),
            true => next.p2.advance(count),
        };
        next
    }

    pub fn get_winner(&self) -> Option<bool> {
        if self.p1.get_score() >= 21 {
            Some(false)
        } else if self.p2.get_score() >= 21 {
            Some(true)
        } else {
            None
        }
    }

    pub fn get_weight(&self) -> u64 {
        self.weight
    }
}

/// All possible rolls and their associated occurrences per step.
const ROLLS: [(u16, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

/// An iterator that yields each possible next state from a given base state.
/// Does not compute each next state until requested, which actually saves about 5.75s during execution.
struct QuantumGameStateIter {
    index: usize,
    base: QuantumGameState,
}

impl QuantumGameStateIter {
    fn new(base: QuantumGameState) -> Self {
        QuantumGameStateIter { index: 0, base }
    }
}

impl Iterator for QuantumGameStateIter {
    type Item = QuantumGameState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 7 {
            None
        } else {
            let roll = ROLLS[self.index];
            let result = self.base.clone_advance(roll.0, roll.1);
            self.index += 1;
            Some(result)
        }
    }
}
