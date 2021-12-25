use crate::instruction::Instruction;
use crate::register::Register;

#[derive(Debug, Copy, Clone)]
pub struct InstructionSet {
    // div: i64,
    pub add_1: i8,
    pub add_2: i8,
    pub idx: usize,
}

impl InstructionSet {
    pub fn new(instructions: [Instruction; 18], idx: usize) -> Self {
        let add_1 = match instructions[5] {
            Instruction::Add((_, r2)) => match r2 {
                Register::Value(v) => v,
                _ => panic!("Unexpected input!"),
            },
            _ => panic!("Unexpected input!"),
        } as i8;
        let add_2 = match instructions[15] {
            Instruction::Add((_, r2)) => match r2 {
                Register::Value(v) => v,
                _ => panic!("Unexpected input!"),
            },
            _ => panic!("Unexpected input!"),
        } as i8;
        InstructionSet { add_1, add_2, idx }
    }

    // pub fn run(&self, current_z: i64, input: i64) -> i64 {
    // Expanded form:
    // let mut z = current_z;
    // let mut x = z % 26;
    // z /= self.div;
    // x += self.add_1;
    // if x == input {
    //     x = 0;
    // } else {
    //     x = 1;
    // }
    // let mut y = 25 * x;
    // y += 1; // y is either 26 or 1
    // z *= y; // either z*26 or no-op
    // y = input;
    // y += self.add_2;
    // y *= x; // only non-zero if z*26 was executed above
    // z += y; // only ever executed if z*26 above also was, otherwise y=0

    // Intermediate form:
    // let z = current_z / self.div;
    // if input != (current_z % 26) + self.add_1 {
    //     (26 * z) + input + self.add_2
    // } else {
    //     z
    // }

    // Intermediate form 2:
    // if self.div == 1 {
    //     // add_1 is always positive when div == 1.
    //     // add_1 is never single-digit (when positive), so the if in the above form is always true when add_1 > 0, and we can skip that calculation
    //     (current_z * 26) + input + self.add_2
    // } else {
    //     // div is always 26 if not 1.
    //     if input == (current_z % 26) + self.add_1 {
    //         current_z / 26
    //     } else {
    //         current_z + input + self.add_2 // this is (current_z * 26 / 26) + input + self.add_2
    //     }
    // }
    // ^ This form is getting at something fundamental. All operations against z are in terms of 26, which is almost certainly exploitable...
    // Additionally, we are solving for z=0. add_1 is positive and negative an equal number of times, so each branch of the if will be hit 7 times.
    // Therefore, we need to construct a set of input numbers that cancel each other out.

    // Values of add_1 are: 011, 014, 013, -04, 011, 010, -04, -12, 010, -11, 012, -01, -00, -11
    // Values of add_2 are: 003, 007, 001, 006, 014, 007, 009, 009, 006, 004, 000, 007, 012, 001
    // The middle branch (current_z / 26) is the only operation that "cancels", so we _have_ to hit it each time add_1 is negative.
    // And because current_z is always some multiple of 26 plus some offset, we should be able to say that the only valid input is the previous input plus the previous add_2 plus the current add_1.
    // For the 4th step, input[4] = input[3] + 1 - 4 = input[3] - 3
    // And that means!!! oh my god. this doesn't need a computer at all. Since these pairs of operations need to cancel,
    // we can just pair them all off, they "disappear" once they're paired. We want the maximum value for each pair, and since 9 is the upper limit for any input, that's easy to find (input[4] = 6 since input[3] = 9 is the highest value we can use).
    // The pairing (using vec.windows(2)) is done in ari_logi_uni.rs, and the computation of values is done in instruction_set_pair.rs.
    // }
}
