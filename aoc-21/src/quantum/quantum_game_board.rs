use crate::pawn::Pawn;
use crate::quantum::quantum_game_state::QuantumGameState;
use std::str::Lines;

pub struct QuantumGameBoard {
    base: QuantumGameState,
    winners: (u64, u64),
}

impl QuantumGameBoard {
    pub fn play(&mut self) {
        self.play_state(self.base)
    }

    pub fn count_winners(&self) -> (u64, u64) {
        self.winners
    }

    fn play_state(&mut self, state: QuantumGameState) {
        for next in state.step() {
            if let Some(winner) = next.get_winner() {
                if winner == true {
                    self.winners.1 += next.get_weight();
                } else {
                    self.winners.0 += next.get_weight();
                }
            } else {
                self.play_state(next)
            }
        }
    }
}

impl From<Lines<'_>> for QuantumGameBoard {
    fn from(mut input: Lines<'_>) -> Self {
        let p1 = Pawn::from(input.next().unwrap());
        let p2 = Pawn::from(input.next().unwrap());
        let first_state = QuantumGameState::new(p1, p2);
        QuantumGameBoard {
            base: first_state,
            winners: (0, 0),
        }
    }
}
