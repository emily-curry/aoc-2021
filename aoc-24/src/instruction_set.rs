use crate::ari_logi_uni_state::AriLogiUniState;
use crate::instruction::Instruction;

#[derive(Debug, Copy, Clone)]
pub struct InstructionSet {
    instructions: [Instruction; 18],
}

impl InstructionSet {
    pub fn new(instructions: [Instruction; 18]) -> Self {
        debug_assert!({
            if let Instruction::Inp(_) = instructions[0] {
                true
            } else {
                false
            }
        });
        debug_assert!({
            let mut success = true;
            for i in 1..18usize {
                if let Instruction::Inp(_) = instructions[i] {
                    success = false;
                }
            }
            success
        });
        InstructionSet { instructions }
    }

    pub fn run(&self, state: &AriLogiUniState, input: i64) -> Result<AriLogiUniState, ()> {
        let mut next = state.clone();
        for inst in &self.instructions {
            self.run_instruction(inst, &mut next, input)?;
        }
        Ok(next)
    }

    fn run_instruction(
        &self,
        inst: &Instruction,
        state: &mut AriLogiUniState,
        input: i64,
    ) -> Result<(), ()> {
        match inst {
            Instruction::Inp(r1) => {
                if input == 0 {
                    panic!("Invalid input!");
                }
                state.record_input(input);
                state.write(&r1, input);
            }
            Instruction::Add((r1, r2)) => {
                let result = state.read(&r1) + state.read(&r2);
                state.write(&r1, result);
            }
            Instruction::Mul((r1, r2)) => {
                let result = state.read(&r1) * state.read(&r2);
                state.write(&r1, result);
            }
            Instruction::Div((r1, r2)) => {
                let v2 = state.read(&r2);
                if v2 == 0 {
                    return Err(()); // Invalid operation, fail fast.
                }
                let result = state.read(&r1) / v2;
                state.write(&r1, result);
            }
            Instruction::Mod((r1, r2)) => {
                let v1 = state.read(&r1);
                let v2 = state.read(&r2);
                if v1 < 0 || v2 <= 0 {
                    return Err(()); // Invalid operation, fail fast.
                }
                let result = v1 % v2;
                state.write(&r1, result);
            }
            Instruction::Eql((r1, r2)) => {
                let result = state.read(&r1) == state.read(&r2);
                let result_value = if result { 1 } else { 0 };
                state.write(&r1, result_value);
            }
        };
        Ok(())
    }
}
