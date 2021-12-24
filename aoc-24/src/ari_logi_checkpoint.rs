use crate::ari_logi_uni_state::AriLogiUniState;
use crate::instruction_set::InstructionSet;
use crate::instruction_state::InstructionState;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AriLogiCheckpoint {
    state: AriLogiUniState,
    values: [InstructionState; 9],
    link: usize,
}

impl AriLogiCheckpoint {
    pub fn get<I: Iterator<Item = i64>>(
        &mut self,
        mut input: I,
        instructions: &[InstructionSet; 14],
    ) -> Option<AriLogiUniState> {
        if self.link == 14 {
            panic!("Shouldn't get here!")
        }
        let next_input = input.next()? as usize;
        if next_input == 0 {
            return None;
        }
        if self.values[next_input - 1] == InstructionState::New {
            if let Ok(next_check) = self.get_next(next_input, instructions) {
                if next_check.link == 14 {
                    self.values[next_input - 1] = InstructionState::Complete;
                    return Some(next_check.state);
                }
                self.values[next_input - 1] = InstructionState::InProgress(Box::new(next_check));
            } else {
                self.values[next_input - 1] = InstructionState::Complete;
            }
        }
        let result = match &mut self.values[next_input - 1] {
            InstructionState::Complete => None,
            InstructionState::New => panic!("This shouldn't happen!"),
            InstructionState::InProgress(state) => state.get(input, instructions),
        };
        if let InstructionState::InProgress(s) = &self.values[next_input - 1] {
            if s.is_complete() {
                if self.link <= 3 {
                    println!(
                        "Closing branch {} for input {}",
                        self.state.input, next_input
                    );
                }
                self.values[next_input - 1] = InstructionState::Complete;
            }
        }

        result
    }

    pub fn is_complete(&self) -> bool {
        if self.link == 14 {
            return true;
        }
        self.values.iter().all(|v| match v {
            InstructionState::Complete => true,
            _ => false,
        })
    }

    fn get_next(&self, input: usize, instructions: &[InstructionSet; 14]) -> Result<Self, ()> {
        let next_set = &instructions[self.link];
        let next_state = next_set.run(&self.state, input as i64)?;
        let mut next = AriLogiCheckpoint::default();
        next.link = self.link + 1;
        next.state = next_state;
        Ok(next)
    }
}

impl Default for AriLogiCheckpoint {
    fn default() -> Self {
        AriLogiCheckpoint {
            state: AriLogiUniState::default(),
            values: [
                InstructionState::default(),
                InstructionState::default(),
                InstructionState::default(),
                InstructionState::default(),
                InstructionState::default(),
                InstructionState::default(),
                InstructionState::default(),
                InstructionState::default(),
                InstructionState::default(),
            ],
            link: 0,
        }
    }
}
